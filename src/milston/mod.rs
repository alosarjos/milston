pub mod config;
pub mod data;
mod provider;

use self::config::DataSource;
use self::data::{Data, ProjectId};
use self::provider::DataProvider;
use self::{config::Config, data::Project};
use crate::error::Result;

/// Milston is the entry point to the API, is the struct that controls all the
/// data and the way it should be saved/loaded.
///
/// A full example on how to use it:
///
/// ```
/// use milston::{config::{Config, DataSource},  
///         data::{Data, Project, Task},
///         Milston,
///         error::Error as MilstonError};
///
/// use std::{path::PathBuf, str::FromStr};
///
/// #[tokio::main]
/// async fn main() -> Result<(), MilstonError> {
///     let config = Config {data_source:
///                     DataSource::File(
///                         PathBuf::from_str("/tmp/data.json")
///                         .unwrap()
///                     )};
///
///     let mut milston = Milston::new(Data::default(), config);
///     let mut project = Project::new("Milston");
///     let task = Task::new("Docs", "Learn to write docs and document the lib");
///
///     project.add_task(task);
///     milston.add_project(project);
///     milston.save_data().await?;
///
///     Ok(())
/// }
/// ```
///
#[derive(Default)]
pub struct Milston {
    data: Data,
    config: Config,
}

impl Milston {
    /// Create a new `Milston` instance
    pub fn new(data: Data, config: Config) -> Milston {
        Milston { data, config }
    }

    /// Clears the whole data
    pub fn clear_data(&mut self) {
        self.data = Data::default();
    }

    /// Get the list of projects
    pub fn get_projects(&self) -> Vec<&Project> {
        self.data.get_projects().get_values()
    }

    /// Get an specific project by Id
    pub fn get_project(&self, id: ProjectId) -> Option<&Project> {
        self.data.get_projects().get(id)
    }

    /// Returns a mutable reference to a project, allowing it's modification.
    pub fn get_project_mut(&mut self, id: ProjectId) -> Option<&mut Project> {
        self.data.get_projects_mut().get_mut(id)
    }

    /// Adds a new project and auto-generates and Id for it
    pub fn add_project(&mut self, project: Project) -> ProjectId {
        self.data.get_projects_mut().add(project)
    }

    /// Removes a project and returns it
    pub fn remove_project(&mut self, id: ProjectId) -> Option<Project> {
        self.data.get_projects_mut().remove(id)
    }

    /// Sets the data source from the available list (File or Http for example)
    pub fn set_data_source(&mut self, source: DataSource) {
        self.config.data_source = source
    }

    /// Saves the current data into the configured data source
    pub async fn save_data(&self) -> Result<()> {
        DataProvider::save(&self.config.data_source, &self.data).await
    }

    /// Loads the data (overwritting the existing) from the configured
    /// data source
    pub async fn load_data(&mut self) -> Result<()> {
        self.data = DataProvider::load(&self.config.data_source).await?;
        Ok(())
    }

    /// Saves the current config from the filesystem
    pub async fn save_config(&self) -> Result<()> {
        DataProvider::save(&Config::get_data_source(), &self.config).await
    }

    /// Loads the config from the local filesystem
    pub async fn load_config(&mut self) -> Result<()> {
        self.config = DataProvider::load(&Config::get_data_source()).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::{
        data::{Project, Task},
        Milston,
    };

    #[test]
    fn create_simple_milston() {
        let mut milston = Milston::default();
        let mut project = Project::new("Test Project");
        let task = Task::new("Test Task", "Test Description");

        let task_id = project.add_task(task);
        let project_id = milston.add_project(project);

        let project = milston.get_project(project_id).unwrap();
        assert_eq!(project.get_title(), "Test Project");

        let task = project.get_task(task_id).unwrap();
        assert_eq!(task.title, "Test Task");
    }

    #[test]
    fn change_project_name() {
        let mut milston = Milston::default();
        let project = Project::new("Test Project");
        let project_id = milston.add_project(project);

        let project_mut = milston.get_project_mut(project_id).unwrap();
        project_mut.set_title("Modified Project");

        let project = milston.get_project(project_id).unwrap();
        assert_eq!(project.get_title(), "Modified Project");
    }
}
