# Importing and reusing components

This section contains language-specific guides on how to reuse existing WebAssembly
components, in particular using an `adder` component to complete a `calculator` component.

The `adder` component has the following [WIT][docs-wit] interface:

```wit
{{#include ../examples/tutorial/wit/adder/world.wit}}
```

The `calculator` component has the following interface:

```wit
{{#include ../examples/tutorial/wit/calculator/world.wit}}
```

## Languages

This guide is implemented for various languages:

| Language                                                                        |
|---------------------------------------------------------------------------------|
| [Rust](./language-support/importing-and-reusing-components/rust.md)             |
| [Javascript](./language-support/importing-and-reusing-components/javascript.md) |

[docs-wit]: ./design/wit.md
