# Wasm on Kubernetes Lab 

## Setup

This Lab can be run locally, but is ment to be run inside a container to keep the environment reproducable. So you'll nedd to have Docker or Podman installed.

The Lab Dockerfile can be found in `lab/Dockerfile`. It contains:

- some system utilities
- docker (but no engine) & podman clis kubectl
- helm
- k9s
- kind
- rust with wasm32-wasip1 toolchain
- wasmedge

The image is inteded to work as a docker outside of docker setup. That meens that you can start containers on your host from inside the lab container. That way we can create an manage a KinD cluster from inside the lab container.

The lab requires there a network called `kind` to be present.

You can start the lab by executing `./run_lab.sh`. It will use a prebuilt docker image (frederikpietzko/wasm-lab). 

```sh
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
```

This mount's the docker socket into the container, which is required for the DooD setup. 

### Note for windows users

I haven't tried doing this on windows. Please do me the favor of running this inside of WSL, as this should hopefully work there. :)

## Lab

This lab aims to provide a glimpse on how to develop and deploy wasm. You'll learn how to compile rust into a wasm binary, execute it with wasmedge (a standalone wasm runtime), build a wasm oci image, run that image locally and admire its size and deploy said image on kubernetes.

### Build and run WASM Binary

Have a look at the `rust/http_server` rust project.

```sh
cd rust/http_server
```

It contains a simple single-threaded hello-world http server. For socket communication it uses the wasmedge_wasi_socket crate.

You can create a release build with:
```sh
cargo build --target wasm32-wasip1 --release
```

Take a look at the created binary.

```sh
file target/wasm32-wasip1/release/http_server.wasm
```

You'll notice that this is a binary format wasm executable. But you cannot execute it on it's own, as it targets the wasm vm. (Similar to Java, JS, Erlang etc)

You can look at the web assembly text (wat) representation with:

```sh
wasm2wat target/wasm32-wasip1/release/http_server.wasm -o http_server.wat
```

Start the http server with

```sh
wasmedge target/wasm32-wasip1/release/http_server.wasm
```

and test it:

```sh
curl http://localhost:8080
```

### Create a wasm oci image

Since its annoying to build wasm images without docker desktop we'll use podman for this example.
Assuming you already built the release binary, you are ready to build a wasm image now. 

The Dockerfile is as simple as it can possibly be. Assuming the application code itself is secure this is as safe as something can possibly be.

```Dockerfile
FROM scratch
COPY target/wasm32-wasip1/release/http_server.wasm /app.wasm
ENTRYPOINT ["/app.wasm"]
```

To build this image, we need to provide podman with some extra information, to let it know, that the result image can only be run by a wasm runtime, and not by a linux container.

```sh
podman build \
  --platform=wasm/wasi \
  --annotation "module.wasm.image/variant=compat" \
  -t wasm_http_server \
  .
```

You can now run this image with podman. Note that I configured `crun` with wasm support. This does not work out of the box. If you are interested in how to do that look at the builder in `lab/Dockerfile`.

```sh
podman run --rm --annotation module.wasm.image/variant=compat-smart wasm_http_server
```

The resulting image is super small. Check out it's size

```sh
podman image ls
```

By contrast you can checkout the normal non wasm variant of this image which uses `gcr.io/distroless/cc-debian12` as a base.

```sh
podman image pull frederikpietzko/http-server-normal
podman image ls
```

### Run the image in kubernetes

To run wasm workloads in kubernetes there always needs to be a wasm runtime like wasmedge installed on the nodes.
Additionally containerd need to be aware of said runtime. There are 2 possiblities to achieve this:

1. runwasi + containerd-shim-wasmedge
2. crun configured with wasmedge

This lab uses the first approach. As the cluster is provisioned locally with kind, it's simplest to tell KinD to use a custom image with the correct configuration.

But the setup is pretty simple. Just install the correct containerd-shim, and configure containerd to use it.

The configuration ends up beaing pretty simple:

```toml
[plugins."io.containerd.cri.v1.runtime".containerd.runtimes.wasm]
  runtime_type = "io.containerd.wasmedge.v1"
```

This is pretty much exactly what this lab does and thus customizing the kidnest/node image. In a real cluster this can be achieved a little easier with [KWasm Operator](https://kwasm.sh).

To run the lab cluster just execute:

```sh
./lab/create_cluster.sh
```

This will setup a 3-node cluster with the wasm compatible kubernetes nodes as well as patch the kubeconfig so that it can be found from inside the lab environment. To cleanup the cluster you can run `./lab/delete_cluster.sh`.

```yaml
kind: Cluster
apiVersion: kind.x-k8s.io/v1alpha4
name: lab
nodes:
  - role: control-plane
    image: frederikpietzko/wasm-kind-node:latest
  - role: worker
    image: frederikpietzko/wasm-kind-node:latest
  - role: worker
    image: frederikpietzko/wasm-kind-node:latest
```

To run wasm workloads in kubernetes alongside normal containers I found it simplest to use a RuntimeClass.

```yaml
apiVersion: node.k8s.io/v1
kind: RuntimeClass
metadata:
  name: wasm
handler: wasm
```

Afterwards it is just a matter of creating a simple deployment like you normal would:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: wasi-demo
spec:
  replicas: 1
  selector:
    matchLabels:
      app: wasi-demo
  template:
    metadata:
      labels:
        app: wasi-demo
    spec:
      runtimeClassName: wasm
      containers:
        - name: demo
          image: ghcr.io/containerd/runwasi/wasi-demo-app:latest
```

Deploy this into the cluster and checkout it's logs. Note the `runtimeClassName: wasm` in the spec.

The deployment for the http server could look like this:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: http-server
spec:
  replicas: 1
  selector:
    matchLabels:
      app: http-server
  template:
    metadata:
      labels:
        app: http-server
    spec:
      runtimeClassName: wasm
      containers:
        - name: demo
          image: frederikpietzko/http-server:latest
          ports:
            - containerPort: 8080
              protocol: TCP
```

Next up create a portforward to `localhost:8080` and try curling the endpoint. You should receive an response, as well observe some logs in the demo container.

Feel free to experiment and try scaling this deployment to 100. If you wish you can contrast this by deploying the linux container alongside it. 

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: http-server-normal
spec:
  replicas: 1
  selector:
    matchLabels:
      app: http-server-normal
  template:
    metadata:
      labels:
        app: http-server-normal
    spec:
      containers:
        - name: demo
          image: frederikpietzko/http-server-normal:latest
          ports:
            - containerPort: 8080
              protocol: TCP
```

Thank you for trying out this lab. If you have any improvement, ideas, experiments etc feel free to talk to me or create an issue.
