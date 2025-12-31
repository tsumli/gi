use crate::command::Command;
use crate::error::{GiError, Result};
use crate::repo::GitRepo;
use crate::ui;
use std::process;

/// Push options.
#[derive(Default)]
pub struct PushOptions {
    pub force: bool,
}

/// Push current branch to remote.
pub struct Push;

impl Push {
    /// Executes push with options.
    pub fn execute_with_options(repo: &GitRepo, options: PushOptions) -> Result<()> {
        let workdir = repo.workdir().unwrap_or(repo.path());
        let branch = Self::current_branch_name(repo)?;

        // Check if upstream is set
        let has_upstream = Self::has_upstream(repo, &branch);

        // Build push command
        let mut args = vec!["push"];

        // Handle force push with safer options
        if options.force {
            let confirm = ui::select(
                &format!(
                    "Force push '{}'? This may overwrite remote changes.",
                    branch
                ),
                &["Yes", "No"],
            )?;

            if confirm == "No" {
                println!("Push cancelled");
                return Ok(());
            }

            args.push("--force-with-lease");
            args.push("--force-if-includes");
        }

        // If no upstream, ask user if they want to set it
        if !has_upstream {
            let confirm = ui::select(
                &format!(
                    "No upstream set. Push and set upstream to 'origin/{}'?",
                    branch
                ),
                &["Yes", "No"],
            )?;

            if confirm == "No" {
                println!("Push cancelled");
                return Ok(());
            }

            args.push("-u");
            args.push("origin");
            args.push(&branch);
        }

        let status = process::Command::new("git")
            .args(&args)
            .current_dir(workdir)
            .status()
            .map_err(|e| GiError::Other(e.into()))?;

        if !status.success() {
            return Err(GiError::Other(anyhow::anyhow!(
                "git push failed (exit code: {:?})",
                status.code()
            )));
        }

        Ok(())
    }

    fn current_branch_name(repo: &GitRepo) -> Result<String> {
        let head = repo.head().map_err(GiError::Git)?;
        head.shorthand()
            .map(String::from)
            .ok_or_else(|| GiError::Other(anyhow::anyhow!("HEAD is not a branch")))
    }

    fn has_upstream(repo: &GitRepo, branch_name: &str) -> bool {
        repo.find_branch(branch_name, git2::BranchType::Local)
            .ok()
            .and_then(|b| b.upstream().ok())
            .is_some()
    }
}

impl Command for Push {
    fn execute(repo: &GitRepo) -> Result<()> {
        Self::execute_with_options(repo, PushOptions::default())
    }
}
