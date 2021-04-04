.PHONY: test

build:
	cargo build

test: build
	cargo run --bin tcp-proxy
