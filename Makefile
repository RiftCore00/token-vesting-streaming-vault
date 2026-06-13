.PHONY: test build clean fmt clippy check

test:
	cargo test

build:
	cargo build --target wasm32v1-none --release

clean:
	cargo clean

fmt:
	cargo fmt

clippy:
	cargo clippy -- -D warnings

check:
	cargo check
