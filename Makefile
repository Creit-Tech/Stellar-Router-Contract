build:
	stellar contract build;
	stellar contract optimize --wasm target/wasm32v1-none/release/stellar_router_v0.wasm;
	stellar contract optimize --wasm target/wasm32v1-none/release/stellar_router_v1.wasm;