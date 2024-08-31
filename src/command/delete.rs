use crate::command::branch::get_branch_names_from_repository;
use anyhow::{Context, Result};

fn delete_branch(repo: &git2::Repository, branch: &str) -> Result<()> {
    let mut branch = repo.find_branch(branch, git2::BranchType::Local)?;
    branch.delete()?;
    Ok(())
}

pub fn delete() -> Result<()> {
    let repo = git2::Repository::open(".")?;
    let branches_string = get_branch_names_from_repository(&repo, false)?;
    let branches_str: Vec<&str> = branches_string.iter().map(AsRef::as_ref).collect();
    let branch_to_delete =
        crate::ui::multi_select_from_options("Select a branch to delete to", &branches_str)?;

    for branch in branch_to_delete {
        delete_branch(&repo, &branch)
            .with_context(|| format!("Failed to delete branch: {}", branch))?;
    }
    Ok(())
}
