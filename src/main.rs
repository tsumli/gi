use anyhow::Result;
use clap::{Parser, Subcommand};

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
    #[command(visible_alias = "d")]
    Delete,
    /// Switch to a branch
    #[command(visible_alias = "s")]
    Switch,
    /// Add files
    #[command(visible_alias = "a")]
    Add,
}

fn main() -> Result<()> {
    let args = Args::parse();
    match &args.command {
        Commands::Delete {} => gi::command::delete::delete()?,
        Commands::Switch {} => gi::command::switch::switch()?,
        Commands::Add {} => gi::command::add::add()?,
    }
    Ok(())
}
