add.wasm:
	wat2wasm add.wat -o build/add.wasm
decomp_add.wat: add.wasm
	wasm2wat build/add.wasm -o build/decomp_add.wat
init:
	mkdir build
