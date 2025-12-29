use crate::command::Command;
use crate::error::Result;
use crate::repo::GitRepo;
use crate::ui;

/// Commit staged changes.
pub struct Commit;

impl Command for Commit {
    fn execute(repo: &GitRepo) -> Result<()> {
        if !repo.has_staged_changes()? {
            println!("No staged changes to commit");
            return Ok(());
        }

        let message = ui::required_text(
            "Commit message:",
            Some("Enter a descriptive commit message"),
        )?;

        let Some(message) = message else {
            println!("Aborting commit due to empty commit message");
            return Ok(());
        };

        repo.commit(&message)?;
        println!("Committed: {}", message);

        Ok(())
    }
}
