use crate::command::Command;
use crate::error::{GiError, Result};
use crate::repo::GitRepo;
use crate::ui;

/// Delete local branches.
pub struct Delete;

impl Command for Delete {
    fn execute(repo: &GitRepo) -> Result<()> {
        let branches = repo.branch_names(false)?;

        if branches.is_empty() {
            return Err(GiError::Empty {
                context: "No branches to delete",
            });
        }

        let selected = ui::multi_select("Select branches to delete", &branches)?;

        for branch in &selected {
            repo.delete_branch(branch)?;
            println!("Deleted branch: {}", branch);
        }

        Ok(())
    }
}
