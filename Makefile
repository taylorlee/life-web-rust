
debug: src/main.rs
	cargo run

asm: src/main.rs
	cargo web deploy --release
