use anyhow::{Context, Result};

pub fn add() -> Result<()> {
    let repo = git2::Repository::open(".").unwrap();

    // Get the list of files that are not staged
    let mut files: Vec<String> = Vec::new();

    // Get the changes not yet staged
    let diff = repo.diff_index_to_workdir(None, None)?;
    for delta in diff.deltas() {
        files.push(
            delta
                .new_file()
                .path()
                .expect("Failed to get path")
                .to_string_lossy()
                .into(),
        );
    }

    // Get the untracked files
    let statuses = repo.statuses(Some(
        git2::StatusOptions::new()
            .include_untracked(true)
            .include_ignored(false),
    ))?;
    for status in statuses.iter() {
        if status.status().contains(git2::Status::WT_NEW) {
            files.push(
                status
                    .path()
                    .context("Failed to get path")?
                    .to_string()
                    .into(),
            );
        }
    }

    if files.is_empty() {
        println!("No files to add");
        return Ok(());
    }

    // Sort with filename
    files.sort();

    let files_to_add = crate::ui::multi_select_from_options(
        "Select files to add",
        &files.iter().map(String::as_str).collect::<Vec<_>>(),
    )?;

    let mut index = repo.index()?;

    index.add_all(files_to_add, git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;
    println!("Added files to stage");
    Ok(())
}
