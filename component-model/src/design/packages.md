# WIT Packages

A **WIT package** is a set of one or more [WebAssembly Interface Type](./wit.md) (WIT) files
that, taken together, contain a set of interfaces and worlds that are related to each other.
WIT is an [interface definition language][wp-idl] (IDL) for the component model.
Packages provide a way for worlds and interfaces to refer to each other,
and thus for an ecosystem of components to share common definitions.

A WIT package groups related interfaces and worlds together
for ease of discovery and reference.
A package is not a [world](./worlds.md): a package maps to one or more files
and contains worlds.
A world is a bundle of imported and exported types and interfaces.

* The WebAssembly System Interface (WASI) defines a number of packages,
  including one named `wasi:clocks`.
  Our HTTP proxy world could import the `wasi:clocks/wall-clock` interface
  (read as "the `wall-clock` interface from the `wasi:clocks` package"),
  rather than having to define a custom clock interface.

> For a more formal definition of what a WIT package is, take a look at the [WIT specification](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md).

[wp-idl]: https://en.wikipedia.org/wiki/Interface_description_language
