use crate::command::Command;
use crate::error::{GiError, Result};
use crate::repo::GitRepo;
use crate::ui;
use std::process;

/// Merge options.
#[derive(Default)]
pub struct MergeOptions {
    pub no_ff: bool,
    pub squash: bool,
}

/// Merge a branch into current branch.
pub struct Merge;

impl Merge {
    /// Executes merge with options.
    pub fn execute_with_options(repo: &GitRepo, options: MergeOptions) -> Result<()> {
        let workdir = repo.workdir().unwrap_or(repo.path());
        let current = Self::current_branch_name(repo)?;

        // Check if there's a merge in progress
        if repo.path().join("MERGE_HEAD").exists() {
            let action = ui::select(
                "Merge in progress. What do you want to do?",
                &["Continue (commit)", "Abort merge"],
            )?;

            return match action {
                s if s == "Abort merge" => Self::run_git(workdir, &["merge", "--abort"]),
                _ => Self::run_git(workdir, &["commit", "--no-edit"]),
            };
        }

        // Get branches excluding current
        let branches: Vec<String> = repo
            .branch_names(true)?
            .into_iter()
            .filter(|b| b != &current)
            .collect();

        if branches.is_empty() {
            return Err(GiError::Empty {
                context: "No branches to merge",
            });
        }

        let selected = ui::select(
            &format!("Select branch to merge into '{}'", current),
            &branches,
        )?;

        // Fetch if it's a remote branch (contains '/')
        if let Some((remote, branch)) = selected.split_once('/') {
            println!("Fetching '{}'...", selected);
            Self::run_git(workdir, &["fetch", remote, branch])?;
        }

        // Build merge command
        let mut args = vec!["merge"];

        if options.no_ff {
            args.push("--no-ff");
        }

        if options.squash {
            args.push("--squash");
        }

        args.push(&selected);

        Self::run_git(workdir, &args)?;

        if options.squash {
            println!("Squash merge completed. Changes are staged but not committed.");
            println!("Run 'gi c' to commit.");
        }

        Ok(())
    }

    fn run_git(workdir: &std::path::Path, args: &[&str]) -> Result<()> {
        let status = process::Command::new("git")
            .args(args)
            .current_dir(workdir)
            .status()
            .map_err(|e| GiError::Other(e.into()))?;

        if !status.success() {
            return Err(GiError::Other(anyhow::anyhow!(
                "git {} failed (exit code: {:?})",
                args.join(" "),
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
}

impl Command for Merge {
    fn execute(repo: &GitRepo) -> Result<()> {
        Self::execute_with_options(repo, MergeOptions::default())
    }
}
