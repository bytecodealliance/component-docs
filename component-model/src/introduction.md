# Home

The WebAssembly Component Model is a broad-reaching architecture for building interoperable WebAssembly libraries, applications, and environments.

This documentation is aimed at _users_ of the component model: developers of libraries and applications.

> [!NOTE]
>
> _Compiler and Wasm runtime developers_ can take a look at the [Component Model specification](https://github.com/WebAssembly/component-model) to
> see how to add support for the component model to their project.

## Table of contents

| Understanding components | Building components  | Using components  |
|--------------------------|----------------------|-------------------|
| [Why Components?]        | [C/C++]              | [Composing]       |
| [Components]             | [C#]                 | [Running]         |
| [Interfaces]             | [Go]                 | [Distributing]    |
| [Worlds]                 | [JavaScript]         |                   |
|                          | [Python]             |                   |
|                          | [Rust]               |                   |

[Why Components?]: ./design/why-component-model.md
[Components]: ./design/components.md
[Interfaces]: ./design/interfaces.md
[Worlds]: ./design/worlds.md

[C/C++]: ./language-support/c.md
[C#]: ./language-support/csharp.md
[Go]: ./language-support/go.md
[JavaScript]: ./language-support/javascript.md
[Python]: ./language-support/python.md
[Rust]: ./language-support/rust.md

[Composing]: ./composing-and-distributing/composing.md
[Running]: ./running-components.md
[Distributing]: ./composing-and-distributing/distributing.md


## APIs for building WebAssembly components

It's useful to have a standard, shared set of APIs
that WebAssembly components can depend on.
[WASI](https://wasi.dev/) (the WebAssembly System Interface) is a standards-track specification
that defines these APIs.
A system or platform may expose some or all of the WASI APIs to components.

### Status

The current stable release of WASI is [WASI 0.2.0](https://github.com/WebAssembly/WASI/pull/577),
which was released on January 25, 2024.
WASI 0.2.0 is [a stable set of WIT definitions](https://github.com/WebAssembly/WASI/tree/main/wasip2)
that components can target.
WASI proposals will continue to evolve and new ones will be introduced;
however, users of the component model can now pin to any stable release >= `v0.2.0`.
The [WASI.dev roadmap](https://wasi.dev/roadmap) tracks upcoming releases.

## Contributing

If you find a mistake, omission, ambiguity, or other problem, please let us know via [GitHub issues](https://github.com/bytecodealliance/component-docs/issues).

If you'd like to contribute content to the guide, please see the [contribution guide](https://github.com/bytecodealliance/component-docs/blob/main/CONTRIBUTING.md) for information on how to contribute.

[!NOTE]: #
