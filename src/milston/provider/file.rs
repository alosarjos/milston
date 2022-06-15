use crate::error::Result;
use serde::{de::DeserializeOwned, Serialize};
use std::{fs, fs::File, path::PathBuf};

pub struct FileDataProvider;

impl FileDataProvider {
    pub async fn save_data<T>(data: &T, file_path: &PathBuf) -> Result<()>
    where
        T: Serialize + Send + Sync,
    {
        fs::create_dir_all(file_path.parent().unwrap())?;
        Ok(serde_json::to_writer(File::create(file_path)?, data)?)
    }

    pub async fn load_data<T: DeserializeOwned>(file_path: &PathBuf) -> Result<T> {
        Ok(serde_json::from_reader(File::open(file_path)?)?)
    }
}
