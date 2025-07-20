#!/bin/sh

podman build \
  -t frederikpietzko/http-server-normal:latest \
  .

podman push frederikpietzko/http-server-normal:latest docker://docker.io/frederikpietzko/http-server-normal:latest