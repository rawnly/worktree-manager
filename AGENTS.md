# Agent Guide for bosco

This file tells autonomous coding agents how to work in this repo.
It is written for Rust contributors and automation.

## Project overview
- Language: Rust (edition 2021)
- Binary: `bosco` (main entry `src/main.rs`)
- Purpose: worktree manager CLI

## Environment setup
- Tooling is tracked with `mise.toml`.
- Recommended: `mise install` to install Rust and goreleaser.

## Build, lint, and test

### Build
- `cargo build` (debug build)
- `cargo build --release` (release build)
- `make release` (release build + tarball + checksum)
- `mise run build` (release build; see `mise.toml`)

### Lint / format
- `cargo fmt` (format code)
- `cargo fmt -- --check` (format check in CI)
- `cargo clippy --all-targets --all-features -- -D warnings`

### Tests
- `cargo test` (all tests)
- `cargo test <test_name>` (single test by name)
- `cargo test <module_path>::<test_name>` (single test in module)
- `cargo test -- --nocapture` (show stdout/stderr)

Note: there are no explicit custom test tasks in `Makefile` or `mise.toml`.

## Source layout
- `src/main.rs`: CLI entry and command routing.
- `src/commands/*`: subcommands (`add`, `init`, `list`, `pick`, `remove`).
- `src/git.rs`: worktree operations and git interaction.
- `src/shell.rs`: shell detection and hook generation.
- `src/version_check/*`: GitHub release check.
- `src/utils.rs`: shared helpers and macros.

## Coding conventions

### General Rust style
- Follow `rustfmt` defaults; do not hand-format.
- Keep functions small and single-purpose.
- Prefer early returns for error cases.
- Use `anyhow::Result<T>` for fallible public functions.
- Avoid `panic!` unless the failure is unrecoverable and justified.

### Imports
- Prefer explicit imports over glob imports (current code uses explicit imports).
- Group imports by origin: std, external crates, internal modules.
- Keep `use` blocks near the top of the file.
- Re-export functions in `commands/mod.rs` to keep main dispatch clean.

### Error handling
- Use `?` to propagate errors.
- When converting errors, keep context: `anyhow::anyhow!(...)`.
- For user-facing failures, use `eprintln!` and a clean message.
- Do not swallow errors silently unless a best-effort operation.

### Naming
- Modules: snake_case filenames and module names.
- Types: PascalCase (`Worktree`).
- Functions and variables: snake_case.
- CLI flags: kebab-case via clap attributes.

### CLI behavior
- Commands are defined in `src/commands/mod.rs` using clap derive.
- Keep command output human-friendly by default; JSON is opt-in.
- Avoid breaking existing command names/aliases.

### Async
- Only the top-level entry is async (`#[tokio::main]`).
- Avoid spawning tasks unless necessary (currently used for version check).

### Formatting and output
- Use `println!` for normal output, `eprintln!` for warnings/errors.
- For rich terminal output, use `termimad` and `colored` as in existing code.
- Keep output stable for CLI users and scripts.

### Git and shell interactions
- Use `shell::execute` for external command calls.
- Ensure arguments are explicit and avoid shell interpolation.
- Prefer `git2` for repo metadata, but CLI git commands are used for worktrees.

### JSON output
- Only emit JSON when the `--json` flag is provided.
- Use `serde_json::to_string` on `serde::Serialize` types.

## Contribution guidelines
- Keep changes small and focused.
- Update tests if behavior changes.
- Update README only when user-facing behavior or options change.
- Avoid introducing new dependencies unless necessary.

## Existing automation
- `Makefile` targets: `release`, `install`.
- `mise` tasks: `install`, `build`, `check`, `release`, `snapshot`.

## Repository-specific notes
- `worktree-manager.rb` is used for the Homebrew formula template.
- `build.rs` exists; check it if adding build-time logic.

## If you add tests
- Prefer unit tests near the module they validate.
- Use descriptive test names; prefer `it_*` or `returns_*` naming.

## If you add new commands
- Add to `Command` enum and expose `exec` in `commands/mod.rs`.
- Update shell hook command list generation in `shell.rs` (it uses enum list).
- Add help text and aliases consistent with existing commands.

## Cursor or Copilot rules
- No `.cursor/rules/`, `.cursorrules`, or `.github/copilot-instructions.md` found.
- If these appear later, mirror their instructions here.
