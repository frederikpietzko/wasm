FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

USER vscode
# Install just via cargo
RUN cargo install just

# Add the wasm32-wasi target to the toolchain
RUN rustup target add wasm32-wasip1
# Install WasmEdge
RUN curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash

RUN echo ". /home/vscode/.wasmedge/env" >> $HOME/.profile
