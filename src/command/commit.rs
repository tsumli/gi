use crate::command::Command;
use crate::error::{GiError, Result};
use crate::repo::GitRepo;
use crate::ui;
use std::process;

/// Commit staged changes.
pub struct Commit;

impl Commit {
    /// Runs a git hook if it exists. Returns Ok if hook doesn't exist or succeeds.
    fn run_hook(repo: &GitRepo, hook_name: &str) -> Result<()> {
        let workdir = repo.workdir().unwrap_or(repo.path());

        let output = process::Command::new("git")
            .args(["hook", "run", hook_name])
            .current_dir(workdir)
            .output()
            .map_err(|e| GiError::Other(e.into()))?;

        // Check if hook doesn't exist (exit code is not 0 and stderr contains "cannot find")
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("cannot find") || stderr.contains("not found") {
            // Hook doesn't exist, that's fine
            return Ok(());
        }

        if !output.status.success() {
            return Err(GiError::Other(anyhow::anyhow!(
                "{} hook failed:\n{}",
                hook_name,
                stderr.trim()
            )));
        }

        Ok(())
    }
}

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

        // Run pre-commit hook (skips if not found)
        Self::run_hook(repo, "pre-commit")?;

        // Create the commit using git2
        repo.commit(&message)?;

        // Run post-commit hook (ignore errors as commit is already done)
        let _ = Self::run_hook(repo, "post-commit");

        println!("Committed: {}", message);
        Ok(())
    }
}
