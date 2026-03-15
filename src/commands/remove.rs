use anyhow::Result;
use inquire::Select;

use crate::git;

pub fn exec() -> Result<()> {
    let worktrees = git::list_worktrees()?;

    let worktree = Select::new("Delete a worktree", worktrees).prompt()?;

    if git::remove_worktree(&worktree, false)? {
        println!("worktree removed successfully");
    } else {
        eprintln!("failed to  removed worktree");
    }

    Ok(())
}
