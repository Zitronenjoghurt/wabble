.PHONY: data

data:
	cargo run --release --package wabble-data --bin generate --features="parse"