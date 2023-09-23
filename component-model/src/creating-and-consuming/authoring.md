# Authoring Components

You can write WebAssembly core modules in a wide variety of languages, and the set of languages that can directly create components is growing. See the [Language Support](../language-support.md) section for information on building components directly from source code.

If your preferred language supports WebAssembly but not components, you can still create components using the [`wasm-tools component`](https://github.com/bytecodealliance/wasm-tools/tree/main/crates/wit-component) tool.  (A future version of this page will cover this in more detail.)

## Command and Reactor Components

There are two categories of components commonly referred to by component model tooling: reactor and command components. Generally, reactor components can be thought of a libraries consumed by components or hosts, while command components can be see as `bin` components that have a `main` function (called `run` in WASI).

Specifically, a reactor component is one that imports all the interfaces in the [`wasi:cli/reactor`](https://github.com/WebAssembly/wasi-cli/blob/main/wit/reactor.wit) world. A `command` component is a superset of a `reactor` component. It is a component that imports all of the `reactor` interfaces and exports the `wasi:cli/run` interface. It is defined by the [`wasi:cli/command` world](https://github.com/WebAssembly/wasi-cli/blob/main/wit/**command**.wit).