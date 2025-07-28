## Component Model Concepts

The WebAssembly Component Model extends core WebAssembly in several ways:
* It adds consistent representation of higher-level types
* It enables interface-driven development
* It makes core WebAssembly composable:
components that provide functionality and those that use them
can be composed together into *one* resulting component.

This section introduces the core concepts behind the component model.
For the rationale behind the component model, see [the previous section](./why-component-model.md).

### Components

A [WebAssembly Component](./components.md) is a binary that
conforms to the [Canonical ABI](../advanced/canonical-abi.md);
often a WebAssembly core module extended with the features
of the Component Model
(higher-level types, interfaces).
WebAssembly components are *nestable*:
they may contain zero or more core modules and/or sub-components composed together.
For example, a component implementing a simple calculator might be written
by composing together a component that parses strings to floating-point numbers
with a component that does the main arithmetic.

### WebAssembly Interface Types (WIT)

[WebAssembly Interface Types (WIT)][wit] is the [IDL (Interface Definition Language)][wiki-idl]
used to formally define functionality for WebAssembly components.
WIT gives WebAssembly components the ability to express type signatures
in a language-agnostic way,
so any component binary can be checked, composed and executed.

#### Interfaces

An [_interface_](./interfaces.md) is a collection of type definitions
and function declarations (function names accompanied by type signatures).
Typically, a single interface describes a specific, focused bit
of functionality.
For example, in [wasi-cli][wasi-cli-stdio],
three separate interfaces are used to implement `stdin`, `stdout`, and `stderr`
(streams typically available in command-line-like environments)

### Worlds

A [_world_](./worlds.md) is a collection of interfaces and types
that expresses what features a component offers
and what features it depends on.
For example, wasi-cli includes the [`command` world][wasi-cli-command],
which depends on interfaces
that represent the `stdin`, `stdout`, and `stderr` streams,
among other things.
A component implementing the `command` world
must be invoked in an environment that implements those interfaces.

### Packages

 A [_package_](./packages.md) is a set of WIT files
 containing a related set of interfaces and worlds.
 For example, the [wasi-http](https://github.com/WebAssembly/wasi-http/blob/main/wit/proxy.wit) package includes
an `imports` world encapsulating the interfaces that an HTTP proxy depends on,
and a `proxy` world that depends on `imports`.

### Platforms

In the context of WebAssembly, a _host_ refers to a WebAssembly runtime
capable of executing WebAssembly binaries.
The runtime can be inside a browser or can stand alone.
A _guest_ refers to the WebAssembly binary that is executed by the host.
(These terms borrow from their analogs in [virtualization](https://en.wikipedia.org/wiki/Virtualization), where a guest is
a software-based virtual machine that runs on physical hardware,
which is the "host")

The Component Model introduces the idea of a _platform_
to core WebAssembly—enabling the structured, standardized use
of host functionality for WebAssembly guests.
Components may import functionality that is provided
by the platform on which they are executed.

### WASI

The WebAssembly System Interface ([WASI][wasi]) defines in WIT
a family of interfaces for common system-level functions.
WASI defines a platform for component writers that mimics
existing programs that developers are familiar with
(for example, `wasi-cli` or `wasi-http`),
standardizing the functionality components depend on.

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
[wasi-cli-command]: https://github.com/WebAssembly/wasi-cli/blob/main/wit/command.wit
[wasi-http]: https://github.com/WebAssembly/wasi-http

[!NOTE]: #
