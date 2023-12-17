# Wasm Language Support

WebAssembly can be targeted by the majority of top programming
languages; however, the level of
support varies. This document details the subset of languages that target WASI and support
components. This is a living document, so if you are aware of advancements in a toolchain, please do
not hesitate to [contribute documentation](https://github.com/bytecodealliance/component-docs/blob/main/CONTRIBUTING.md). You can find more information about the development of support for specific languages [here](https://github.com/bytecodealliance/governance/blob/main/SIGs/SIG-guest-languages/proposal.md). Each section covers how to build and
run components for a given toolchain.

One of the benefits of components is their portability across host runtimes. The runtime only needs
to know what world the component is targeting in order to import or execute the component. This
language guide hopes to demonstrate that with a prevailing `example` world defined in
[`examples/example-host/add.wit`](https://github.com/bytecodealliance/component-docs/blob/main/component-model/examples/example-host/add.wit). Furthermore, an example host that understands the `example`
world has been provided in [`examples/example-host`](https://github.com/bytecodealliance/component-docs/blob/main/component-model/examples/example-host/README.md) for running components. Each
toolchain section walks through creating a component of this world, which can be run either in the
example host or from an application of that toolchain. This aims to provide a full story for using
components within and among toolchains.

- [Wasm Language Support](#wasm-language-support)
  - [Language Agnostic Tooling](#language-agnostic-tooling)
    - [Building a Component with `wasm-tools`](#building-a-component-with-wasm-tools)
    - [Running a Component with Wasmtime](#running-a-component-with-wasmtime)
    - [Serving a HTTP Component with Wasmtime](#serving-a-http-component-with-wasmtime)
  - [Rust Tooling](./language-support/rust.md)
    - [Building a Component with `cargo component`](./language-support/rust.md#building-a-component-with-cargo-component)
    - [Running a Component from Rust Applications](./language-support/rust.md#running-a-component-from-rust-appliacations)
  - [JavaScript Tooling](./language-support/javascript.md)
    - [Building a Component with `jco`](./language-support/javascript.md#building-a-component-with-jco)
    - [Running a Component from JavaScript Applications](#./language-support/javascript#running-a-component-from-javascript-applications)
  - [Python Tooling](./language-support/python.md)
    - [Building a Component with `componentize-py`](./language-support/python.md#building-a-component-with-componentize-py)
    - [Running components from Python Applications](./language-support/python.md#running-components-from-python-applications)

## Language Agnostic Tooling

### Building a Component with `wasm-tools`

[`wasm-tools`](https://github.com/bytecodealliance/wasm-tools) provides a suite of subcommands for
working with WebAssembly modules and components.

`wasm-tools` can be used to create a component from WebAssembly Text (WAT). This walks through creating a component from WAT that implements the [`example` world](https://github.com/bytecodealliance/component-docs/blob/main/component-model/examples/example-host/add.wit) and simply adds two numbers.

1. Install [`wasm-tools`](https://github.com/bytecodealliance/wasm-tools/tree/main#installation), a
   tool for low-level manipulation of Wasm modules and components.
2. The `add` function is defined inside the following `example` world:

   ```wit
   package example:component;

   world example {
       export add: func(x: s32, y: s32) -> s32;
   }
   ```

3. Define an `add` core module in WAT that exports an `add` function that adds two parameters:

   ```wat
   (module
     (func $add (param $lhs i32) (param $rhs i32) (result i32)
         local.get $lhs
         local.get $rhs
         i32.add)
     (export "add" (func $add))
   )
   ```

4. Use `wasm-tools` to create a component from the core module, first embedding component metadata
   inside the core module and then encoding the WAT to a Wasm binary.

   ```sh
   $ wasm-tools component embed add.wit add.wat -o add.wasm
   $ wasm-tools component new add.wasm -o add.component.wasm
   ```

### Running a Component with Wasmtime

You can "run" a component by calling one of its exports. Hosts and runtimes often only support
running components with certain exports. The [`wasmtime`](https://github.com/bytecodealliance/wasmtime) CLI can only run "command" components, so in
order to run the `add` function above, it first must be composed with a primary "command" component
that calls it. See [documentation on running components](./creating-and-consuming/running.md) for
more details.
