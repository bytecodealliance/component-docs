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
|                          | [MoonBit]               |                   |

[Why Components?]: ./design/why-component-model.md
[Components]: ./design/components.md
[Interfaces]: ./design/interfaces.md
[Worlds]: ./design/worlds.md

[C/C++]: ./language-support/building-a-simple-component/c.md
[C#]: ./language-support/building-a-simple-component/csharp.md
[Go]: ./language-support/building-a-simple-component/go.md
[JavaScript]: ./language-support/building-a-simple-component/javascript.md
[Python]: ./language-support/building-a-simple-component/python.md
[Rust]: ./language-support/building-a-simple-component/rust.md
[MoonBit]: ./language-support/building-a-simple-component/moonbit.md

[Composing]: ./composing-and-distributing/composing.md
[Running]: ./running-components.md
[Distributing]: ./composing-and-distributing/distributing.md


## WebAssembly components

As with all programming, the goal of writing a component
is to make new functionality available
by building it out of existing functionality.

A WebAssembly component runs on a _platform_,
which may be a Web browser,
a stand-alone runtime,
or even an operating system (when compiling WebAssembly to an executable).
By running the component, the platform gains the functionality
that the component implements.
Likewise, the platform provides functionality
that code in components can use to interact
with the outside world.

For example:

- A user of the component model can build a component
  that converts the system time to another time zone.
- For the component to work as intended, the underlying platform
  must provide the component with a means to access
  the current system time and the system time zone.

## APIs for building WebAssembly components

In general, a platform that runs components
must provide well-defined APIs for accessing functionality
that components need:
for example, reading from standard input,
accessing environment variables,
or manipulating network sockets.

It's useful to have a standard, shared set of APIs
that WebAssembly components can depend on.
[WASI](https://wasi.dev/) (the WebAssembly System Interface) is a standards-track specification
that defines these APIs.
A system or platform may expose some or all of the WASI APIs to components.

### Status

The current stable release of WASI is [WASI 0.2.0](https://github.com/WebAssembly/WASI/pull/577),
which was released on January 25, 2024.
WASI 0.2.0 is [a stable set of WIT definitions](https://github.com/WebAssembly/WASI/blob/main/docs/Preview2.md)
that components can target.
WASI proposals will continue to evolve and new ones will be introduced;
however, users of the component model can now pin to any stable release >= `v0.2.0`.
The [WASI.dev roadmap](https://wasi.dev/roadmap) tracks upcoming releases.

## Contributing

If you find a mistake, omission, ambiguity, or other problem, please let us know via [GitHub issues](https://github.com/bytecodealliance/component-docs/issues).

If you'd like to contribute content to the guide, please see the [contribution guide](https://github.com/bytecodealliance/component-docs/blob/main/CONTRIBUTING.md) for information on how to contribute.

[!NOTE]: #
