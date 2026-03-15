use anyhow::Result;
use colored::*;

use crate::git;

pub fn exec(json: bool) -> Result<()> {
    let worktrees = git::list_worktrees()?;
    let root = git::worktree_root()?;

    if json {
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

    Ok(())
}
