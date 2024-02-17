use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use frontend::{definitions::table_definition::TableName, TableDefinition};

use crate::errors::{BEErrors, BEResult};

type Sharable<T> = Arc<Mutex<T>>;

#[derive(Debug, Clone, Default)]
pub(crate) struct Database {
    table_definitions: Sharable<HashMap<TableName, TableDefinition>>,
}

impl Database {
    pub(crate) fn add_table_definitions(&self, definition: TableDefinition) -> BEResult<()> {
        let mut definition_holder = self.table_definitions.lock().unwrap();
        if definition_holder.get(&definition.name).is_some() {
            return Err(BEErrors::DuplicateDefinition(definition.name));
        }

        definition_holder.insert(definition.name.clone(), definition);
        Ok(())
    }
}
