# How to build applications using Wasmtime and deploying them on kubernetes

## Build the wasmtime echo Server 

```sh
cd rust/wasmtime/echo
make build
```

## Inspect the WASM Binary

```sh
file target/wasm32-wasip2/release/echo.wasm
```

## Run the WASM Binary using Wasmtime

```sh
make run
```

or 

```sh
wasmtime serve -Scli -Shttp target/wasm32-wasip2/release/echo.wasm
```

## Build a docker Image

The Dockerfile is as simple as it can be. We just need to copy the wasm binary into the image! 

```Dockerfile
FROM scratch
COPY target/wasm32-wasip2/release/echo.wasm /echo.wasm
ENTRYPOINT ["/echo.wasm"]
```

```sh
docker build --platform=wasm/wasi --annotation "module.wasm.image/variant=compat" -t <your-handle>/wasm-echo-server .
```

## Inspect docker Image

```sh
docker image ls
```

In my case it is just 130kB!

`frederikpietzko/wasm-echo-server:latest                               7abe6fd6a479        130kB          130kB`

## Run the image in kubernetes

### Create Cluster with Wasmtime enabled nodes

```sh
cd lab/
./create-cluster.sh ClusterConfig-wasmtime.yaml
```

