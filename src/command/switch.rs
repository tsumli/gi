use crate::command::Command;
use crate::error::{GiError, Result};
use crate::repo::GitRepo;
use crate::ui;

/// Switch to a different branch.
pub struct Switch;

impl Command for Switch {
    fn execute(repo: &GitRepo) -> Result<()> {
        let branches = repo.branch_names(true)?;

        if branches.is_empty() {
            return Err(GiError::Empty {
                context: "No branches available",
            });
        }

        let selected = ui::select("Select a branch to switch to", &branches)?;
        repo.switch_to(&selected)?;
        println!("Switched to branch: {}", selected);

        Ok(())
    }
}
