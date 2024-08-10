use inquire::{MultiSelect, Select};

pub fn single_select_from_options(description: &str, options: &[&str]) -> String {
    let ans = Select::new(description, options.to_vec()).prompt();
    match ans {
        Ok(ans) => ans.into(),
        Err(_) => {
            println!("Operation was canceled");
            "".into()
        }
    }
}

pub fn multi_select_from_options(description: &str, options: &[&str]) -> Vec<String> {
    let ans = MultiSelect::new(description, options.to_vec()).prompt();
    match ans {
        Ok(ans) => ans.into_iter().map(|x| x.into()).collect(),
        Err(_) => {
            println!("Operation was canceled");
            vec![]
        }
    }
}
