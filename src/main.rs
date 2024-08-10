use clap::{Parser, Subcommand};
use std::error::Error;

/// git command with interactive shell
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// subcommand to run
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Delete branches
    Delete,
    /// Switch to a branch
    Switch,
    /// Add files
    Add,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    match &args.command {
        Commands::Delete {} => gi::command::delete::delete().expect("Failed to delete branch"),
        Commands::Switch {} => gi::command::switch::switch().expect("Failed to switch branch"),
        Commands::Add {} => gi::command::add::add().expect("Failed to add files"),
    }
    Ok(())
}
