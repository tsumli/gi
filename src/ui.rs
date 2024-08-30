use anyhow::Result;
use inquire::{MultiSelect, Select};

pub fn single_select_from_options(description: &str, options: &[&str]) -> Result<String> {
    let ans = Select::new(description, options.to_vec()).prompt()?;
    Ok(ans.into())
}

pub fn multi_select_from_options(description: &str, options: &[&str]) -> Result<Vec<String>> {
    let ans = MultiSelect::new(description, options.to_vec()).prompt()?;
    Ok(ans.into_iter().map(|x| x.into()).collect())
}
