.PHONY: test

build:
	cargo build

test: build
	RUST_LOG=info cargo run --bin exer3
