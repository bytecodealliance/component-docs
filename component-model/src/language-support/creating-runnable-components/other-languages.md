# Creating Runnable Components (Other Languages)

This guide is a work in progress and does not have examples for all language toolchains with components support.

For languages not listed in this guide, it is often possible to create runnable components by
following the main principles of the other guides, using the help of the available WebAssembly
toolchain.

Generally, WebAssembly toolchains in the language in question may contain a way to:

1. Create a WebAssembly component with the `_start` export (a "command" compnent)
2. Create a component that exports the `wasi:cli/run` interface

## Adding a New Language to the Guide

Know of a language guide we should add to this guide? Create a PR to this [repository][repo-pr] that adds 
the new language guide (similar to others in this section).

[repo-pr]: https://github.com/bytecodealliance/component-docs/pulls
