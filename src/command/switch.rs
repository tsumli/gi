use crate::command::branch::get_branch_names_from_repository;

fn switch_branch(repo: &git2::Repository, branch: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (object, reference) = repo.revparse_ext(&branch).expect("Object not found");
    repo.checkout_tree(&object, None)
        .expect("Failed to checkout");

    match reference {
        // gref is an actual reference like branches or tags
        Some(gref) => repo.set_head(gref.name().unwrap()),
        // this is a commit, not a reference
        None => repo.set_head_detached(object.id()),
    }
    .expect("Failed to set HEAD");
    Ok(())
}

pub fn switch() -> Result<(), Box<dyn std::error::Error>> {
    let repo = git2::Repository::open(".").unwrap();
    let branches_string =
        get_branch_names_from_repository(&repo, true).expect("Failed to get branches");
    let branches_str: Vec<&str> = branches_string.iter().map(AsRef::as_ref).collect();
    let branch_to_switch =
        crate::ui::single_select_from_options("Select a branch to switch to", &branches_str)
            .expect("Failed to get branch");
    switch_branch(&repo, &branch_to_switch)
        .expect(&format!("Failed to switch branch: {}", branch_to_switch));
    Ok(())
}
