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
podman build \
  --platform=wasm/wasi \
  --annotation "module.wasm.image/variant=compat" \
  -t <your handle>/wasm-echo-server:latest \
```

## Inspect docker Image

```sh
podman image ls
```

In my case it is just 130kB!

`frederikpietzko/wasm-echo-server:latest                               7abe6fd6a479        130kB          130kB`

It is also possible to run this using podman like so:

```sh
podman run 
```

## Run the image in kubernetes

### Create Cluster with Wasmtime enabled nodes

```sh
cd lab/
./create-cluster.sh ClusterConfig-wasmtime.yaml
```

This will create a cluster with wasmtime enabled nodes. Under the hood they use a custom `kind` docker image that has runwasi + containerd-shim-wasmtime configured.

The containerd configuration looks like this:

```toml
[plugins."io.containerd.cri.v1.runtime".containerd.runtimes.wasm]
  runtime_type = "io.containerd.wasmtime.v1"
```

and requires the correct `runwasi containerd shim` to be installed. In our case it is `containerd-shim-wasmtime-v1` (in truth the wasmtime shim has version 0.6.0).

This enables kubernetes to run oci images with the wasm/wasi platform, as long as we apply the correct `RuntimeClass` like so:


```yaml
apiVersion: node.k8s.io/v1
kind: RuntimeClass
metadata:
  name: wasm
handler: wasm
```


Next we can create the deployment. Pay special attention to the runtimeClassName in the pod spec.

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: wasm-echo-server
spec:
  replicas: 1
  selector:
    matchLabels:
      app: wasm-echo-server
  template:
    metadata:
      labels:
        app: wasm-echo-server
    spec:
      runtimeClassName: wasm
      containers:
        - name: wasm-echo-server
          image: frederikpietzko/wasm-echo-server
          ports:
            - containerPort: 8080
```

This deployment will now work as any other kubernetes pod and will be scheduled and scaled as you would expect.

Let create a port forward and try it out!

```sh
kubectl port-forward deployment/wasm-echo-server 8080:8080
```

```sh
curl http://localhost:8080/echo -d 'Hello from WASM!'
```

