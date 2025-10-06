# Creating Runnable Components

This section contains language-specific guides on how to create runnable components.

At a high level there are at least two ways to create components that are more like binaries than libraries
(i.e. that are easy to run from a tool like `wasmtime`):

1. Exporting the `wasi:cli/run` interface (recommended)
2. Creating a "command" component

This section explores how to do the above in relevant languages.

## Languages

This guide is implemented for various languages:

| Language                                                        |
|-----------------------------------------------------------------|
| [Rust](./language-support/creating-runnable-components/rust.md) |

[docs-wit]: ./design/wit.md
