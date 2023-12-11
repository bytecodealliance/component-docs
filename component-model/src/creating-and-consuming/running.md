# Running Components

You can "run" a component by calling one of its exports. In some cases, this requires a custom host. For "command" components, though, you can use the `wasmtime` command line. This can be a convenient tool for testing components and exploring the component model. Other runtimes are also available - see the "Runtimes" section of the sidebar for more info.

> A "command" component is one that exports the `wasi:cli/run` interface, and imports only interfaces listed in the [`wasi:cli/command` world](https://github.com/WebAssembly/wasi-cli/blob/main/wit/command.wit).

You must use a recent version of `wasmtime` ([`v14.0.0` or greater](https://github.com/bytecodealliance/wasmtime/releases)), as earlier releases of the `wasmtime` command line do not include component model support.

> If you build the `wasmtime` CLI from source, you must pass `--features component-model` to the build command.

To run your component, run:

```sh
wasmtime run --wasm component-model <path-to-wasm-file>
```

## Running components with custom exports

If you're writing a library-style component - that is, one that exports a custom API - then you can run it in `wasmtime` by writing a "command" component that imports and invokes your custom API. By [composing](./composing.md) the command and the library, you can exercise the library in `wasmtime`.

1. Write your library component. The component's world (`.wit` file) must export an interface. (Do not export functions directly, only interfaces.) See the [language support guide](../language-support.md) for how to implement an export.

2. Build your library component to a `.wasm` file.

3. Write your command component. The component's world (`.wit` file) must import the interface exported from the library. Write the command to call the library's API. See the [language support guide](../language-support.md) for how to call an imported interface.

4. Build your command component to a `.wasm` file. You will not be able to run this in `wasmtime` yet, as its imports are not yet satisfied.

5. Compose your command component with your library component by running `wasm-tools compose <path/to/command.wasm> -d <path/to/library.wasm> -o main.wasm`.

6. Run the composed component using `wasmtime run --wasm component-model main.wasm`

See [Composing Components](./composing.md) for more details.
