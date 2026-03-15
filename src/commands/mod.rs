use crate::Shell;
use clap::{Parser, Subcommand};
use strum::IntoEnumIterator;

pub mod add;
pub mod init;
pub mod list;
pub mod pick;
pub mod remove;

pub use add::exec as add;
pub use init::exec as init;
pub use list::exec as list;
pub use pick::exec as pick;
pub use remove::exec as remove;

#[derive(Debug, Clone, Parser)]
#[clap(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    #[arg(long, global = true)]
    pub json: bool,
}

#[derive(Subcommand, Debug, strum_macros::Display, strum::EnumIter, Clone)]
pub enum Command {
    #[clap(alias = "root")]
    GetRoot,

    Init {
        shell: Option<Shell>,
    },

    #[clap(alias = "a")]
    Add {
        branch: String,

        #[clap(short)]
        b: bool,
    },

    /// Remove worktree
    #[clap(alias = "rm")]
    Remove,

    /// List available worktrees
    #[clap(alias = "ls")]
    List,

    /// print worktree path
    #[clap(alias = "p")]
    Pick {
        /// Print the path of the current worktree
        #[arg(long, short)]
        current: bool,
    },
}

pub(crate) fn list_commands() -> Vec<String> {
    Command::iter().map(|c| c.to_string()).collect()
}
