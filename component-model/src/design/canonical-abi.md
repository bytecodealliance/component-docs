# Canonical ABI

An ABI is an **application binary interface** - an agreement on how to pass data around in a binary format. ABIs are specifically concerned with data layout at the bits-and-bytes level. For example, an ABI might define how integers are represented (big-endian or little-endian?), how strings are represented (pointer to null-terminated character sequence or length-prefixed? UTF-8 or UTF-16 encoded?), and how composite types are represented (the offsets of each field from the start of the structure).

The component model defines a **canonical ABI** - an ABI to which all [components](./components.md) adhere. This guarantees that components can talk to each other without confusion, even if they are built in different languages. Internally, a C component might represent strings in a quite different way from a Rust component, but the canonical ABI provides a format for them to pass strings across the boundary between them.

> ⓘ For a more formal definition of what the Canonical ABI is, take a look at the [Canonical ABI explainer](https://github.com/WebAssembly/component-model/blob/main/design/mvp/CanonicalABI.md).
