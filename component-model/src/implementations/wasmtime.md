# Wasmtime

[Wasmtime](https://github.com/bytecodealliance/wasmtime/) is the reference implementation of the Component Model. It supports the [`wasi:cli/command` world](https://github.com/WebAssembly/wasi-cli/blob/main/wit/command.wit).

> At the time of writing, component support is not yet included in a numbered release. It is expected to be included in Wasmtime 13. In the meantime, use the [dev release](https://github.com/bytecodealliance/wasmtime/releases/tag/dev).

To run a component with wasmtime, run:

```sh
wasmtime run --wasm-features component-model <path-to-wasm-file>
```

By default, Wasmtime denies the component access to all system resources. For example, the component cannot access the file system or environment variables. See the [Wasmtime guide](https://docs.wasmtime.dev/) for information on granting access, and for other Wasmtime features.
