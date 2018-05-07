all:
	cargo build --release --target wasm32-unknown-unknown
	cp ./target/wasm32-unknown-unknown/release/rs_wasm_phaser.wasm ./static/
