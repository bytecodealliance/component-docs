## Component Model Concepts

This section introduces the core concepts and [rationale](./why-component-model.md) of the component model.

* A [WebAssembly Component](./components.md) is the next evolution of core WebAssembly binaries.
  * WebAssembly components are *nestable* -- they may contain one or more core modules and/or sub-components composed together.
* The Component Model extends core WebAssembly by introducing higher level types and interface-driven development
  * [WebAssembly Interface Types (WIT)][wit] is the [IDL (Interface Definition Language)][wiki-idl] used to formally define functionality for WebAssembly modules.
  * With WIT, WebAssembly components gain the ability to conform an language-agnostic and encode that support, so any WebAssembly component binary can be interrogated *and* executed.
  * An [Interface](./interfaces.md) describes the types and functions used for a specific, focused bit of functionality.
  * A [World](./worlds.md) assembles interfaces to express what features a component offers, and what features it depends on.
  * A [Package](./packages.md) is a set of WIT files containing a related set of interfaces and worlds.
* The Component Model introduces the idea of a "platform" to core WebAssembly -- enabling the structured, standardized use of "host" functionality for WebAssembly "guest"s.
  * The WebAssembly System Interface (WASI) defines in WIT a family of interfaces for common system-level functions.
  * WASI defines common execution environments such as the command line (`wasi:cli`) or a HTTP server (`wasi:http`).
* The Component Model makes core WebAssembly composable -- components that provide functionality and those that use them can be composed together into *one* resulting component

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

[!NOTE]: #
