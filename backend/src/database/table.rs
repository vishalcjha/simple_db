use frontend::{command::statement::insert::Value, ColumnName, TableDefinition};

use crate::errors::BEResult;

use super::page::Page;

#[derive(Debug, Clone, Default)]
pub(super) struct Table {
    pub pages: Vec<Page>,
}

impl Table {
    pub(super) fn read_pages(
        &self,
        column_names: Vec<ColumnName>,
        table_definition: &TableDefinition,
    ) -> BEResult<Vec<Vec<Value>>> {
        let mut result = Vec::new();
        for page in self.pages.iter() {
            if let Some(mut values) = page.read_all_rows(&column_names, table_definition)? {
                result.append(&mut values);
            }
        }
        Ok(result)
    }

    pub(super) fn write_row(
        &mut self,
        values: Vec<Value>,
        table_definition: &TableDefinition,
    ) -> BEResult<()> {
        if let Some(last_page) = self.pages.last_mut() {
            if let Some(_) = last_page.available_slot_pos() {
                last_page.write(values, &table_definition)?;
                return Ok(());
            }
        }

        let mut page = Page::default();
        page.write(values, &table_definition)?;
        self.pages.push(page);

        Ok(())
    }
}
