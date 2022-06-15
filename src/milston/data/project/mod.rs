pub mod task;

use self::task::{Task, TaskStatus};
use crate::helper::{AutoIncrementalMap, MapId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

type TaskId = MapId;
type Tasks = AutoIncrementalMap<Task>;

#[derive(Serialize, Deserialize)]
pub struct Project {
    title: String,
    tasks: Tasks,
    creation_timestamp: DateTime<Utc>,
}

impl Project {
    /// Creates a new `Project` instance. The default values have no tasks
    pub fn new<S: Into<String>>(name: S) -> Self {
        Project {
            title: name.into(),
            tasks: Tasks::default(),
            creation_timestamp: Utc::now(),
        }
    }

    /// Get the title
    pub fn get_title(&self) -> &str {
        &self.title
    }

    /// Set the title
    pub fn set_title<T: Into<String>>(&mut self, title: T) {
        self.title = title.into();
    }

    /// Add a new task
    pub fn add_task(&mut self, task: Task) -> TaskId {
        self.tasks.add(task)
    }

    /// Remove a task by Id, returns the task if it exists
    pub fn remove_task(&mut self, task_id: TaskId) -> Option<Task> {
        self.tasks.remove(task_id)
    }

    /// Clears all the tasks from a project
    pub fn clear_tasks(&mut self) {
        self.tasks = Tasks::default();
    }

    /// Get a reference to a task if it exists
    pub fn get_task(&self, task_id: TaskId) -> Option<&Task> {
        self.tasks.get(task_id)
    }

    /// Get a vector with all the tasks
    pub fn get_all_tasks(&self) -> Vec<&Task> {
        self.tasks.get_values()
    }

    /// Get a mutable reference to a task if it exists
    pub fn get_task_mut(&mut self, task_id: TaskId) -> Option<&mut Task> {
        self.tasks.get_mut(task_id)
    }

    /// Get the creation DateTime as UTC
    pub fn get_creation_timestamp(&self) -> DateTime<Utc> {
        self.creation_timestamp
    }

    /// Get the completion DateTime as UTC based on the tasks
    pub fn get_completion_timestamp(&self) -> Option<DateTime<Utc>> {
        self.tasks
            .get_values()
            .iter()
            .max_by(|x, y| {
                x.get_completion_timestamp()
                    .cmp(&y.get_completion_timestamp())
            })
            .and_then(|task| task.get_completion_timestamp())
    }

    /// Returns true if there aren't any task
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    /// Returns the amount of tasks
    pub fn get_tasks_count(&self) -> usize {
        self.tasks.get_values().len()
    }

    /// Returns true if all the tasks are completed
    pub fn is_completed(&self) -> bool {
        let tasks = self.tasks.get_values();
        tasks
            .iter()
            .all(|task| task.get_status() == TaskStatus::Completed)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        task::{Task, TaskStatus},
        Project,
    };

    #[test]
    fn create_new_project() {
        let project = Project::new("Test Project");
        assert_eq!(project.title, "Test Project");
        assert!(project.is_empty());
        assert!(project.is_completed());
    }

    #[test]
    fn add_new_task() {
        let mut project = Project::new("Test Project");
        let task = Task::new("Test Task", "Task for testing");
        let task_id = project.add_task(task);

        assert!(!project.is_empty());
        assert!(!project.is_completed());
        assert_eq!(task_id, 1);
    }

    #[test]
    fn change_task_status() {
        let mut project = Project::new("Test Project");
        let task = Task::new("Test Task", "Task for testing");
        let task_id = project.add_task(task);
        assert!(!project.is_completed());

        let task = project.get_task_mut(task_id).unwrap();
        task.set_status(TaskStatus::Completed);
        assert!(project.is_completed());
    }

    #[test]
    fn remove_task() {
        let mut project = Project::new("Test Project");
        let task = Task::new("Test Task", "Task for testing");
        let task_id = project.add_task(task);

        assert_eq!(project.get_tasks_count(), 1);
        project.remove_task(task_id);
        assert_eq!(project.get_tasks_count(), 0);
        assert!(project.is_empty());
    }

    #[test]
    fn clear_tasks() {
        let mut project = Project::new("Test Project");
        let task = Task::new("Test Task", "Task for testing");
        project.add_task(task);

        assert_eq!(project.get_tasks_count(), 1);
        project.clear_tasks();
        assert_eq!(project.get_tasks_count(), 0);
        assert!(project.is_empty());
    }
}
