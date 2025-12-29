use crate::error::Result;
use inquire::{MultiSelect, Select, Text};

/// Prompts user to select a single item from options.
///
/// # Type Parameters
/// * `T` - Any type that can be referenced as a string slice
pub fn select<T: AsRef<str>>(prompt: &str, options: &[T]) -> Result<String> {
    let options: Vec<&str> = options.iter().map(AsRef::as_ref).collect();
    let selected = Select::new(prompt, options).prompt()?;
    Ok(selected.to_string())
}

/// Prompts user to select multiple items from options.
///
/// # Type Parameters
/// * `T` - Any type that can be referenced as a string slice
pub fn multi_select<T: AsRef<str>>(prompt: &str, options: &[T]) -> Result<Vec<String>> {
    let options: Vec<&str> = options.iter().map(AsRef::as_ref).collect();
    let selected = MultiSelect::new(prompt, options).prompt()?;
    Ok(selected.into_iter().map(String::from).collect())
}

/// Prompts user for text input.
pub fn text(prompt: &str, help: Option<&str>) -> Result<String> {
    let mut builder = Text::new(prompt);
    if let Some(help_msg) = help {
        builder = builder.with_help_message(help_msg);
    }
    Ok(builder.prompt()?)
}

/// Prompts user for required text input (non-empty).
pub fn required_text(prompt: &str, help: Option<&str>) -> Result<Option<String>> {
    let input = text(prompt, help)?;
    Ok((!input.trim().is_empty()).then_some(input))
}
