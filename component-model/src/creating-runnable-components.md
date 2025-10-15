# Creating Runnable Components

This section contains language-specific guides on how to create runnable components.

## Running arbitrary functions from reactor (library-like) components

In practice, any interface that is exported from a WebAssembly component can be run by either:

- Creating a custom host/component that imports and reuses the functionality
- Using high level generic tooling like `wasmtime run --invoke`

For example, given a WebAssembly component which satisfies the following WIT:

```wit
{{#include ../examples/tutorial/wit/adder/world.wit}}
```

Use of the exported `add` function inside the `add` interface requires writing a host or other component that is
built to import and use that functionality. This is exemplified by the [`example-host` available in this repo][example-host].

Alternatively tooling that works generically over components `wasmtime run --invoke`:

```sh
wasmtime run --invoke 'add(1, 2)' add.component.wasm
```

Wasmtime contains code that can generically interpret exports, convert arguments to WebAssembly arguments, and execute
an existing component dynamically.

[example-host](https://github.com/bytecodealliance/component-docs/blob/main/component-model/examples/example-host/README.md)

## Creating components that behave like binaries

While running arbitrary functions require either a custom host/platform or a dynamic tool like `wasmtime run --invoke`,
components that are treatable as binaries (i.e. a CLI application) can also be built.

At a high level there are at least two ways to create components that are more like binaries than libraries
(i.e. that are easy to run from a tool like `wasmtime run`):

1. Creating a "command" component
2. Exporting the [`wasi:cli/run` interface][wasi-cli-run]

This section explores how to do the above in relevant languages.

[wasi-cli-iface-run]: https://github.com/WebAssembly/wasi-cli/tree/main/wit/run.wit

## Languages

This guide is implemented for various languages:

| Language                                                                    |
|-----------------------------------------------------------------------------|
| [Rust](./language-support/creating-runnable-components/rust.md)             |
| [Javascript](./language-support/creating-runnable-components/javascript.md) |

[docs-wit]: ./design/wit.md
