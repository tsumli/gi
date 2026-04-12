use crate::command::Command;
use crate::error::{GiError, Result};
use crate::repo::GitRepo;
use crate::ui;
use std::process;

/// Commit staged changes.
pub struct Commit;

impl Commit {
    fn normalize_message(message: String) -> Option<String> {
        (!message.trim().is_empty()).then_some(message)
    }

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

    /// Executes commit, optionally using a provided commit message.
    pub fn execute_with_message(repo: &GitRepo, message: Option<String>) -> Result<()> {
        if !repo.has_staged_changes()? {
            println!("No staged changes to commit");
            return Ok(());
        }

        let message = match message {
            Some(message) => Self::normalize_message(message),
            None => ui::required_text(
                "Commit message:",
                Some("Enter a descriptive commit message"),
            )?,
        };

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

impl Command for Commit {
    fn execute(repo: &GitRepo) -> Result<()> {
        Self::execute_with_message(repo, None)
    }
}

#[cfg(test)]
mod tests {
    use super::Commit;

    #[test]
    fn normalize_message_keeps_non_empty_message() {
        assert_eq!(
            Commit::normalize_message("commit message".to_string()),
            Some("commit message".to_string())
        );
    }

    #[test]
    fn normalize_message_rejects_blank_message() {
        assert_eq!(Commit::normalize_message("   ".to_string()), None);
    }
}
