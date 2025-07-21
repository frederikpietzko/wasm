FROM debian:bookworm-slim

ARG DEBIAN_FRONTEND=noninteractive

RUN set -eux && \
    apt-get update && \
    apt-get install -y --no-install-recommends \
    sudo \
    ca-certificates \
    iptables \
    net-tools \
    openssl \
    pigz \
    xz-utils \
    procps \
    git \
    curl \
    bash \
    jq \
    build-essential \
    supervisor \
    tmux \
    make gcc build-essential pkgconf libtool \
    libsystemd-dev libprotobuf-c-dev libcap-dev libseccomp-dev libyajl-dev \
    go-md2man autoconf python3 automake \
    gnupg && \
    curl -fsSL https://download.docker.com/linux/debian/gpg | gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg && \
    echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/debian bookworm stable" | tee /etc/apt/sources.list.d/docker.list > /dev/null && \
    apt-get update && \
    apt-get install -y docker-ce-cli bash-completion podman 


WORKDIR /workspace

RUN curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl"
RUN chmod +x ./kubectl && mv ./kubectl /usr/local/bin/kubectl
RUN [ $(uname -m) = x86_64 ] && curl -Lo ./kind https://kind.sigs.k8s.io/dl/v0.29.0/kind-linux-amd64
RUN [ $(uname -m) = x86_64 ] && chmod +x ./kind && mv ./kind /usr/local/bin/kind

RUN curl -fsSL -o get_helm.sh https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3
RUN chmod 700 get_helm.sh && ./get_helm.sh && rm get_helm.sh
RUN curl -fsSL -o ./k9s https://github.com/derailed/k9s/releases/download/v0.50.9/k9s_linux_amd64.deb && \
    dpkg -i ./k9s && \
    rm ./k9s

COPY .bashrc /root/.bashrc

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup target add wasm32-wasip1 

RUN curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash 
RUN git clone https://github.com/containers/crun
RUN . $HOME/.wasmedge/env && \
    cd crun && \
    ./autogen.sh && \
    ./configure --with-wasmedge && \
    make && \
    make install
RUN mv crun/crun /usr/bin/crun
COPY entrypoint.sh /usr/local/bin
RUN chmod +x /usr/local/bin/entrypoint.sh

ENTRYPOINT [ "/usr/local/bin/entrypoint.sh" ]
