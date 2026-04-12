use clap::{Parser, Subcommand};
use gi::{
    Add, Commit, Delete, Fetch, GitRepo, Merge, MergeOptions, Push, PushOptions, Switch, run,
};

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
    Commit {
        /// Commit message. If omitted, prompts interactively.
        #[arg(value_name = "MESSAGE")]
        message: Option<String>,
    },
    /// Delete local branches
    #[command(visible_alias = "d")]
    Delete,
    /// Fetch all remotes with pruning
    #[command(visible_alias = "f")]
    Fetch,
    /// Merge a branch into current branch
    #[command(visible_alias = "m")]
    Merge {
        /// Create merge commit even for fast-forward
        #[arg(long)]
        no_ff: bool,
        /// Squash commits (stages changes without committing)
        #[arg(long)]
        squash: bool,
    },
    /// Push current branch to remote
    #[command(visible_alias = "p")]
    Push {
        /// Force push (uses --force-with-lease --force-if-includes)
        #[arg(short, long)]
        force: bool,
    },
    /// Switch to a branch
    #[command(visible_alias = "s")]
    Switch,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let result = match args.command {
        Commands::Add => run::<Add>(),
        Commands::Commit { message } => {
            let repo = GitRepo::discover(5)?;
            Commit::execute_with_message(&repo, message)
        }
        Commands::Delete => run::<Delete>(),
        Commands::Fetch => run::<Fetch>(),
        Commands::Merge { no_ff, squash } => {
            let repo = GitRepo::discover(5)?;
            Merge::execute_with_options(&repo, MergeOptions { no_ff, squash })
        }
        Commands::Push { force } => {
            let repo = GitRepo::discover(5)?;
            Push::execute_with_options(&repo, PushOptions { force })
        }
        Commands::Switch => run::<Switch>(),
    };

    result.map_err(Into::into)
}
