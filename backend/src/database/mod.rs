mod page;
mod table;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use frontend::{
    command::statement::insert::Value, definitions::table_definition::TableName, InsertStatement,
    SelectStatement, TableDefinition,
};

use crate::errors::{BEErrors, BEResult};

use self::{page::Page, table::Table};

type Sharable<T> = Arc<Mutex<T>>;
pub type Rows = Vec<Vec<Value>>;

#[derive(Debug, Clone, Default)]
pub(super) struct Database {
    table_definitions: Sharable<HashMap<TableName, &'static TableDefinition>>,
    tables: Sharable<HashMap<TableName, Table>>,
}

impl Database {
    pub(super) fn add_table_definitions(&self, definition: TableDefinition) -> BEResult<()> {
        let mut definition_holder = self.table_definitions.lock().unwrap();
        if definition_holder.get(&definition.name).is_some() {
            return Err(BEErrors::DuplicateDefinition(definition.name));
        }

        let table_name = definition.name.clone();
        let definition = Box::new(definition);
        let definition: &'static TableDefinition = Box::leak(definition);
        definition_holder.insert(table_name.clone(), definition);

        let mut tables = self.tables.lock().unwrap();
        tables.insert(table_name, Table::default());
        Ok(())
    }

    pub(super) fn insert_record(&self, insert_statement: InsertStatement) -> BEResult<()> {
        let table_definition = self.get_table_definition(&insert_statement.0)?;

        let mut tables = self.tables.lock().unwrap();
        let table = tables.get_mut(&insert_statement.0).unwrap();
        table.write_row(insert_statement.1, table_definition)
    }

    pub(super) fn select_records(&self, select_statement: SelectStatement) -> BEResult<Rows> {
        let table_definition = self.get_table_definition(&select_statement.0)?;
        let tables = self.tables.lock().unwrap();
        let table = tables.get(&select_statement.0).unwrap();

        table.read_pages(select_statement.1, table_definition)
    }

    fn get_table_definition(&self, table_name: &TableName) -> BEResult<&'static TableDefinition> {
        let definition_holder = self.table_definitions.lock().unwrap();
        let Some(&table_definition) = definition_holder.get(&table_name) else {
            return Err(BEErrors::MissingTable(table_name.0.clone()));
        };

        Ok(table_definition)
    }
}
