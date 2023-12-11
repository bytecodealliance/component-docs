# Wasmtime

[Wasmtime](https://github.com/bytecodealliance/wasmtime/) is the reference implementation of the Component Model. It supports running components that implement the [`wasi:cli/command` world](https://github.com/WebAssembly/wasi-cli/blob/main/wit/command.wit) and serving components the implement the [`wasi:http/proxy` world](https://github.com/WebAssembly/wasi-http/blob/main/wit/proxy.wit).

## Running command components with Wasmtime
To run a command component with wasmtime, execute:

```sh
wasmtime run --wasm component-model <path-to-wasm-file>
```

By default, Wasmtime denies the component access to all system resources. For example, the component cannot access the file system or environment variables. See the [Wasmtime guide](https://docs.wasmtime.dev/) for information on granting access, and for other Wasmtime features.

## Running HTTP components with Wasmtime

You can now execute components that implement the [HTTP proxy world](https://github.com/WebAssembly/wasi-http/blob/main/wit/proxy.wit) with the `wasmtime serve` subcommand. [The Wasmtime CLI](https://github.com/bytecodealliance/wasmtime) supports serving these components as of `v14.0.3`. 

To run a HTTP component with Wasmtime, execute:
```sh
wasmtime serve <path-to-wasm-file>
```

Try out building and running HTTP components with one of these tutorials

1. [Hello WASI HTTP tutorial](https://github.com/sunfishcode/hello-wasi-http) - build and serve a simple Rust-based HTTP component

2. [HTTP Auth Middleware tutorial](https://github.com/fermyon/http-auth-middleware#running-with-wasmtime) - compose a HTTP authentication middleware component with a business logic component
