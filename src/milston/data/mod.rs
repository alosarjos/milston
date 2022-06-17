//! Here all the required structures for `Data` handling are available.
//! Completely separated from structures required to setup the `Milston`
//! settings.
//!
//! You can get access to all data structs like this:
//!
//! ```
//! use milston::data::*;
//! ```

mod project;

use crate::helper::{AutoIncrementalMap, MapId};
use serde::{Deserialize, Serialize};

pub type ProjectId = MapId;
type Projects = AutoIncrementalMap<Project>;

pub use project::{
    task::{Task, TaskStatus},
    Project, TaskId,
};

/// Data contains all the available structs required to
/// operate with Milston directly. The point on having a `config` and
/// a `data` module is to differenciate between which structs are related
/// to the different parts contained inside a `Milston` instance.
#[derive(Default, Deserialize, Serialize)]
pub struct Data {
    pub projects: Projects,
}
