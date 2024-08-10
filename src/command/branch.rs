use git2::{Error, Repository};

pub fn get_branch_names_from_repository(repo: &Repository) -> Result<Vec<String>, Error> {
    let branches = repo.branches(None)?;
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
        let branches = get_branch_names_from_repository(&repo);
        assert!(branches.unwrap().contains(&"main".to_string()));
    }
}
