use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum DataSource {
    File(PathBuf),
    #[cfg(feature = "http")]
    Http(url::Url),
}
