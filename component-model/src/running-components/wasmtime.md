# Wasmtime

<div class="version-notice">

This page has content for both **WASI P2** and **WASI P3**. Use the tabs below to switch between versions where they differ.

</div>

{{#tabs global="wasi-version" }}
{{#tab name="WASI P2" }}
[Wasmtime](https://github.com/bytecodealliance/wasmtime/) is the reference implementation of the Component Model.
It supports running components that implement the [`wasi:cli/command` world](https://github.com/WebAssembly/WASI/blob/main/proposals/cli/wit/command.wit)
and serving components that implement the [`wasi:http/proxy` world](https://github.com/WebAssembly/WASI/blob/main/proposals/http/wit/proxy.wit).
Wasmtime can also invoke functions exported from a component.
{{#endtab }}
{{#tab name="WASI P3" }}
[Wasmtime](https://github.com/bytecodealliance/wasmtime/) is the reference implementation of the Component Model. WASI P3 runtime support is available in Wasmtime 43 and later, which implements the WASI 0.3 ABI (`async func`, `stream<T>`, `future<T>`).

It supports running components that implement the [`wasi:cli/command` world](https://github.com/WebAssembly/WASI/blob/main/proposals/cli/wit-0.3.0-draft/command.wit). Support for serving components in the [`wasi:http/service` world](https://github.com/WebAssembly/WASI/blob/main/proposals/http/wit-0.3.0-draft/worlds.wit) and the new [`wasi:http/middleware` world](https://github.com/WebAssembly/WASI/blob/main/proposals/http/wit-0.3.0-draft/worlds.wit), which both imports and exports the HTTP handler interface, is in development. See [the tracking issue](https://github.com/bytecodealliance/wit-bindgen/issues/1554) for current status.
Wasmtime can also invoke functions exported from a component.
{{#endtab }}
{{#endtabs }}

## Running command components with Wasmtime
To run a command component with Wasmtime, execute:

```sh
wasmtime run <path-to-wasm-file>
```

> If you are using an older version of `wasmtime`, you may need to add the `--wasm component-model` flag
> to specify that you are running a component rather than a core module.

By default, Wasmtime denies the component access to all system resources.
For example, the component cannot access the file system or environment variables.
See the [Wasmtime guide](https://docs.wasmtime.dev/) for information on granting access, and for other Wasmtime features.

## Running HTTP components with Wasmtime

{{#tabs global="wasi-version" }}
{{#tab name="WASI P2" }}
You can execute components that implement the [HTTP proxy world](https://github.com/WebAssembly/WASI/blob/main/proposals/http/wit/proxy.wit) with the `wasmtime serve` subcommand.
[The Wasmtime CLI](https://github.com/bytecodealliance/wasmtime) supports serving these components as of `v18.0.0`.

To run a HTTP component with Wasmtime, execute:
```sh
wasmtime serve <path-to-wasm-file>
```

Try out building and running HTTP components with one of these tutorials

1. [Hello WASI HTTP tutorial](https://github.com/sunfishcode/hello-wasi-http) - build and serve a simple Rust-based HTTP component

2. [HTTP Auth Middleware tutorial](https://github.com/fermyon/http-auth-middleware#running-with-wasmtime) - compose a HTTP authentication middleware component with a business logic component
{{#endtab }}
{{#tab name="WASI P3" }}
Wasmtime 43 and later provide runtime support for the WASI P3 ABI. The `wasmtime serve` subcommand currently targets the [`wasi:http/proxy` world](https://github.com/WebAssembly/WASI/blob/main/proposals/http/wit/proxy.wit) from WASI P2; support for serving components in the WASI P3 [`wasi:http/service`](https://github.com/WebAssembly/WASI/blob/main/proposals/http/wit-0.3.0-draft/worlds.wit) and [`wasi:http/middleware`](https://github.com/WebAssembly/WASI/blob/main/proposals/http/wit-0.3.0-draft/worlds.wit) worlds is in progress. See [the tracking issue](https://github.com/bytecodealliance/wit-bindgen/issues/1554) for current status.

For an overview of the WASI P3 HTTP interfaces, see [WASI 0.3](https://wasi.dev/releases/wasi-p3) on WASI.dev.
{{#endtab }}
{{#endtabs }}

## Running components with custom exports

As of Wasmtime Version 33.0.0, there is [support for invoking components with custom exports](https://bytecodealliance.org/articles/invoking-component-functions-in-wasmtime-cli).


As an example, if your component exports a function `add` which takes two numeric arguments, you can make use of this feature with the following command.

```sh
wasmtime run --invoke 'add(1, 2)' <path-to-wasm-file>
```

Make sure to wrap your invocation in single quotes and to include parentheses, even if your function doesn't take any arguments.
For a full list of ways to represent the various WIT types when passing arguments to your exported function,
visit the [WAVE repository](https://github.com/bytecodealliance/wasm-tools/tree/main/crates/wasm-wave).
