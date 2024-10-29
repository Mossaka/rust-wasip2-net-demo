# Rust wasip2 `std::net` demo

This is a simple demo of wasip2 wasi:sockets APIs in Rust using the `nightly` toolchain. Even though Rust 1.82.0 release has promoted the `wasm32-wasip2` target to tier 2, meaning you can install it using `rustup target add wasm32-wasip2`, the `wasi:sockets` API is still not available in the stable toolchain. This demo uses the `nightly` toolchain to demonstrate the `wasi:sockets` API.

This project implements a simple echo server that listens on `127.0.0.1:8080` and echoes back any message it receives, and a echo client that connects to the server and sends a message.

## Prerequisites

Install the wasmtime CLI:

```bash
$ curl https://wasmtime.dev/install.sh -sSf | bash

$ wasmtime -V
wasmtime 26.0.0 (c92317bcc 2024-10-22)
```

## Running the demo

```bash
# build 
$ cargo build --release

# run the server
$ wasmtime run -Scli -Sinherit-network target/wasm32-wasip2/release/echo-server.wasm
Echo server listening on 127.0.0.1:8080
```

In another terminal, run the client:

```bash
$ wasmtime run -Scli -Sinherit-network target/wasm32-wasip2/release/echo-client.wasm
Connected to the server on 127.0.0.1:8080
Enter message:
```
