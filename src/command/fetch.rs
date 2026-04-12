use crate::command::Command;
use crate::error::{GiError, Result};
use crate::repo::GitRepo;
use std::process;

/// Fetch all remotes with pruning.
pub struct Fetch;

impl Command for Fetch {
    fn execute(repo: &GitRepo) -> Result<()> {
        let workdir = repo.workdir().unwrap_or(repo.path());

        let status = process::Command::new("git")
            .args(["fetch", "--all", "--prune"])
            .current_dir(workdir)
            .status()
            .map_err(|e| GiError::Other(e.into()))?;

        if !status.success() {
            return Err(GiError::Other(anyhow::anyhow!(
                "git fetch failed (exit code: {:?})",
                status.code()
            )));
        }

        Ok(())
    }
}
