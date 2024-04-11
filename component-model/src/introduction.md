# Home

The WebAssembly Component Model is a broad-reaching architecture for building interoperable Wasm libraries, applications, and environments.

| Understanding components | Building components  | Using components  |
|--------------------------|----------------------|-------------------|
| [Why Components?]        | [Javascript]         | [Composing]       |
| [Components]             | [Python]             | [Running]         |
| [Interfaces]             | [Rust]               | [Distributing]    |
| [Worlds]                 | [Go]                 |                   |

[Why Components?]: ./design/why-component-model.md
[Components]: ./design/components.md
[Interfaces]: ./design/interfaces.md
[Worlds]: ./design/worlds.md

[Javascript]: ./language-support/javascript.md
[Python]: ./language-support/python.md
[Rust]: ./language-support/rust.md
[Go]: ./language-support/go.md

[Composing]: ./creating-and-consuming/composing.md
[Running]: ./creating-and-consuming/running.md
[Distributing]: ./creating-and-consuming/distributing.md

> ⓘ This documentation is aimed at _users_ of the component model: developers of libraries and applications. _Compiler and Wasm runtime developers_ can take a look at the [Component Model specification](https://github.com/WebAssembly/component-model) to see how to add support for the component model to their project.

## Status

[WASI 0.2.0 was released](https://github.com/WebAssembly/WASI/pull/577) Jan 25, 2024, providing a stable release of WASI and the component model. This [is a stable set of WIT definitions](https://github.com/WebAssembly/WASI/tree/main/preview2) that components can target. WASI proposals will continue to evolve and new ones will be introduced; however, users of the component model can now pin to the stable 0.2.0 release.

## Contributing

If you find a mistake, omission, ambiguity, or other problem, please let us know via [GitHub issues](https://github.com/bytecodealliance/component-docs/issues).

If you'd like to contribute content to the guide, please see the [contribution guide](https://github.com/bytecodealliance/component-docs/blob/main/CONTRIBUTING.md) for information on how to contribute.
