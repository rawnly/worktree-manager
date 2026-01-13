use anyhow::Result;
use core::panic;
use inquire::formatter::OptionFormatter;
use serde::Serialize;
use std::ffi::OsStr;
use std::io;
use std::process::{Command, Output};
use strum_macros::{Display, EnumString};

#[derive(Debug, Serialize, Default)]
pub struct Worktree {
    pub path: String,
    pub branch: String,
}

pub fn worktree_root() -> Result<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--git-common-dir"])
        .output()?;

    let git_common = String::from_utf8(output.stdout)?.trim().to_string();

    let git_common_path = if git_common.starts_with("/") {
        std::path::PathBuf::from(git_common)
    } else {
        std::env::current_dir()?.join(&git_common)
    };

    let path = git_common_path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("cannot find parent directory"))?
        .to_string_lossy()
        .to_string();

    Ok(path)

    // let pwd = std::env::var("PWD").unwrap();
    // let parts: Vec<&str> = value.split('/').collect();
    //
    // if parts.len() <= 1 {
    //     return Ok(pwd);
    // }
    //
    // let path = parts[..parts.len() - 3].join("/");
    //
    // Ok(path)
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

pub fn add_worktree(
    path: String,
    branch: String,
    create: bool,
) -> anyhow::Result<(Worktree, String)> {
    let data = if create {
        execute("git", ["worktree", "add", &path, "-b", &branch])?
    } else {
        execute("git", ["worktree", "add", &path, &branch])?
    };

    let stdout = String::from_utf8(data.stdout)?;

    Ok((Worktree { path, branch }, stdout))
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

/// Enum representing supported shell types
#[derive(Debug, Display, Clone, EnumString)]
pub enum Shell {
    #[strum(serialize = "bash")]
    Bash,
    #[strum(serialize = "zsh")]
    Zsh,
    #[strum(serialize = "fish")]
    Fish,
}

/// Generates shell-specific script based on the shell type
pub fn generate_hook_script(shell: Shell, no_alias: bool, no_git_alias: bool) -> String {
    let git_snippet = r#"
git config --global alias.wt "!worktree-manager"
git config --global alias.wtls "!worktree-manager list"
git config --global alias.wtrm "!worktree-manager remove"
    "#;

    let mut script: String;

    match shell {
        Shell::Bash => {
            script = r#"
function worktree-manager-go() {
    p="$(worktree-manager pick)"

    cd "$p"
};
            "#
            .to_string();

            if !no_alias {
                script += r#"
alias wm=worktree-manager
alias wmg=worktree-manager-go
                "#;
            }
        }
        Shell::Zsh => {
            script = r#"
worktree-manager-go() {
    p="$(worktree-manager pick)"

    cd "$p"
};
            "#
            .to_string();

            if !no_alias {
                script += r#"
alias wm=worktree-manager
alias wmg=worktree-manager-go
                "#;
            }
        }
        Shell::Fish => {
            script = r#"
function worktree-manager-go
    set p (worktree-manager pick)

    cd "$p"
end;
            "#
            .to_string();

            if !no_alias {
                script += r#"
alias wm 'worktree-manager'
alias wmg 'worktree-manager-go'
                "#;
            }
        }
    }

    if !no_git_alias {
        script += git_snippet;
    }

    script
}
