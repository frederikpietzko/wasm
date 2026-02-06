build:
	cargo build --release --target wasm32-wasip2

run: build
	wasmtime serve -Scli -Shttp target/wasm32-wasip2/release/echo.wasm

publish: build
	docker build --platform=wasm/wasi --annotation "module.wasm.image/variant=compat" -t frederikpietzko/wasm-echo-server .
	docker push frederikpietzko/wasm-echo-server
