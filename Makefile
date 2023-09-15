.PHONY: run
run:
	cargo run --

.PHONY: build
build:
	cargo build

.PHONY: test
test:
	cargo test

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: lint
lint:
	cargo clippy