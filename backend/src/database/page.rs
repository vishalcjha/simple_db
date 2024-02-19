use std::alloc::{alloc, dealloc, Layout};

use frontend::{command::statement::insert::Value, ColumnName, TableDefinition};

use crate::errors::{BEErrors, BEResult};

use super::{Rows, PAGE_SIZE};

const SLOT_COUNT: usize = 20;
const SLOT_SIZE: usize = 32;

#[derive(Debug)]
pub struct Slot(u8);

/// Page a in-memory storage of rows in table.
/// To allow multiple rows to be present, it will use slot page design.
/// For simplicity, header of page consists of 20 slots. i.e. a page can have max of 20 rows in it.
/// First entry in page is slot count which is free to use.
#[derive(Debug, Clone)]
pub(super) struct Page {
    page: *mut u8,
    free_offset: usize,
}

unsafe impl Send for Page {}
unsafe impl Sync for Page {}

impl Default for Page {
    fn default() -> Self {
        let page = unsafe { alloc(Layout::from_size_align_unchecked(PAGE_SIZE, 1)) as *mut u8 };
        let free_slot = 0_u32;
        unsafe {
            std::ptr::copy_nonoverlapping(
                &free_slot as *const u32 as *const u8,
                page,
                std::mem::size_of::<u32>(),
            )
        };
        Self {
            page,
            // free slot num + 20 slots space are available for table data.
            free_offset: (SLOT_COUNT * SLOT_SIZE) + std::mem::size_of::<u32>(),
        }
    }
}

impl Drop for Page {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.page, Layout::from_size_align_unchecked(PAGE_SIZE, 1));
        }
    }
}

impl Page {
    fn get_alignment_padding<T>(offset: usize) -> usize {
        let required_alignment = std::mem::align_of::<T>();
        let misalignment = offset % required_alignment;
        if misalignment > 0 {
            required_alignment - misalignment
        } else {
            0
        }
    }

    pub fn available_slot_pos(&self) -> Option<u32> {
        let free_slot_pos = unsafe { *(self.page as *const u32) };
        if free_slot_pos <= 20 {
            Some(free_slot_pos)
        } else {
            None
        }
    }

    unsafe fn increment_free_slot_pos(&self, new_value: u32) {
        std::ptr::copy_nonoverlapping(
            &new_value as *const u32 as *const u8,
            self.page,
            std::mem::size_of::<u32>(),
        );
    }

    pub fn get_data(&self) -> *const u8 {
        self.page
    }

    pub fn write(
        &mut self,
        values: Vec<Value>,
        table_definition: &TableDefinition,
    ) -> BEResult<()> {
        let Some(available_slot_pos) = self.available_slot_pos() else {
            return Err(BEErrors::InternalError(String::from(
                "No free space in page",
            )));
        };

        unsafe {
            let ptr = self.page.add(
                (1 * std::mem::size_of::<u32>()
                    + available_slot_pos as usize * std::mem::size_of::<u32>())
                    as usize,
            );
            std::ptr::copy_nonoverlapping(
                &self.free_offset as *const usize as *const u8,
                ptr,
                std::mem::size_of::<u32>(),
            );
            self.increment_free_slot_pos(available_slot_pos + 1);
        }

        for value in values {
            match value {
                Value::NamedValue(name, value) => {
                    let Some(column) = table_definition.columns.iter().find(|it| it.0 == name)
                    else {
                        return Err(BEErrors::MissingColumn(format!("Column {name} not found")));
                    };
                    match column.1 {
                        frontend::ColumnType::Int => {
                            let Ok(value) = value.parse::<i64>() else {
                                return Err(BEErrors::MismatchedDataType(
                                    format!("{name}"),
                                    "Int",
                                    format!("{value}"),
                                ));
                            };

                            unsafe {
                                self.free_offset +=
                                    Page::get_alignment_padding::<i64>(self.free_offset);
                                let ptr = self.page.add(self.free_offset);
                                std::ptr::copy_nonoverlapping(
                                    &value as *const i64 as *const u8,
                                    ptr,
                                    std::mem::size_of::<i64>(),
                                );
                            }
                            self.free_offset += std::mem::size_of::<i64>();
                        }
                        frontend::ColumnType::Text => unsafe {
                            //write the length of string
                            self.free_offset +=
                                Page::get_alignment_padding::<usize>(self.free_offset);
                            let ptr = self.page.add(self.free_offset);
                            let str_len = value.len();
                            std::ptr::copy_nonoverlapping(
                                &str_len as *const usize as *const u8,
                                ptr,
                                std::mem::size_of::<usize>(),
                            );
                            self.free_offset += std::mem::size_of::<usize>();

                            //write actual string
                            let ptr = self.page.add(self.free_offset);
                            std::ptr::copy_nonoverlapping(value.as_ptr(), ptr, value.len());
                            self.free_offset += value.len();
                        },
                    };
                }
                Value::UnnamedValue(_) => todo!("not yet implemented"),
            }
        }

        Ok(())
    }

    pub fn read_all_rows(
        &self,
        columns: &[ColumnName],
        table_definition: &TableDefinition,
    ) -> BEResult<Option<Rows>> {
        let written_slots_count = unsafe { *(self.page as *const u32) };
        if written_slots_count <= 0 {
            return Ok(None);
        }
        let mut rows = Vec::new();
        let mut offset = std::mem::size_of::<u32>();
        for _ in 0..written_slots_count {
            let data_offset = unsafe { *(self.page.add(offset) as *const u32) } as usize;
            let row = self.read(data_offset, columns, table_definition)?;
            offset += std::mem::size_of::<u32>();
            rows.push(row);
        }

        Ok(Some(rows))
    }

    pub fn read(
        &self,
        offset: usize,
        columns: &[ColumnName],
        table_definition: &TableDefinition,
    ) -> BEResult<Vec<Value>> {
        let mut offset = offset;
        let mut result = Vec::with_capacity(columns.len());
        for ColumnName(name) in columns {
            let Some(column) = table_definition.columns.iter().find(|it| *it.0 == *name) else {
                return Err(BEErrors::MissingColumn(format!("Column {name} not found")));
            };

            match column.1 {
                frontend::ColumnType::Int => unsafe {
                    offset += Page::get_alignment_padding::<i64>(offset);
                    let ptr = self.page.add(offset);
                    let value = *(ptr as *const i64);
                    result.push(Value::NamedValue(name.to_string(), value.to_string()));
                    offset += std::mem::size_of::<i64>();
                },
                frontend::ColumnType::Text => unsafe {
                    offset += Page::get_alignment_padding::<usize>(offset);
                    let ptr = self.page.add(offset);
                    let str_len = *(ptr as *const usize);
                    offset += std::mem::size_of::<usize>();

                    let ptr = self.page.add(offset);
                    let byte_slice = std::slice::from_raw_parts(ptr, str_len);
                    let string_value = std::str::from_utf8_unchecked(byte_slice);
                    offset += str_len;
                    result.push(Value::NamedValue(
                        name.to_string(),
                        string_value.to_string(),
                    ));
                },
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::fixtures::student_table_fixture;
    use rstest::rstest;

    #[test]
    fn test_write() -> BEResult<()> {
        Ok(())
    }

    #[rstest]
    fn test_read_after_write(student_table_fixture: TableDefinition) -> BEResult<()> {
        let mut page = Page::default();
        page.write(
            vec![
                Value::NamedValue(String::from("name"), String::from("student1")),
                Value::NamedValue(String::from("age"), String::from("22")),
            ],
            &student_table_fixture,
        )?;

        let columns = page.read(
            0,
            &vec![
                ColumnName(String::from("name")),
                ColumnName(String::from("age")),
            ],
            &student_table_fixture,
        )?;
        println!("{columns:?}");
        Ok(())
    }
}
