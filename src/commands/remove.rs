use anyhow::Result;
use inquire::MultiSelect;

use crate::git;

pub async fn exec() -> Result<()> {
    let worktrees = git::list_worktrees()?;
    let worktrees = MultiSelect::new("Delete a worktree", worktrees).prompt()?;

    let mut tasks = tokio::task::JoinSet::new();

    for worktree in worktrees {
        tasks.spawn_blocking(move || {
            git::remove_worktree(&worktree, false).map(|removed| (worktree, removed))
        });
    }

    while let Some(result) = tasks.join_next().await {
        match result {
            Ok(Ok((worktree, true))) => {
                println!("worktree removed successfully: {worktree:?}");
            }
            Ok(Ok((worktree, false))) => {
                eprintln!("worktree not removed: {worktree:?}");
            }
            Ok(Err(error)) => {
                eprintln!("failed to remove worktree: {error}");
            }
            Err(join_error) => {
                eprintln!("removal task panicked: {join_error}");
            }
        }
    }

    Ok(())
}
