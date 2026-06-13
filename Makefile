.PHONY: all build test fmt fmt-check clippy clean

all: fmt-check clippy test build

build:
	cargo build --target wasm32v1-none --release

test:
	cargo test

fmt:
	cargo fmt

fmt-check:
	cargo fmt --check

clippy:
	cargo clippy -- -D warnings

clean:
	cargo clean
