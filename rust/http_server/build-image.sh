#!/bin/sh

podman build \
  --platform=wasm/wasi \
  --annotation "module.wasm.image/variant=compat" \
  -t frederikpietzko/http-server:latest \
  .

podman push frederikpietzko/http-server:latest docker://docker.io/frederikpietzko/http-server:latest