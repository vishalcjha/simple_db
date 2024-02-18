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
            let values = page.read(0, column_names.clone(), table_definition)?;
            result.push(values);
        }
        Ok(result)
    }
}
