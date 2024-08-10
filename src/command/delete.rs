use crate::command::branch::get_branch_names_from_repository;

fn delete_branch(repo: &git2::Repository, branch: &str) {
    let mut branch = repo
        .find_branch(branch, git2::BranchType::Local)
        .expect("Failed to find branch");
    branch.delete().expect("Failed to delete branch");
}

pub fn delete() {
    let repo = git2::Repository::open(".").unwrap();
    let branches_string = get_branch_names_from_repository(&repo).expect("Failed to get branches");
    let branches_str: Vec<&str> = branches_string.iter().map(AsRef::as_ref).collect();
    let branch_to_delete =
        crate::ui::multi_select_from_options("Select a branch to delete to", &branches_str);

    for branch in branch_to_delete {
        delete_branch(&repo, &branch);
    }
}
