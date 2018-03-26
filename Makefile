
release: src/main.rs
	cargo web deploy --release

test:
	cargo test --release
