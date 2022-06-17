use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// The `DataResource` is an enum representing the type of data origin
/// for Milston. Different providers may be available dependening on the
/// enabled features.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum DataSource {
    File(PathBuf),
    #[cfg(feature = "http")]
    Http(url::Url),
}
