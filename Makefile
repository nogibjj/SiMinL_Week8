# Makefile for Rust Project
all: check build format lint test

check:
	cargo check

build:
	cargo build

format:
	cargo fmt

lint:
	cargo clippy

test:
	cargo test

release:
	cargo build --release

#Debug mode: binary is found at target/debug/
#release mode: binary is found at target/release. Uses full optimisations.