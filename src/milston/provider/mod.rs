pub mod file;

#[cfg(feature = "http")]
pub mod http;

#[cfg(feature = "http")]
use self::http::HttpDataProvider;

use self::file::FileDataProvider;
use super::config::data_source::DataSource;
use crate::error::Result;
use serde::{de::DeserializeOwned, Serialize};

pub struct DataProvider;

impl DataProvider {
    pub async fn save<T>(data_source: &DataSource, data: &T) -> Result<()>
    where
        T: Serialize + Send + Sync,
    {
        match data_source {
            DataSource::File(file_path) => FileDataProvider::save_data(data, file_path).await,
            #[cfg(feature = "http")]
            DataSource::Http(url) => HttpDataProvider::save_data(&data, url).await,
        }
    }

    pub async fn load<T>(data_source: &DataSource) -> Result<T>
    where
        T: DeserializeOwned,
    {
        match data_source {
            DataSource::File(file_path) => FileDataProvider::load_data(file_path).await,
            #[cfg(feature = "http")]
            DataSource::Http(url) => HttpDataProvider::load_data(url).await,
        }
    }
}
