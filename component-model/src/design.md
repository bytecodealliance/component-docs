# Design

This section introduces the core concepts and [rationale](./design/why-component-model.md) of the component model.

* An [interface](./design/interfaces.md) describes the types and functions used for a specific, focused bit of functionality.
* A [world](./design/worlds.md) assembles interfaces to express what features a component offers, and what features it depends on.
* [WIT (Wasm Interface Types)](./wit-overview.md) is the IDL (Interface Definition Language) used to formally define interfaces and worlds.
* A [package](./design/packages.md) is a set of WIT files containing a related set of interfaces and worlds.
* WASI (the WebAssembly System Interface) defines a family of interfaces for common system-level functions, and worlds describing common execution environments, such as the command line or a cloud-based HTTP server.
* A [component](./design/components.md) is a special kind of Wasm file, which may contain one or more core modules and/or sub-components composed together, and conforms to a world.
