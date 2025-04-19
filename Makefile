build:
	stellar contract build;
	stellar contract optimize --wasm target/wasm32-unknown-unknown/release/router.wasm;