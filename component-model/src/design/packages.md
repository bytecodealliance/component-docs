# Packages

A **package** is a set of one or more WIT (Wasm Interface Type) files containing a related set of interfaces and worlds. WIT is an IDL (interface definition language) for the Component Model. Packages provide a way for worlds and interfaces to refer to each other, and thus for an ecosystem of components to share common definitions.

A package is not a world. It's a way of grouping related interfaces and worlds together for ease of discovery and reference, more like a namespace.

* The WebAssembly System Interface (WASI) defines a number of packages, including one named `wasi:clocks`. Our HTTP proxy world could import the `wall-clock` interface from the `wasi:clocks` package, rather than having to define a custom clock interface.
