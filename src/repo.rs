use crate::error::{GiError, Result};
use git2::{BranchType, Diff, Repository, Signature, Status, StatusOptions};
use std::ops::Deref;
use std::path::PathBuf;

/// A wrapper around `git2::Repository` with convenient extension methods.
///
/// Uses `Deref` to allow transparent access to all `Repository` methods.
pub struct GitRepo(Repository);

impl GitRepo {
    /// Opens a repository by searching up the directory tree.
    ///
    /// # Arguments
    /// * `max_depth` - Maximum number of parent directories to search
    pub fn discover(max_depth: usize) -> Result<Self> {
        let mut current = std::env::current_dir().map_err(|e| GiError::Other(e.into()))?;

        for _ in 0..max_depth {
            if let Ok(repo) = Repository::open(&current) {
                return Ok(Self(repo));
            }
            current = current
                .parent()
                .map(PathBuf::from)
                .ok_or(GiError::NotARepository { depth: max_depth })?;
        }

        Err(GiError::NotARepository { depth: max_depth })
    }

    /// Gets branch names from the repository.
    ///
    /// # Arguments
    /// * `include_remote` - Whether to include remote branches
    pub fn branch_names(&self, include_remote: bool) -> Result<Vec<String>> {
        let filter = (!include_remote).then_some(BranchType::Local);

        self.0
            .branches(filter)?
            .filter_map(|b| b.ok())
            .filter_map(|(branch, _)| branch.name().ok().flatten().map(String::from))
            .collect::<Vec<_>>()
            .pipe(Ok)
    }

    /// Gets unstaged files (modified + untracked).
    pub fn unstaged_files(&self) -> Result<Vec<String>> {
        let mut files = Vec::new();

        // Modified files
        let diff = self.0.diff_index_to_workdir(None, None)?;
        files.extend(
            diff.deltas()
                .filter_map(|d| d.new_file().path())
                .map(|p| p.to_string_lossy().into_owned()),
        );

        // Untracked files
        let statuses = self.0.statuses(Some(
            StatusOptions::new()
                .include_untracked(true)
                .include_ignored(false),
        ))?;

        files.extend(
            statuses
                .iter()
                .filter(|s| s.status().contains(Status::WT_NEW))
                .filter_map(|s| s.path().map(String::from)),
        );

        files.sort();
        Ok(files)
    }

    /// Gets the diff of staged changes.
    pub fn staged_diff(&self) -> Result<Diff<'_>> {
        let parent_tree = self
            .0
            .head()
            .ok()
            .and_then(|h| h.peel_to_commit().ok())
            .and_then(|c| c.tree().ok());

        Ok(self
            .0
            .diff_tree_to_index(parent_tree.as_ref(), None, None)?)
    }

    /// Checks if there are staged changes.
    pub fn has_staged_changes(&self) -> Result<bool> {
        Ok(self.staged_diff()?.deltas().count() > 0)
    }

    /// Gets the default signature for commits.
    pub fn default_signature(&self) -> Result<Signature<'_>> {
        self.0.signature().map_err(|_| {
            GiError::Other(anyhow::anyhow!(
                "Failed to get signature. Please configure user.name and user.email"
            ))
        })
    }

    /// Switches to a branch.
    pub fn switch_to(&self, branch: &str) -> Result<()> {
        let (object, reference) = self.0.revparse_ext(branch)?;
        self.0.checkout_tree(&object, None)?;

        match reference {
            Some(gref) => self.0.set_head(gref.name().unwrap()),
            None => self.0.set_head_detached(object.id()),
        }?;

        Ok(())
    }

    /// Deletes a local branch.
    pub fn delete_branch(&self, name: &str) -> Result<()> {
        let mut branch = self.0.find_branch(name, BranchType::Local)?;
        branch.delete()?;
        Ok(())
    }

    /// Creates a commit with the staged changes.
    pub fn commit(&self, message: &str) -> Result<git2::Oid> {
        let signature = self.default_signature()?;
        let mut index = self.0.index()?;
        let tree_oid = index.write_tree()?;
        let tree = self.0.find_tree(tree_oid)?;

        let parent = self.0.head().ok().and_then(|h| h.peel_to_commit().ok());

        let parents: Vec<&git2::Commit> = parent.iter().collect();

        let oid = self.0.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &parents,
        )?;

        Ok(oid)
    }

    /// Stages files by path patterns.
    pub fn stage_files<I, S>(&self, paths: I) -> Result<()>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut index = self.0.index()?;
        let patterns: Vec<_> = paths.into_iter().map(|s| s.as_ref().to_string()).collect();
        index.add_all(patterns, git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;
        Ok(())
    }
}

impl Deref for GitRepo {
    type Target = Repository;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Extension trait for pipe operator (method chaining).
trait Pipe: Sized {
    fn pipe<F, R>(self, f: F) -> R
    where
        F: FnOnce(Self) -> R,
    {
        f(self)
    }
}

impl<T> Pipe for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discover_repository() {
        let repo = GitRepo::discover(5);
        assert!(repo.is_ok());
    }

    #[test]
    fn test_branch_names() {
        let repo = GitRepo::discover(5).unwrap();
        let branches = repo.branch_names(false).unwrap();
        assert!(branches.contains(&"main".to_string()));
    }
}
