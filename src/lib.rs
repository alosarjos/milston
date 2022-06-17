//! Standard library for projects and tasks management
//!
//! Milston aims to provide the required methods to abstract any implementation
//! for a project/task management app. Since it holds the data and config
//! structure, different apps using this library are compatible between themselves
//! and the data can be shared between them, without the need to export and import
//! it between different clients.
//!
//! # Quick Start
//!
//! To begin, create a new [`Milston`] instance.
//!
//!
//! ```
//! use milston::Milston;
//!
//! let milston = Milston::default();
//! ```
//!
//! A default Milston instance will save the data in the local storage as a JSON
//! file.
//!
//! # Features
//!
//! Milston providers different data sources to configure where the data should be saved.
//!
//! By default, the only available source is the local filesystem. The available features are:
//!
//! - `http`: Allows saving data to a REST endpoint by sending GET and POST requests
//!
//! To set the features, specificy them on the `Cargo.toml` file.
//!
//! ```toml
//! milston = { version = "0.1", features = ["http"] }
//! ```

mod error;
mod helper;

mod milston;

pub use crate::error::Error;
pub use crate::milston::{config, data, Milston};
