use anyhow::Result;
use clap::Parser;
use shell::Shell;

mod commands;
mod git;
mod shell;
mod utils;
mod version_check;

use crate::{
    commands::{Cli, Command},
    git::Worktree,
};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    let thread_handle = tokio::spawn(async {
        version_check::is_update_available()
            .await
            .unwrap_or_default()
    });

    match args.command {
        Command::Init { shell } => commands::init(shell),
        Command::Pick { current } => commands::pick(current)?,
        Command::Remove => commands::remove()?,
        Command::Add { b, branch } => commands::add(b, &branch)?,
        Command::GetRoot => println!("{}", git::worktree_root()?),
        Command::List => commands::list(args.json)?,
    }

    if let Some(version) = thread_handle.await? {
        eprintln!("A new update is available! {version}");
        eprintln!("run `brew upgrade bosco` to update");
    }

    Ok(())
}
