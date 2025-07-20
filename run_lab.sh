#!/bin/bash

docker run \
  --rm \
  --name lab \
  -it \
  --privileged \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v "$(pwd)/lab:/workspace/lab" \
  --network kind \
  frederikpietzko/wasm-lab:latest \
  bash