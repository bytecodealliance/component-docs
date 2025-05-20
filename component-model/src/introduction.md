# Home

The WebAssembly Component Model is a broad-reaching architecture for building interoperable WebAssembly libraries, applications, and environments.

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

[Composing]: ./creating-and-consuming/composing.md
[Running]: ./creating-and-consuming/running.md
[Distributing]: ./creating-and-consuming/distributing.md

> [!NOTE]
>This documentation is aimed at _users_ of the component model: developers of libraries and applications.
>
> _Compiler and Wasm runtime developers_ can take a look at the [Component Model specification](https://github.com/WebAssembly/component-model) to
> see how to add support for the component model to their project.

## A quick overview of core concepts

This section introduces the core concepts and [rationale](./design/why-component-model.md) of the component model.

* A [WebAssembly Component](./design/components.md) is the next evolution of core WebAssembly binaries.
  * WebAssembly components are *nestable* -- they may contain one or more core modules and/or sub-components composed together.
* The Component Model extends core WebAssembly by introducing higher level types and interface-driven development
  * [WebAssembly Interface Types (WIT)][wit] is the [IDL (Interface Definition Language)][wiki-idl] used to formally define functionality for WebAssembly modules.
  * With WIT, WebAssembly components gain the ability to conform an language-agnostic and encode that support, so any WebAssembly component binary can be interrogated *and* executed.
  * An [Interface](./design/interfaces.md) describes the types and functions used for a specific, focused bit of functionality.
  * A [World](./design/worlds.md) assembles interfaces to express what features a component offers, and what features it depends on.
  * A [Package](./design/packages.md) is a set of WIT files containing a related set of interfaces and worlds.
* The Component Model introduces the idea of a "platform" to core WebAssembly -- enabling the structured, standardized use of "host" functionality for WebAssembly "guest"s.
  * The WebAssembly System Interface (WASI) defines in WIT a family of interfaces for common system-level functions.
  * WASI defines common execution environments such as the command line (`wasi:cli`) or a HTTP server (`wasi:http`).
* The Component Model introducs makes core WebAssembly composable -- components that provide functionality and those that use them can be composed together into *one* resulting component

> [!NOTE]
> The Component Model is stewarded by the Bytecode Alliance and designed [in the open][cm-repo].
>
> See the [`WebAssembly/component-model`][cm-repo] repository for [Goals][goals],[use cases][use-cases], and [high level design choices][design-choices].

[cm-repo]: https://github.com/WebAssembly/component-model
[wiki-idl]: https://en.wikipedia.org/wiki/Web_IDL
[goals]: https://github.com/WebAssembly/component-model/blob/main/design/high-level/Goals.md
[use-cases]: https://github.com/WebAssembly/component-model/blob/main/design/high-level/UseCases.md
[design-choices]: https://github.com/WebAssembly/component-model/blob/main/design/high-level/Choices.md
[wit]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md

## Status

[WASI 0.2.0 was released](https://github.com/WebAssembly/WASI/pull/577) Jan 25, 2024, providing a stable release of WASI and the component model.
This [is a stable set of WIT definitions](https://github.com/WebAssembly/WASI/tree/main/wasip2) that components can target. WASI proposals will
continue to evolve and new ones will be introduced; however, users of the component model can now pin to any stable release >= 0.2.0. See WASI.dev to stay up to date on the latest releases.

## Contributing

If you find a mistake, omission, ambiguity, or other problem, please let us know via [GitHub issues](https://github.com/bytecodealliance/component-docs/issues).

If you'd like to contribute content to the guide, please see the [contribution guide](https://github.com/bytecodealliance/component-docs/blob/main/CONTRIBUTING.md) for information on how to contribute.

[!NOTE]: #
[!WARNING]: #
