use anyhow::Result;
use clap::Parser;
use colored::*;
use indoc::formatdoc;
use inquire::Select;
use joyful::{joyful, Options};
use shell::Shell;

mod commands;
mod git;
mod shell;
mod utils;

use crate::{
    commands::{Cli, Command},
    git::Worktree,
};

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Command::Init { shell } => commands::init(shell),
        Command::Pick { current } => {
            let worktrees = git::list_worktrees()?;

            // currentWorktree
            let current_worktree = worktrees.iter().find(|wk| {
                if let Ok(p) = shell::execute::<Vec<String>>("pwd", vec![]) {
                    match String::from_utf8(p.stdout) {
                        Ok(v) => {
                            return v.trim_end() == wk.path;
                        }
                        _ => return false,
                    }
                }

                false
            });

            let mut options = worktrees.clone();

            if let Some(cwt) = current_worktree {
                // remove current dir from the list
                options.retain(|e| e.path != cwt.path);

                if current {
                    println!("{}", cwt.path)
                }
            }

            if options.is_empty() {
                let bin = env!("CARGO_BIN_NAME");

                eprintln!("No worktrees.");
                eprintln!("To get started run `{bin} add`");

                return Ok(());
            }

            fuzzy_scorer!(wk_scorer, Worktree);
            let worktree = Select::new("Pick a worktree", options)
                .with_scorer(wk_scorer)
                .prompt()?;
            println!("{}", worktree.path);

            return Ok(());
        }
        Command::Remove => {
            let worktrees = git::list_worktrees()?;

            let branch = Select::new(
                "Delete a worktree",
                worktrees.iter().map(|w| w.branch.clone()).collect(),
            )
            .prompt()?;

            if let Some(wk) = worktrees.iter().find(|wk| wk.branch == branch) {
                if git::remove_worktree(wk, false)? {
                    println!("worktree removed successfully");
                } else {
                    println!("failed to  removed worktree");
                }

                return Ok(());
            }

            println!("Invalid worktree branch: {}", branch.yellow());
            return Ok(());
        }
        Command::Add { b, branch } => {
            let root = git::worktree_root()?;
            let wk_name = joyful(Options::default()).map_err(|e| anyhow::anyhow!(e.to_string()))?;

            let path = format!("{root}/{wk_name}");

            let (wk, _) = git::add_worktree(path, branch, b)?;

            termimad::print_text(&formatdoc! {"
                Successfully created _{name}_ with `{branch}`

                ```
                cd {name}
                ```

                ", 
                name=wk_name,
                branch=wk.branch
            });
        }
        Command::GetRoot => {
            println!("{}", git::worktree_root()?)
        }
        Command::List => {
            let worktrees = git::list_worktrees()?;
            let root = git::worktree_root()?;

            if args.json {
                let json_str = serde_json::to_string(&worktrees).unwrap();
                println!("{json_str}");
                return Ok(());
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
