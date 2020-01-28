test:
	cargo test --verbose
.PHONY: test

doc:
	rustdoc -o docs src/lib.rs
.PHONY: doc

build:
	cargo build --verbose
.PHONY: build

clean:
	cargo clean
.PHONY: clean

example:
	cargo run --example colors
.PHONY: example
