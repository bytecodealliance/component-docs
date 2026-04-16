# Rust

## 1. Setup

Add the `wasm32-wasip2` target to the Rust toolchain.

```rust
rustup target add wasm32-wasip2
```


Install [`wasmtime`][wasmtime]. The Wasmtime CLI has a built-in HTTP server that supports serving WebAssembly HTTP components.

```console
curl https://wasmtime.dev/install.sh -sSf | bash
```

[wasmtime]: https://github.com/bytecodealliance/wasmtime#installation

## 2. Creating a Rust WebAssemly project

Create a new Rust project with `cargo new`:


```console
cargo new wasm-http-hello-world
cd wasm-http-hello-world
```


Add [`wstd`][wstd], a Rust async standard library for Wasm components as a dependency with `cargo add`:
```console
cargo add wstd
```
`wstd` provides idiomatic Rust bindings for WASI standard interfaces [(`wasi:http`)](https://github.com/WebAssembly/WASI/tree/main/proposals/http) to increase ease-of-use for Rust WebAssembly components. Since we are using `wstd`, we will not need to add WIT files or depend on [`wit-bindgen`](https://crates.io/crates/wit-bindgen) directly.


> [!NOTE]

> It is possible to build an HTTP component in Rust without `wstd`. Building a HTTP component without `wstd` would require defining the [`wasi:http`](https://github.com/WebAssembly/WASI/tree/main/proposals/http) imports/exports of the component in WIT, fetching  WIT dependencies with `wkg` and generating the Rust bindings with `wit-bindgen`.
>
> Both approaches are valid, but `wstd` offers superior developer experience, so we opt to use it here.
>
> `wstd` and `wit-bindgen` are not mutually exclusive and can co-exist in the same project.

[wstd]: https://docs.rs/wstd/latest/wstd/index.html

## 3. Writing the HTTP handler

We will implement the HTTP handler in `src/main.rs`. The file should look like the following:
```rust
use wstd::http::{Body, Request, Response, Result, StatusCode};

// WASI HTTP server components don't use a traditional `main` function.
// They export a function named `handle` which takes a `Request`
// argument, and which may be called multiple times on the same
// instance. To let users write a familiar `fn main` in a file
// named src/main.rs, wstd provides this `wstd::http_server` macro, which
// transforms the user's `fn main` into the appropriate `handle` function.
#[wstd::http_server]
async fn main(req: Request<Body>) -> Result<Response<Body>> {
    match req.uri().path() {
        "/" => home(req).await,
        _ => not_found(req).await,
    }
}

async fn home(_req: Request<Body>) -> Result<Response<Body>> {
    Ok(Response::new("Hello, world!\n".into()))
}

async fn not_found(_req: Request<Body>) -> Result<Response<Body>> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(().into())
        .expect("builder succeeds"))
}
```

## 4. Compiling and running the component


Build the component:


```console
cargo build --release --target wasm32-wasip2
```


The `.wasm` binary for the component can be found at `target/wasm32-wasip2/release/wasm-http-hello-world.wasm`.


To run the component, we can use [`wasmtime`](https://github.com/bytecodealliance/wasmtime/), a reference implementation host that supports the Component Model.

In particular, we can use `wasmtime serve` subcommand, which will spin-up an HTTP server at `http://localhost:8080` which will use our component to fulfill web requests. `wasmtime` creates a *fresh* instance of the component every time a request is served.


```console
wasmtime serve -Scli -Shttp target/wasm32-wasip2/release/wasm-http-hello-world.wasm
```


You can test it with `curl -i localhost:8080`


```console
HTTP/1.1 200 OK
transfer-encoding: chunked
date: Mon, 13 Apr 2026 23:22:20 GMT

Hello, world!
```

With this, we have successfully built and run a basic WebAssembly HTTP component with Rust 🎉

## 5. Going further

Explore more examples of projects that use `wstd`:

- [An example `wasi:http` server component](https://github.com/bytecodealliance/sample-wasi-http-rust)
- [Various examples of using wstd](https://github.com/bytecodealliance/wstd/tree/main/examples)
- [Examples of using wstd with Axum](https://github.com/bytecodealliance/wstd/tree/main/axum/examples)
