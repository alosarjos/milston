pub mod project;

use self::project::Project;
use crate::helper::{AutoIncrementalMap, MapId};
use serde::{Deserialize, Serialize};

pub type ProjectId = MapId;
pub type Projects = AutoIncrementalMap<Project>;

#[derive(Default, Deserialize, Serialize)]
pub struct Data {
    pub projects: Projects,
}
