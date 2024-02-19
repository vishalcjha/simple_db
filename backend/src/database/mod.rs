mod page;
mod table;
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use frontend::{
    command::statement::insert::Value, definitions::table_definition::TableName, InsertStatement,
    SelectStatement, TableDefinition,
};

use crate::{
    disk::{
        data_iterator::DiskDataIterator, def_iterator::DiskTableDefinitionIterator, DiskAccessor,
    },
    errors::{BEErrors, BEResult},
    DATABASE,
};

use self::table::Table;

type Sharable<T> = Arc<Mutex<T>>;
pub type Rows = Vec<Vec<Value>>;
pub(super) const PAGE_SIZE: usize = 4096;

#[derive(Debug, Clone, Default)]
pub(super) struct Database {
    table_definitions: Sharable<HashMap<TableName, &'static TableDefinition>>,
    tables: Sharable<HashMap<TableName, Table>>,
    disk_accessor: Sharable<Option<DiskAccessor>>,
}

impl Database {
    pub(super) fn init_db_with_file(&self, base_path: PathBuf) -> BEResult<()> {
        let mut disk_accessor = self.disk_accessor.lock().unwrap();
        *disk_accessor = Some(DiskAccessor::new(base_path));

        let def_iterator = DiskTableDefinitionIterator::new(disk_accessor.as_ref().unwrap());
        for table_def in def_iterator.into_iter() {
            DATABASE.add_table_definitions(table_def)?;
        }

        let data_iter = DiskDataIterator::new(disk_accessor.as_ref().unwrap());
        for (table_name, data) in data_iter.into_iter() {
            let table = Table::new(data);
            let mut tables = self.tables.lock().unwrap();
            tables.insert(table_name, table);
        }

        Ok(())
    }

    pub(super) fn flush_db(&self) -> BEResult<()> {
        {
            let tables = self.table_definitions.lock().unwrap();
            let disk_accessor = self.disk_accessor.lock().unwrap();
            for (name, def) in tables.iter() {
                disk_accessor
                    .as_ref()
                    .unwrap()
                    .write_table_definition(name, def)?;
            }
        }

        {
            let tables = self.tables.lock().unwrap();
            let disk_accessor = self.disk_accessor.lock().unwrap();

            for (name, data) in tables.iter() {
                for (index, page) in data.pages.iter().enumerate() {
                    disk_accessor.as_ref().unwrap().write_data_page(
                        name,
                        index,
                        page.get_data(),
                    )?;
                }
            }
        }

        Ok(())
    }

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
