#!/bin/bash

docker build -t lab:latest .

docker run \
  --rm \
  --name lab \
  -it \
  --privileged \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v "$(pwd):/workspace/lab" \
  -v "$(pwd)/../rust/http_server:/workspace/http_server" \
  --network kind \
  lab:latest \
  bash