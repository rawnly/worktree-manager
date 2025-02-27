release:
	@cargo build --release
	@tar -czf worktree-manager.tar.gz --directory=./target/release worktree-manager
	@shasum -a 256 worktree-manager.tar.gz | cut -d ' ' -f1 | xargs -I{} sd '\{\{shasum\}\}' '{}' worktree-manager.rb

install:
	cargo install --path .

