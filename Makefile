build:
	stellar contract build;
	stellar contract optimize --wasm target/wasm32-unknown-unknown/release/stellar_router.wasm;