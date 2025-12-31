mod add;
mod commit;
mod delete;
mod push;
mod switch;

use crate::error::Result;
use crate::repo::GitRepo;

/// Trait for implementing git commands.
///
/// Each command receives a `GitRepo` and performs its operation.
pub trait Command {
    /// Executes the command.
    fn execute(repo: &GitRepo) -> Result<()>;
}

/// Convenience function to run a command.
pub fn run<C: Command>() -> Result<()> {
    let repo = GitRepo::discover(5)?;
    C::execute(&repo)
}

// Re-export command types
pub use add::Add;
pub use commit::Commit;
pub use delete::Delete;
pub use push::{Push, PushOptions};
pub use switch::Switch;
