kind-image-wasmtime:
	docker buildx build . -f Dockerfile.kind-wasmtime --platform linux/amd64 -t frederikpietzko/wasmtime-kind-node

kind-image-wasmedge:
	docker buildx build . -f Dockerfile.kind --platform linux/amd64 -t frederikpietzko/wasm-kind-node

publish-wasmtime: kind-image-wasmtime
	docker push frederikpietzko/wasmtime-kind-node

publish-wasmedge: kind-image-wasmedge
	docker push frederikpietzko/wasm-kind-node

create-cluster:
	@if [ -z "$(config)" ]; then \
		echo "Error: config parameter required. Usage: make create-cluster config=<config-file>"; \
		exit 1; \
	fi
	kind create cluster --config $(config)
