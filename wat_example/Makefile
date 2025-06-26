run_main: add.wasm
	wasmtime --invoke main build/add.wasm
run_echo: add.wasm
	wasmtime --invoke echo build/add.wasm
add.wasm:
	wat2wasm add.wat -o build/add.wasm
decomp_add.wat: add.wasm
	wasm2wat build/add.wasm -o build/decomp_add.wat
init:
	mkdir build
