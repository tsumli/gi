use crate::command::Command;
use crate::error::{GiError, Result};
use crate::repo::GitRepo;
use crate::ui;
use git2::BranchType;
use std::process;

/// Merge options.
#[derive(Default)]
pub struct MergeOptions {
    pub no_ff: bool,
    pub squash: bool,
}

/// Information about a remote branch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteBranch {
    pub remote: String,
    pub branch: String,
}

/// Checks if a branch name is a remote tracking branch using git2 API.
///
/// Returns `Some(RemoteBranch)` if it's a remote branch, `None` if local.
fn parse_remote_branch(repo: &GitRepo, branch_name: &str) -> Option<RemoteBranch> {
    // Not a remote branch
    if repo.find_branch(branch_name, BranchType::Remote).is_err() {
        return None;
    }

    // Remote branch names are formatted as "remote/branch"
    // e.g., "origin/main", "origin/feat/foo"
    // Find the matching remote name from configured remotes
    let remotes = repo.remotes().ok()?;

    (0..remotes.len())
        .filter_map(|i| remotes.get(i))
        .find_map(|remote| {
            let prefix = format!("{}/", remote);
            branch_name
                .strip_prefix(&prefix)
                .map(|branch| RemoteBranch {
                    remote: remote.to_string(),
                    branch: branch.to_string(),
                })
        })
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

        // Fetch if it's a remote branch
        if let Some(remote_branch) = parse_remote_branch(repo, &selected) {
            println!("Fetching '{}'...", selected);
            Self::run_git(
                workdir,
                &["fetch", &remote_branch.remote, &remote_branch.branch],
            )?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_remote_branch_with_remote() {
        let repo = GitRepo::discover(5).unwrap();

        // origin/main should be detected as remote branch
        let result = parse_remote_branch(&repo, "origin/main");
        assert!(result.is_some());
        let remote_branch = result.unwrap();
        assert_eq!(remote_branch.remote, "origin");
        assert_eq!(remote_branch.branch, "main");
    }

    #[test]
    fn test_parse_remote_branch_with_nested_name() {
        let repo = GitRepo::discover(5).unwrap();

        // If origin/feat/foo exists as a remote branch, it should parse correctly
        // Note: This test depends on the actual repository state
        if let Some(remote_branch) = parse_remote_branch(&repo, "origin/feat/foo") {
            assert_eq!(remote_branch.remote, "origin");
            assert_eq!(remote_branch.branch, "feat/foo");
        }
    }

    #[test]
    fn test_parse_remote_branch_with_local() {
        let repo = GitRepo::discover(5).unwrap();

        // main (local branch) should return None
        let result = parse_remote_branch(&repo, "main");
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_remote_branch_local_with_slash() {
        let repo = GitRepo::discover(5).unwrap();

        // A local branch like "feat/foobar" should return None
        // (unless it happens to exist as a remote branch)
        // We check that it doesn't incorrectly parse as remote
        if repo.find_branch("feat/foobar", BranchType::Local).is_ok() {
            let result = parse_remote_branch(&repo, "feat/foobar");
            // If it's only local, should be None
            if repo.find_branch("feat/foobar", BranchType::Remote).is_err() {
                assert!(result.is_none());
            }
        }
    }
}
