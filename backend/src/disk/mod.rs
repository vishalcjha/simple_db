use std::{fs, path::PathBuf};

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
}
