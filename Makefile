release:
	cargo build --release
	tar -czf worktree-manager.tar.gz --directory=./target/release worktree-manager
	SHASUM=$$(shasum -a 256 worktree-manager.tar.gz | cut -d ' ' -f 1) ; \
	sd '\{\{shasum\}\}' "$$SHASUM" worktree-manager.rb

install:
	cargo install --path .

