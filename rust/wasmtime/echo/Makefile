build:
	cargo build --release --target wasm32-wasip2

run: build
	wasmtime serve -Scli -Shttp target/wasm32-wasip2/release/echo.wasm

