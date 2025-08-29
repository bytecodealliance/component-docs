# Creating components

Many popular programming languages can be compiled to WebAssembly,
but the level of support varies across languages.
This document details languages with compilers and runtimes
that support WebAssembly with WASI as a target platform.

> This is a living document, so if you are aware of advancements in a toolchain, please do
not hesitate to [contribute documentation](https://github.com/bytecodealliance/component-docs/blob/main/CONTRIBUTING.md). You can find more information about the development of support for specific languages in the [Guest Languages Special Interest Group Proposal](https://github.com/bytecodealliance/governance/blob/main/SIGs/SIG-guest-languages/proposal.md) document.

One of the benefits of components is their portability across host runtimes. The runtime only needs
to know what world the component is targeting in order to import or execute the component. This
language guide hopes to demonstrate that with a prevailing `adder` world defined in
[`examples/tutorial/wit/adder/world.wit`](https://github.com/bytecodealliance/component-docs/blob/main/component-model/examples/tutorial/wit/adder/world.wit). Furthermore, an example host that understands the `example`
world has been provided in [`examples/example-host`](https://github.com/bytecodealliance/component-docs/blob/main/component-model/examples/example-host/README.md) for running components. Each
toolchain section walks through creating a component of this world, which can be run either in the
example host or from an application of that toolchain. This aims to provide a full story for using
components within and among toolchains.

Each section covers how to build and
run components for a given toolchain.
The last section, on WebAssembly Text Format (WAT),
details how to write WebAssembly components by hand,
without using a higher-level language front-end.

  - [C/C++ Tooling](./language-support/c.md)
    - [Building a Component with `wit-bindgen` and `wasm-tools`](./language-support/c.md#building-a-component-with-wit-bindgen-and-wasm-tools)
    - [Running a Component from C/C++ Applications](./language-support/c.md#running-a-component-from-cc-applications)
  - [C# Tooling](./language-support/csharp.md)
  - [Go Tooling](./language-support/go.md)
  - [JavaScript Tooling](./language-support/javascript.md)
    - [Building a Component with `jco`](./language-support/javascript.md#building-a-component-with-jco)
    - [Running a Component from JavaScript Applications](./language-support/javascript.md#running-a-component-from-javascript-applications)
  - [Python Tooling](./language-support/python.md)
    - [Building a Component with `componentize-py`](./language-support/python.md#building-a-component-with-componentize-py)
    - [Running components from Python Applications](./language-support/python.md#running-components-from-python-applications)
  - [Rust Tooling](./language-support/rust.md)
    - [Building a Component](./language-support/rust.md#building-a-component)
    - [Running a Component from Rust Applications](./language-support/rust.md#running-a-component-from-rust-appliacations)
  - [MoonBit Tooling](./language-support/moonbit.md)
  - [WebAssembly Text Format (WAT)](./language-support/wat.md#wat-webassembly-text-format)
    - [Building a Component from WAT with `wasm-tools`](./language-support/wat.md#building-a-component-with-wasm-tools)
    - [Running a Component with Wasmtime](./language-support/wat.md#running-a-component-with-wasmtime)
  - [Other Languages with Component Model Support](./language-support/other-languages.md)
