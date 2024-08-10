use inquire::{MultiSelect, Select};

pub fn single_select_from_options(
    description: &str,
    options: &[&str],
) -> Result<String, Box<dyn std::error::Error>> {
    let ans = Select::new(description, options.to_vec()).prompt();
    match ans {
        Ok(ans) => Ok(ans.into()),
        Err(_) => Err("Operation was canceled".into()),
    }
}

pub fn multi_select_from_options(
    description: &str,
    options: &[&str],
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let ans = MultiSelect::new(description, options.to_vec()).prompt();
    match ans {
        Ok(ans) => Ok(ans.into_iter().map(|x| x.into()).collect()),
        Err(_) => Err("Operation was canceled".into()),
    }
}
