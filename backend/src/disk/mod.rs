use frontend::{definitions::table_definition::TableName, TableDefinition};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::PathBuf,
};

pub(crate) mod data_iterator;
pub(crate) mod def_iterator;
use crate::{database::PAGE_SIZE, errors::BEResult};

#[derive(Debug)]
pub(crate) struct DiskAccessor {
    base_path: String,
}

impl DiskAccessor {
    pub fn new(dir_path: PathBuf) -> DiskAccessor {
        if !dir_path.exists() {
            fs::create_dir_all(&dir_path)
                .expect(&format!("failed to create database dir {:?}", dir_path));

            // Create the subdirectories 'meta' and 'data'
            let meta_dir = dir_path.join("meta");
            let data_dir = dir_path.join("data");
            fs::create_dir(&meta_dir).expect(&format!(
                "failed to create database meta dir at {:?}",
                dir_path
            ));
            fs::create_dir(&data_dir).expect(&format!(
                "failed to create database data dir at {:?}",
                dir_path
            ));
        }

        DiskAccessor {
            base_path: dir_path.to_string_lossy().to_string(),
        }
    }

    fn read_file_as_bytes(&self, file_name: impl Into<String>) -> BEResult<Vec<u8>> {
        let mut path_buf = PathBuf::from(self.base_path.clone());
        path_buf.push(file_name.into());
        let mut file = File::open(path_buf)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    fn get_files_in_dir(&self, dir_name: impl Into<String>) -> BEResult<Vec<String>> {
        let mut files = Vec::new();
        let mut path_buf = PathBuf::from(self.base_path.clone());
        path_buf.push(dir_name.into());
        if let Ok(entries) = fs::read_dir(path_buf) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(filename) = entry.file_name().to_str() {
                        files.push(filename.to_string());
                    }
                }
            }
        }
        Ok(files)
    }

    pub fn write_table_definition(
        &self,
        name: &TableName,
        definition: &TableDefinition,
    ) -> BEResult<()> {
        let mut path_buf = PathBuf::from(self.base_path.clone());
        path_buf.push("meta");
        path_buf.push(name.0.clone());
        if path_buf.exists() {
            return Ok(());
        }

        let mut file = File::create(path_buf)?;
        let table_def_str = serde_json::to_string(definition)?;
        file.write_all(table_def_str.as_bytes())?;

        Ok(())
    }

    pub fn write_data_page(
        &self,
        name: &TableName,
        page_num: usize,
        data: *const u8,
    ) -> BEResult<()> {
        let mut path_buf = PathBuf::from(self.base_path.clone());
        path_buf.push("data");
        path_buf.push(name.0.clone());

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(path_buf)?;

        file.seek(SeekFrom::Start((page_num * PAGE_SIZE) as u64))?;

        unsafe { file.write_all(std::slice::from_raw_parts(data, PAGE_SIZE))? }

        Ok(())
    }
}
