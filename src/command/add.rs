use crate::command::Command;
use crate::error::{GiError, Result};
use crate::repo::GitRepo;
use crate::ui;

/// Add files to staging area.
pub struct Add;

impl Command for Add {
    fn execute(repo: &GitRepo) -> Result<()> {
        let files = repo.unstaged_files()?;

        if files.is_empty() {
            println!("No files to add");
            return Ok(());
        }

        let selected = ui::multi_select("Select files to add", &files)?;

        if selected.is_empty() {
            return Err(GiError::Empty {
                context: "No files selected",
            });
        }

        repo.stage_files(&selected)?;
        println!("Added {} file(s) to stage", selected.len());

        Ok(())
    }
}
