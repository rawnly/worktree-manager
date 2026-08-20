use anyhow::Result;
use indoc::formatdoc;
use inquire::Select;
use joyful::*;

use crate::{git, project_config::ProjectConfig, shell};

fn create_worktree(name: &str, path: &str, branch: &str, create: bool) -> Result<()> {
    let (wk, _) = git::add_worktree(path, branch, create)?;

    termimad::print_text(&formatdoc! {"
                Successfully created _{name}_ with `{branch}`

                ```
                cd {name}
                ```

                ", 
        name=name,
        branch=wk.branch
    });

    Ok(())
}

pub fn exec(create: bool, branch: Option<String>) -> Result<()> {
    let root = git::worktree_root()?;
    let wk_name = joyful(Options::default()).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let project_config = ProjectConfig::load()?;

    let path = format!("{root}/{wk_name}");

    match branch {
        Some(branch) => create_worktree(&wk_name, &path, &branch, create),
        None => {
            let branches: Vec<String> = git::list_branches()
                .into_iter()
                .filter(|s| !s.starts_with("+"))
                .collect();

            let branch = Select::new("Pick a branch:", branches).prompt()?;

            create_worktree(&wk_name, &path, &branch, create)
        }
    }?;

    let hooks = project_config.hooks.unwrap_or_default();
    if hooks.on_create.is_some() {
        let handlers = hooks.on_create.unwrap_or_default();

        for h in handlers {
            let parts: Vec<&str> = h.split_whitespace().collect();

            if let Some((command, args)) = parts.split_first() {
                shell::execute_in_dir(command, args, &path)?;
            }
        }
    }

    Ok(())
}
