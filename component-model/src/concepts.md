# Concepts

The Component Model is predicated on a small number of fundamental concepts:

## Interfaces

An **interface** describes a single-focus, composable contract, through which components can interact with each other and with hosts. Interfaces describe the types and functions used to carry out that interaction. For example:

* A "receive HTTP requests" interface might have only a single "handle request" function, but contain types representing incoming requests, outgoing responses, HTTP methods and headers, and so on.
* A "wall clock" interface might have two functions, one to get the current time and one to get the granularity of the timer. It would also include a type to represent an instant in time.

## Canonical ABI

An ABI is an **application binary interface** - an agreement on how to pass data around in a binary format. ABIs are specifically concerned with data layout at the bits-and-bytes level. For example, an ABI might define how integers are represented (big-endian or little-endian?), how strings are represented (pointer to null-terminated character sequence or length-prefixed? UTF-8 or UTF-16 encoded?), and how composite types are represented (the offsets of each field from the start of the structure).

The component model defines a **canonical ABI** - an ABI to which all components adhere. This guarantees that components can talk to each other without confusion, even if they are built in different languages. Internally, a C component might represent strings in a quite different way from a Rust component, but the canonical ABI provides a format for them to pass strings across the boundary between them.

## Worlds

A **world** is a higher-level contract that describes a component's capabilities and needs.

In one sense, a world _defines_ a component - it says what interfaces the component exposes for other code to call, and what interfaces the component depends on - but it defines only the surface of a component, not the internal behaviour. This is a useful way to think about "business logic" components, like libraries that export APIs for other components to use. If you're an application or library developer, you'll define custom worlds to expose the functionality of each component you create - and to declare the functionality your component depends on.

In another sense, though, a world defines a _hosting environment_ for components. For example, WASI (the WebAssembly System Interface) defines a "command line" world which imports interfaces such as file I/O, random number generation, clocks and so on - the sort of APIs a piece of code running in a POSIX or Win32 environment might want to call. Worlds like this are more akin to operating systems or server frameworks; unless you're building a Wasm host, you'll never define or implement one of these worlds, only _use_ the worlds these environments define for you. The 'world' concept unifies the "library" and "host" senses at a technical level, which enables some powerful techniques, but can also be daunting when first encountering the component model!

A world is composed of interfaces, but each interface is _directional_ - it indicates whether the interface is available for outside code to call (an "export"), or outside code must fulfill the interface for the component to call (an "import"). These interfaces strictly bound the component. A component cannot interact with anything outside itself except by having its exports called, or by calling its imports. This provides very strong sandboxing; for example, if a component does not have an import for a secret store, then it _cannot_ access that secret store, even if the store is running in the same process.

For a component to run, its imports must be fulfilled, by a host or by other components.  Connecting up one component's imports to another component's matching exports is called _composition_.

* A (trivial) "HTTP proxy" world would export a "receive HTTP requests" interface, and import a "send HTTP requests" interface. A host, or another component, would call the exported "receive" interface, passing an HTTP request; the component would forward it on via the imported "send" interface. To be a _useful_ proxy, the component may also need to import interfaces such as I/O and clock time - without those imports the component could not perform, for example, on-disk caching.
* The "WASI (WebAssembly System Interface) command line" world is a classic example of a "hosting environment" use of the world concept. This world exports APIs such as file I/O, sockets, random number generation, and other POSIX-style functionality, so that application components that depend on this world - that is, command line applications - can use those familiar capabilities.
* A "regex parser" world would export a "parse regex" function, and would import nothing. This declares not only that the component implementing this world can parse regular expressions, but also that it calls no other APIs. A user of such a parser could know, without looking at the implementation, that is does not access the file system, or send the user's regexes to a network service.

## Components

Physically, a **component** is a specially-formatted WebAssembly file. Internally, the component could include multiple traditional ("core") WebAssembly modules, and sub-components, composed via their imports and exports.

The external interface of a component - its imports and exports - corresponds to a world. The component, however, internally defines how that world is implemented.

## Packages

A **package** is a set of one or more WIT (Wasm Interface Type) files containing a related set of interfaces and worlds. WIT is an IDL (interface definition language) for the Component Model. Packages provide a way for worlds and interfaces to refer to each other, and thus for an ecosystem of components to share common definitions.

A package is not a world. It's a way of grouping related interfaces and worlds together for ease of discovery and reference, more like a namespace.

* The WebAssembly System Interface (WASI) defines a number of packages, including one named `wasi:clocks`. Our HTTP proxy world could import the `wall-clock` interface from the `wasi:clocks` package, rather than having to define a custom clock interface.
