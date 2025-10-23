# Importing and Reusing components (Other Languages)

This guide is a work in progress and does not have examples for all language toolchains with components support.

For languages not listed in this guide, it is often possible to import and reuse WebAssembly components
by following the main principles of the other guides where applicable, using the help of the local
WebAssembly toolchain.

Generally, WebAssembly toolchains in the language in question should contain a way to:

1. Create components that import other components
2. Create host/platforms that can load and run components
3. Compose together components (possibly during build time)

Note that generic tooling like [`wac`][wac] can be used to compose components together, regardless of language.

[wac]: https://github.com/bytecodealliance/wac

## Adding a New Language to the Guide

Are you interested in documenting this section for your language toolchain? Create a PR to this [repository][repo-pr] that adds 
the guide (similar to others in this section).

[repo-pr]: https://github.com/bytecodealliance/component-docs/pulls
