use anyhow::Result;

pub fn get_repo_root_recursive(depth: usize) -> Result<git2::Repository> {
    if depth == 0 {
        return Err(anyhow::anyhow!("Seems like not a git repository"));
    }

    let mut current_path = std::env::current_dir()?;

    for _ in 0..depth {
        // Try to open repository at current path
        if let Ok(repo) = git2::Repository::open(&current_path) {
            return Ok(repo);
        }

        // Move up to parent directory
        if let Some(parent) = current_path.parent() {
            current_path = parent.to_path_buf();
        } else {
            // Reached filesystem root, no more parents
            break;
        }
    }

    Err(anyhow::anyhow!(
        "No git repository found within {} levels",
        depth
    ))
}
