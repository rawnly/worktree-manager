use anyhow::Result;
use core::panic;
use serde::Serialize;
use std::ffi::OsStr;
use std::io;
use std::process::{Command, Output};

#[derive(Debug, Serialize, Default)]
pub struct Worktree {
    pub path: String,
    pub branch: String,
}

pub fn worktree_root() -> Result<String> {
    let pwd = std::env::var("PWD").unwrap();
    let stdout = Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .output()?
        .stdout;

    let value = String::from_utf8(stdout)?;
    let value = value.trim();

    let parts: Vec<&str> = value.split('/').collect();

    if parts.len() <= 1 {
        return Ok(pwd);
    }

    let path = parts[..parts.len() - 3].join("/");

    Ok(path)
}

pub fn remove_worktree(wk: &Worktree, force: bool) -> Result<bool> {
    let mut args = vec!["worktree", "remove", &wk.path];

    if force {
        args.push("--force");
    }

    let output = Command::new("git").args(&args).output()?;

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

pub fn execute<T>(cmd: &str, args: T) -> io::Result<Output>
where
    T: IntoIterator,
    T::Item: AsRef<OsStr>,
{
    Command::new(cmd).args(args).output()
}

pub fn list_worktrees() -> Vec<Worktree> {
    let mut worktrees: Vec<Worktree> = vec![];

    let output = match execute("git", ["worktree", "list"]) {
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

    worktrees
}
