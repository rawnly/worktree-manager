use anyhow::Result;
use indoc::formatdoc;
use inquire::MultiSelect;
use joyful::*;

use crate::git;

pub fn exec() -> Result<()> {
    let branches: Vec<String> = git::list_branches()
        .into_iter()
        .filter(|s| !s.starts_with("+"))
        .collect();

    let selected_branches = MultiSelect::new("Pick branches:", branches).prompt()?;

    if selected_branches.is_empty() {
        return Ok(());
    }

    let mut tasks = tokio::task::JoinSet::new();

    for branch in selected_branches {
        tasks.spawn_blocking(move || {
            println!("Creating worktree for branch: '{branch}'");
            let root = git::worktree_root()?;
            let wk_name = joyful(Options::default()).map_err(|e| anyhow::anyhow!(e.to_string()))?;
            let worktree_path = format!("{root}/{wk_name}");

            let (wk, _) = git::add_worktree(&worktree_path, &branch, false)?;

            termimad::print_text(&formatdoc! {"
            Successfully created _{name}_ with `{branch}`

            ```
                cd {name}
            ```

                ",
                name=wk_name,
                branch=wk.branch
            });

            Result::<(), anyhow::Error>::Ok(())
        });
    }

    Ok(())
}
