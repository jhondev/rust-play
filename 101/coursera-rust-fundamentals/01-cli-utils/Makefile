.PHONY: help

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
	awk 'BEGIN {FS = ":.*?## "}; \
	{printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

clean: ## Clean all build files
	cargo clean

build: ## Build the project
	cargo build

lint: ## Lint code using clippy
	@rustup component add clippy
	cargo clippy --all-targets --all-features -- -D warnings

fmt: ## Format code using rustfmt
	@rustup component add rustfmt
	cargo fmt

doc: ## Generate documentation
	cargo doc --no-deps --open

bump: ## Bump version
	@cargo install cargo-bump
	@cargo bump patch