.PHONY: all build test clippy fmt charon clean

all: test clippy

build:
	cargo build

test:
	cargo test

clippy:
	cargo clippy --all-targets -- -D warnings

fmt:
	cargo fmt --all

# Placeholder: extract LLBC with charon once it is wired up.
# See https://github.com/AeneasVerif/charon for installation.
charon:
	@echo "TODO: run charon to extract LLBC"
	@echo "e.g. charon --input Cargo.toml --dest-file target/llbc/trustyrustcode-core.llbc"

clean:
	cargo clean
