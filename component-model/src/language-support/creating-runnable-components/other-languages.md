# Creating Runnable Components (Other Languages)

This guide is a work in progress and does not have examples for all language toolchains with components support.

For languages not listed in this guide, it is often possible to create runnable components by
applying the core concepts found in other guides, using the help of the available WebAssembly
toolchain.

Generally, WebAssembly toolchains in the language in question may contain a way to:

1. Create a WebAssembly component with the `_start` export (a "command" component)
2. Create a component that exports the `wasi:cli/run` interface

## Adding an Example for a Language to this Section

Are you interested in documenting this section for your language toolchain? Create a PR to this [repository][repo-pr] that adds 
the guide (similar to others in this section).

[repo-pr]: https://github.com/bytecodealliance/component-docs/pulls
