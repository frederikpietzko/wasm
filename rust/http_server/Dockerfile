FROM scratch
COPY target/wasm32-wasip1/debug/http_server.wasm /app.wasm
ENTRYPOINT ["/app.wasm"]