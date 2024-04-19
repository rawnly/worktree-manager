use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use inquire::Select;

mod shell;

#[derive(Debug, Clone, Parser)]
#[clap(version)]
struct Args {
    #[command(subcommand)]
    command: Command,

    #[arg(long, global = true)]
    json: bool,
}

#[derive(Subcommand, Debug, Clone)]
enum Command {
    #[clap(alias = "root")]
    GetRoot,

    Init {
        #[arg(long)]
        no_alias: bool,

        #[arg(long)]
        no_git_alias: bool,
    },

    /// Remove worktree
    #[clap(alias = "rm")]
    Remove,

    /// List available worktrees
    #[clap(alias = "ls")]
    List,

    /// print worktree path
    #[clap(alias = "path")]
    PrintPath,
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Init {
            no_alias,
            no_git_alias,
        } => {
            let mut bash: String = r#"
function worktree-manager-go() {
    p="$(worktree-manager print-path)"

    cd "$p"
};
            "#
            .to_string();

            if !no_alias {
                bash += "\nalias wm=worktree-manager";
            }

            if !no_git_alias {
                bash += r#"
git config --global alias.wt "!worktree-manager"
git config --global alias.wtls "!worktree-manager list"
git config --global alias.wtrm "!worktree-manager remove"
                "#
            }

            println!("{bash}");
        }
        Command::PrintPath => {
            let worktrees = shell::list_worktrees();

            let branch = Select::new(
                "Delete a worktree",
                worktrees.iter().map(|w| w.branch.clone()).collect(),
            )
            .prompt()?;

            if let Some(wk) = worktrees.iter().find(|wk| wk.branch == branch) {
                println!("{}", wk.path);

                return Ok(());
            }

            println!("Invalid worktree branch: {}", branch.yellow());
            return Ok(());
        }
        Command::Remove => {
            let worktrees = shell::list_worktrees();

            let branch = Select::new(
                "Delete a worktree",
                worktrees.iter().map(|w| w.branch.clone()).collect(),
            )
            .prompt()?;

            if let Some(wk) = worktrees.iter().find(|wk| wk.branch == branch) {
                if shell::remove_worktree(wk, false)? {
                    println!("worktree removed successfully");
                } else {
                    println!("failed to  removed worktree");
                }

                return Ok(());
            }

            println!("Invalid worktree branch: {}", branch.yellow());
            return Ok(());
        }
        Command::GetRoot => {
            println!("{}", shell::worktree_root()?)
        }
        Command::List => {
            let worktrees = shell::list_worktrees();
            let root = shell::worktree_root()?;

            if args.json {
                let json_str = serde_json::to_string(&worktrees).unwrap();
                println!("{json_str}");
            }

            for wk in worktrees {
                println!(
                    "{} on {}",
                    wk.path.replace(&root, "@").yellow(),
                    wk.branch.blue()
                );
            }
        }
    }

    Ok(())
}
