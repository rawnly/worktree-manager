use anyhow::Result;
use core::panic;
use git2::Repository;
use serde::Serialize;
use std::fmt::Display;

use crate::shell;

#[derive(Debug, Clone, Serialize, Default)]
pub struct Worktree {
    pub path: String,
    pub branch: String,
}

impl Display for Worktree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.branch)
    }
}

pub fn worktree_root() -> Result<String> {
    let repo = Repository::open_from_env()?;

    let git_common = repo.commondir();

    let git_common_path = if git_common.starts_with("/") {
        std::path::PathBuf::from(git_common)
    } else {
        std::env::current_dir()?.join(git_common)
    };

    let path = git_common_path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("cannot find parent directory"))?
        .to_string_lossy()
        .to_string();

    Ok(path)
}

pub fn remove_worktree(wk: &Worktree, force: bool) -> Result<bool> {
    let mut args = vec!["worktree", "remove", &wk.path];

    if force {
        args.push("--force");
    }

    let output = shell::execute("git", args)?;

    if output.stdout.is_empty() {
        let err = String::from_utf8(output.stderr)?;

        if err.contains("--force") {
            if inquire::Confirm::new("force delete?")
                .with_default(false)
                .prompt()?
            {
                return remove_worktree(wk, true);
            } else {
                return Ok(false);
            };
        }
    }

    Ok(true)
}
pub fn add_worktree(path: &str, branch: &str, create: bool) -> anyhow::Result<(Worktree, String)> {
    let data = if create {
        shell::execute("git", ["worktree", "add", &path, "-b", &branch])?
    } else {
        shell::execute("git", ["worktree", "add", &path, &branch])?
    };

    let stdout = String::from_utf8(data.stdout)?;

    if stdout.is_empty() {
        let stderr = String::from_utf8(data.stderr)?;

        return Err(anyhow::anyhow!(format!(
            "Error adding worktree: {}",
            stderr
        )));
    }

    Ok((
        Worktree {
            path: path.to_string(),
            branch: branch.to_string(),
        },
        stdout,
    ))
}
pub fn list_worktrees() -> anyhow::Result<Vec<Worktree>> {
    let mut worktrees: Vec<Worktree> = vec![];

    let output = match shell::execute("git", ["worktree", "list"]) {
        Err(e) => panic!("Something is wrong: {:?}", e),
        Ok(data) => String::from_utf8(data.stdout).unwrap(),
    };

    let parts: Vec<&str> = output.split('\n').collect();

    for part in parts {
        if part.is_empty() {
            continue;
        }

        let mut wk = Worktree::default();

        let items: Vec<&str> = part.splitn(2, ' ').collect();
        if let Some(path) = items.first() {
            wk.path = path.to_string()
        }

        if let Some(branch) = &part.split(' ').last() {
            let branch = branch.replace(['[', ']'], "");
            wk.branch = branch;
        }

        if wk.branch.is_empty() || wk.branch.starts_with('(') && wk.branch.ends_with(')') {
            continue;
        }

        worktrees.push(wk)
    }

    Ok(worktrees)
}
