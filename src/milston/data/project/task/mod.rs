use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub description: String,
    status: TaskStatus,
    creation_timestamp: DateTime<Utc>,
    completion_timestamp: Option<DateTime<Utc>>,
}

impl Task {
    /// Creates a new `Task` instance
    /// The default status is `Todo`
    pub fn new<S: Into<String>>(title: S, description: S) -> Self {
        Task {
            title: title.into(),
            description: description.into(),
            status: TaskStatus::Todo,
            creation_timestamp: Utc::now(),
            completion_timestamp: None,
        }
    }

    /// Get the status
    pub fn get_status(&self) -> TaskStatus {
        self.status
    }

    /// Set the status
    pub fn set_status(&mut self, status: TaskStatus) {
        self.status = status;
        self.completion_timestamp = match status {
            TaskStatus::Completed => Some(Utc::now()),
            _ => None,
        };
    }

    /// Get the creation DateTime as UTC
    pub fn get_creation_timestamp(&self) -> DateTime<Utc> {
        self.creation_timestamp
    }

    /// Get the completion DateTime as UTC if it's finished
    pub fn get_completion_timestamp(&self) -> Option<DateTime<Utc>> {
        self.completion_timestamp
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    Todo,
    Doing,
    Completed,
}
