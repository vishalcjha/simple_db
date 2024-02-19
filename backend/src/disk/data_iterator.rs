use frontend::definitions::table_definition::TableName;

use super::DiskAccessor;

pub(crate) struct DiskDataIterator<'a> {
    disk_accessor: &'a DiskAccessor,
    unprocessed_tables: Vec<String>,
}

impl<'a> DiskDataIterator<'a> {
    pub fn new(disk_accessor: &'a DiskAccessor) -> Self {
        let unprocessed_tables = disk_accessor.get_files_in_dir("data").unwrap();

        DiskDataIterator {
            disk_accessor,
            unprocessed_tables,
        }
    }
}

impl<'a> Iterator for DiskDataIterator<'a> {
    type Item = (TableName, Vec<u8>);

    fn next(&mut self) -> Option<Self::Item> {
        self.unprocessed_tables.pop().map(|it| {
            let data = self
                .disk_accessor
                .read_file_as_bytes(format!("data/{}", it))
                .unwrap();
            (TableName(it), data)
        })
    }
}
