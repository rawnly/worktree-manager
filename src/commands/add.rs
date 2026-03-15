use anyhow::Result;
use indoc::formatdoc;
use joyful::*;

use crate::git;

pub fn exec(create: bool, branch: &str) -> Result<()> {
    let root = git::worktree_root()?;
    let wk_name = joyful(Options::default()).map_err(|e| anyhow::anyhow!(e.to_string()))?;

    let path = format!("{root}/{wk_name}");

    let (wk, _) = git::add_worktree(&path, branch, create)?;

    termimad::print_text(&formatdoc! {"
                Successfully created _{name}_ with `{branch}`

                ```
                cd {name}
                ```

                ", 
        name=wk_name,
        branch=wk.branch
    });

    Ok(())
}
