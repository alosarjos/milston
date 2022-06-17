//! Here all the required structures for `Config` handling are available.
//! Completely separated from structures required to manipulate the `Milston`
//! data.
//!
//! You can get access to all config structs like this:
//!
//! ```
//! use milston::config::*;
//! ```

mod data_source;

pub use data_source::DataSource;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "milston";
const APPLICATION: &str = "Milston";

const DATA_FILENAME: &str = "data.json";
const CONFIG_FILENAME: &str = "config.json";

/// The `Config` contains all the required config for `Milston` to work. It
/// contains the data origin for example so it's used consistently between
/// different calls to the library.
///
/// It's actually saved on the `config` dir for the user on the system, which
/// means that different clients will access the same config without the need
/// to reconfigure each one of them
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
