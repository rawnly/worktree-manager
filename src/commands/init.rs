use crate::shell;

pub fn exec(shell: Option<shell::Shell>) {
    let shell = shell.or(shell::detect());
    let hook = shell::generate_hook(shell.unwrap_or_default());
    eprintln!("{hook}")
}
