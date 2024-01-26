use core::panic;
use std::ffi::OsStr;
use std::io;
use std::process::{Command, Output, Stdio};

pub fn execute<T>(cmd: &str, args: T) -> io::Result<Output>
where
    T: IntoIterator,
    T::Item: AsRef<OsStr>,
{
    Command::new(cmd)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
}

pub fn list_worktrees() {
    match execute("git", ["worktree", "list"]) {
        Err(e) => panic!("Something is wrong: {:?}", e),
        Ok(data) => println!("data: {:?}", data),
    }
}
