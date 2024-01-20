.PHONY: help

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
	awk 'BEGIN {FS = ":.*?## "}; \
	{printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

clean:
	cargo clean

build:
	cargo build

lint:
	cargo clippy --all-targets --all-features -- -D warnings

fmt:
	@rustup component add rustfmt
	cargo fmt

doc:
	cargo doc --no-deps --open