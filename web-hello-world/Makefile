all: release

build:
	cargo +nightly build --target wasm32-unknown-unknown --release

release: build
	wasm-gc target/wasm32-unknown-unknown/release/web_hello_world.wasm -o public/web_hello_world.gc.wasm