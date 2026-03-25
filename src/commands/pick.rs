use anyhow::Result;
use inquire::{InquireError, Select};

use crate::{fuzzy_scorer, git, shell, Worktree};

pub fn exec(current: bool) -> Result<()> {
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
    let worktree = match Select::new("Pick a worktree", options)
        .with_scorer(wk_scorer)
        .prompt()
    {
        Ok(wt) => wt,
        Err(InquireError::OperationCanceled) | Err(InquireError::OperationInterrupted) => {
            std::process::exit(130);
        }
        Err(e) => return Err(e.into()),
    };

    println!("{}", worktree.path);

    Ok(())
}
