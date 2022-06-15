pub mod data_source;

use self::data_source::DataSource;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "milston";
const APPLICATION: &str = "Milston";

const DATA_FILENAME: &str = "data.json";
const CONFIG_FILENAME: &str = "config.json";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub data_source: DataSource,
}

impl Config {
    pub fn get_data_source() -> DataSource {
        DataSource::File(Config::get_user_config_file_path())
    }

    fn get_user_data_file_path() -> PathBuf {
        let project_dirs = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION).unwrap();
        let data_dir_path = project_dirs.data_local_dir();
        data_dir_path.join(DATA_FILENAME)
    }

    fn get_user_config_file_path() -> PathBuf {
        let project_dirs = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION).unwrap();
        let data_dir_path = project_dirs.config_dir();
        data_dir_path.join(CONFIG_FILENAME)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            data_source: DataSource::File(Config::get_user_data_file_path()),
        }
    }
}
