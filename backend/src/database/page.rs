use std::{
    alloc::{alloc, dealloc, Layout},
    mem::size_of,
};

use frontend::{command::statement::insert::Value, ColumnName, TableDefinition};

use crate::errors::{BEErrors, BEResult};
const PAGE_SIZE: usize = 4096;
const BYTE_COUNT: usize = PAGE_SIZE / size_of::<u8>();
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
        Self {
            page,
            free_offset: Default::default(),
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
    pub fn write(
        &mut self,
        values: Vec<Value>,
        table_definition: &TableDefinition,
    ) -> BEResult<()> {
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

    pub fn read(
        &self,
        offset: usize,
        columns: Vec<ColumnName>,
        table_definition: &TableDefinition,
    ) -> BEResult<Vec<Value>> {
        let mut offset = offset;
        let mut result = Vec::with_capacity(columns.len());
        for ColumnName(name) in columns {
            let Some(column) = table_definition.columns.iter().find(|it| it.0 == name) else {
                return Err(BEErrors::MissingColumn(format!("Column {name} not found")));
            };

            match column.1 {
                frontend::ColumnType::Int => unsafe {
                    let ptr = self.page.add(offset);
                    let value = *(ptr as *const i64);
                    result.push(Value::NamedValue(name, value.to_string()));
                    offset += std::mem::size_of::<i64>();
                },
                frontend::ColumnType::Text => unsafe {
                    let ptr = self.page.add(offset);
                    let str_len = *(ptr as *const usize);
                    offset += std::mem::size_of::<usize>();

                    let ptr = self.page.add(offset);
                    let byte_slice = std::slice::from_raw_parts(ptr, str_len);
                    let string_value = std::str::from_utf8_unchecked(byte_slice);
                    offset += str_len;
                    result.push(Value::NamedValue(name, string_value.to_string()));
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
            vec![
                ColumnName(String::from("name")),
                ColumnName(String::from("age")),
            ],
            &student_table_fixture,
        )?;
        println!("{columns:?}");
        Ok(())
    }
}
