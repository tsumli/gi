use clap::{Parser, Subcommand};
use gi::{Add, Commit, Delete, Switch, run};

/// git interactive cli tool
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add files to staging area
    #[command(visible_alias = "a")]
    Add,
    /// Commit staged changes
    #[command(visible_alias = "c")]
    Commit,
    /// Delete local branches
    #[command(visible_alias = "d")]
    Delete,
    /// Switch to a branch
    #[command(visible_alias = "s")]
    Switch,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let result = match args.command {
        Commands::Add => run::<Add>(),
        Commands::Commit => run::<Commit>(),
        Commands::Delete => run::<Delete>(),
        Commands::Switch => run::<Switch>(),
    };

    result.map_err(Into::into)
}
