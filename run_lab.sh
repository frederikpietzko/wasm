#!/bin/bash

docker run \
  --rm \
  --name lab \
  -it \
  --privileged \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v "$(pwd)/lab:/workspace/lab" \
  -v "$(pwd)/rust:/workspace/rust" \
  -v "$(pwd)/k8s:/workspace/k8s" \
  --network kind \
  frederikpietzko/wasm-lab:latest \
  bash