#!/bin/sh

podman build \
  --platform=wasm/wasi \
  --annotation "module.wasm.image/variant=compat" \
  -t frederikpietzko/wasm-http-server:latest \
  .

podman push frederikpietzko/wasm-http-server:latest docker://docker.io/frederikpietzko/wasm-http-server:latest