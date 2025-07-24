## Component Model Concepts

The WebAssembly Component Model extends core WebAssembly
by adding consistent representation of higher-level types
and enabling interface-driven development, amongst other benefits.
The Component Model makes core WebAssembly composable:
components that provide functionality and those that use them
can be composed together into *one* resulting component.

This section introduces the core concepts behind the component model.
For the rationale behind the component model, see [the previous section](./why-component-model.md).

### Components

A [WebAssembly Component](./components.md) is a core module extended with higher-level types and interfaces.
WebAssembly components are *nestable*:
they may contain zero or more core modules and/or sub-components composed together.
For example, a component implementing a simple calculator might be written
by composing together a component that parses strings to floating-point numbers
with a component that does the main arithmetic.

### WebAssembly Interface Types (WIT)

[WebAssembly Interface Types (WIT)][wit] is the [IDL (Interface Definition Language)][wiki-idl]
used to formally define functionality for WebAssembly modules.
WIT gives WebAssembly components the ability to express type signatures
in a language-agnostic way,
so any component binary can be both checked and executed.

### Interfaces

An [_interface_](./interfaces.md) is a collection of type definitions
and function declarations (function names accompanied by type signatures).
Typically, a single interface describes a specific, focused bit
of functionality.
For example, in [wasi-cli][wasi-cli-stdio],
three separate interfaces are used to implement `stdin`, `stdout`, and `stderr`
(the three input and output streams typically available
in a command-line environment.)

### Worlds

A [_world_](./worlds.md) is a collection of interfaces
that expresses what features a component offers
and what features it depends on.
For example, wasi-cli includes the [`stdio` world][wasi-cli-stdio],
which collects together three separate interfaces
that represent the `stdin`, `stdout`, and `stderr` streams.
Any component implementing the `stdio` world
must implement those three interfaces.

### Packages

 A [_package_](./packages.md) is a set of WIT files
 containing a related set of interfaces and worlds.
 For example, the wasi-cli package includes
 the `stdio` world and the `environment` world, among others,
 with each defined in its own WIT file.

### Platforms

In the context of WebAssembly, the _host_ is the browser or stand-alone runtime
that runs WebAssembly modules.
The _guest_ is the WebAssembly module that is executed by the host.
(These terms come from virtualization, where a guest is
a software-based virtual machine that runs on physical hardware,
which is the "host")

The Component Model introduces the idea of a _platform_
to core WebAssembly—enabling the structured, standardized use
of host functionality for WebAssembly guests.

## WASI

The WebAssembly System Interface ([WASI][wasi]) defines in WIT
a family of interfaces for common system-level functions.
WASI standardizes the functionality provided by platforms,
so that component writers can rely on functionality
that is guaranteed to be available on any conformant platform.

WASI defines common collections of functionality
such as the command line ([`wasi:cli`][wasi-cli])
or an HTTP server ([`wasi:http`][wasi-http]).

> [!NOTE]
> The Component Model is stewarded by the [Bytecode Alliance](https://bytecodealliance.org/) and designed [in the open][cm-repo].
>
> See the [`WebAssembly/component-model`][cm-repo] repository for [goals][goals], [use cases][use-cases], and [high level design choices][design-choices].

[cm-repo]: https://github.com/WebAssembly/component-model
[wiki-idl]: https://en.wikipedia.org/wiki/Interface_description_language
[goals]: https://github.com/WebAssembly/component-model/blob/main/design/high-level/Goals.md
[use-cases]: https://github.com/WebAssembly/component-model/blob/main/design/high-level/UseCases.md
[design-choices]: https://github.com/WebAssembly/component-model/blob/main/design/high-level/Choices.md
[wit]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md
[wasi]: https://wasi.dev/
[wasi-cli]: https://github.com/WebAssembly/wasi-cli/
[wasi-cli-stdio]: https://github.com/WebAssembly/wasi-cli/blob/main/wit/stdio.wit
[wasi-http]: https://github.com/WebAssembly/wasi-http

[!NOTE]: #
