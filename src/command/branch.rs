use git2::{BranchType, Error, Repository};

pub fn get_branch_names_from_repository(
    repo: &Repository,
    include_remote: bool,
) -> Result<Vec<String>, Error> {
    let branch_filter = if include_remote {
        None
    } else {
        Some(BranchType::Local)
    };

    let branches = repo
        .branches(branch_filter)
        .expect("Failed to get branches");
    let mut branches_string: Vec<String> = Vec::new();
    for branch in branches {
        let (branch, _) = branch?;
        if let Some(name) = branch.name()? {
            branches_string.push(name.to_string());
        }
    }
    Ok(branches_string)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_branch_names_from_repository() {
        let repo = Repository::open(".").unwrap();
        let branches = get_branch_names_from_repository(&repo, false);
        assert!(branches.as_ref().unwrap().contains(&"main".to_string()));
    }
}
