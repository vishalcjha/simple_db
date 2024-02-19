use frontend::TableDefinition;

use super::DiskAccessor;

pub(crate) struct DiskTableDefinitionIterator<'a> {
    disk_accessor: &'a DiskAccessor,
    unprocessed_tables: Vec<String>,
}

impl<'a> DiskTableDefinitionIterator<'a> {
    pub fn new(disk_accessor: &'a DiskAccessor) -> Self {
        let unprocessed_tables = disk_accessor.get_files_in_dir("meta").unwrap();

        DiskTableDefinitionIterator {
            disk_accessor,
            unprocessed_tables,
        }
    }
}

impl<'a> Iterator for DiskTableDefinitionIterator<'a> {
    type Item = TableDefinition;

    fn next(&mut self) -> Option<Self::Item> {
        self.unprocessed_tables.pop().map(|it| {
            let definition = self
                .disk_accessor
                .read_file_as_bytes(format!("meta/{}", it))
                .unwrap();
            serde_json::from_slice::<TableDefinition>(&definition).unwrap()
        })
    }
}
