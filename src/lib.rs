pub mod command;
pub mod error;
pub mod repo;
mod ui;

pub use command::{Add, Command, Commit, Delete, Fetch, Merge, MergeOptions, Push, PushOptions, Switch, run};
pub use error::{GiError, Result};
pub use repo::GitRepo;
