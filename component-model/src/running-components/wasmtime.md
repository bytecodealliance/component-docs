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
[Wasmtime](https://github.com/bytecodealliance/wasmtime/) is the reference implementation of the Component Model. WASI P3 runtime support is available in Wasmtime 43 and later, which implements the WASI P3 ABI (`async func`, `stream<T>`, `future<T>`). Both `wasmtime run` and `wasmtime serve` can run P3 components when the P3 ABI is enabled at the command line.

Wasmtime supports running components that implement the [`wasi:cli/command` world](https://github.com/WebAssembly/WASI/blob/main/proposals/cli/wit/command.wit), and serving components that implement either the [`wasi:http/service` world](https://github.com/WebAssembly/WASI/blob/main/proposals/http/wit/worlds.wit) or the [`wasi:http/middleware` world](https://github.com/WebAssembly/WASI/blob/main/proposals/http/wit/worlds.wit), which both imports and exports the HTTP handler interface. Wasmtime can also invoke functions exported from a component.
{{#endtab }}
{{#endtabs }}

## Running command components with Wasmtime

{{#tabs global="wasi-version" }}
{{#tab name="WASI P2" }}
To run a command component with Wasmtime, execute:

```sh
wasmtime run <path-to-wasm-file>
```

> If you are using an older version of `wasmtime`, you may need to add the `--wasm component-model` flag
> to specify that you are running a component rather than a core module.
{{#endtab }}
{{#tab name="WASI P3" }}
To run a P3 command component with Wasmtime 43 or later, enable the WASI P3 ABI:

```sh
wasmtime run -Sp3 -W component-model-async=y <path-to-wasm-file>
```

`-Sp3` enables WASI P3 imports. `-W component-model-async=y` enables the Component Model's async primitives (`async func`, `stream<T>`, `future<T>`).
{{#endtab }}
{{#endtabs }}

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
`wasmtime serve` runs P3 HTTP components that implement the [`wasi:http/service` world](https://github.com/WebAssembly/WASI/blob/main/proposals/http/wit/worlds.wit) or the [`wasi:http/middleware` world](https://github.com/WebAssembly/WASI/blob/main/proposals/http/wit/worlds.wit). Pass the same flags that enable the P3 ABI for `wasmtime run`:

```sh
wasmtime serve -Sp3 -W component-model-async=y <path-to-wasm-file>
```

If the component does not export a P3 `service` world, `wasmtime serve` falls back to the WASI P2 `wasi:http/proxy` world automatically, so the same binary serves both P2 and P3 components.

For an overview of the WASI P3 HTTP interfaces, see [WASI P3](https://wasi.dev/releases/wasi-p3) on WASI.dev.

> **Version pinning.** WASI P3 tools (wit-bindgen, Wasmtime, jco, and so on) must all target the same WIT version. WASI 0.3.0 is the stable target; some toolchains still ship the `0.3.0-rc-2026-03-15` WIT pending a refresh against the final tag. Mismatched pins surface as confusing `wrong type` errors at instantiation. See [wit-bindgen #1554](https://github.com/bytecodealliance/wit-bindgen/issues/1554) for tracking.
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
