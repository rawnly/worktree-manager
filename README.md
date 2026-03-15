# bosco

Friendly git worktree manager for side projects.

Pairs well with [bivio](https://github.com/rawnly/bivio).

Bosco keeps your worktrees organized under a single root, makes new worktrees with a single command, and lets you jump between them quickly.

## Highlights
- Create worktrees with memorable names
- List, pick, and remove worktrees interactively
- Optional `--json` output for scripting
- Shell hook for fast `cd` into a worktree

## Install

### Homebrew
```bash
brew install bosco
```

### From source
```bash
cargo install --path .
```

## Quick start

Add the shell hook (prints a function you can source):
```bash
bosco init zsh
# or: bosco init bash
# or: bosco init fish
```

Create a worktree:
```bash
bosco add <branch>
bosco add -b <new-branch>
```

Jump to a worktree:
```bash
bosco pick
```

List worktrees:
```bash
bosco list
bosco list --json
```

Remove a worktree:
```bash
bosco remove
```

## Usage
```text
Usage: bosco [OPTIONS] <COMMAND>

Commands:
  get-root
  init
  add
  remove    Remove worktree
  list      List available worktrees
  pick      print worktree path
  help      Print this message or the help of the given subcommand(s)

Options:
      --json
  -h, --help     Print help
  -V, --version  Print version
```

## How it works
- Worktrees are created under the git common dir parent.
- `bosco pick` provides a fuzzy finder to select a worktree.
- The shell hook makes `bosco <query>` behave like `cd`.

## Development

Build:
```bash
cargo build
cargo build --release
```

Format and lint:
```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
```

Test:
```bash
cargo test
cargo test <test_name>
cargo test <module_path>::<test_name>
```

## License
MIT
