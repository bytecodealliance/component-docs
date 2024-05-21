# Distributing Components

Modern applications rely extensively on third-party packages - so extensively that distributing packages is almost an industry in itself. Traditionally, these have been specific to a language. For example, JavaScript developers are used to using packages from NPM, and Rust developers use `crates.io`. Some runtimes support binary distribution and linking, enabling limited cross-language interop; for example, Maven packages can be written in any language that targets the Java runtime. Services like this are variously referred to as "package managers" or "registries."

Today, the primary way to distribute components is via [OCI](https://opencontainers.org/) and Warg registries.  The Warg registry protocol, developed as a Bytecode Alliance project, was designed specifically for Wasm components. You can learn more about the project and codebase [here](https://github.com/bytecodealliance/registry). If you'd like to start publishing and exploring components with Warg, visit wa.dev.

## Using Warg registries for WIT packages with the `wit` CLI

One of the primary use cases of a Warg registry is publishing and downloading WIT packages.  The easiest way to create and use WIT packages is with the [wit](https://github.com/bytecodealliance/cargo-component/tree/main/crates/wit) CLI.  After installing the CLI, you can use it to create WIT packages that depend on other packages in a registry, as well as to build a final binary and publish the result to a registry

## Using Warg registries to author and distribute components
Another tool that is set up to interact with warg registries is `cargo-component`.  You can read about how to build and use published packages in your Rust projects in the [Rust](../language-support/rust.md) section.  Components authored with other tools and languages can also be published to the registry, but have less support for importing/exporting functionality between components today.  In general, given a .wasm binary, you can always publish and download using the [warg cli](https://github.com/bytecodealliance/registry?tab=readme-ov-file#installation)

## Using Warg registries in your component compositions
The `wac` CLI is yet another tool with warg registry integration.  Learn more about how to use registries when you compose components in the [composition](./composing.md#composing-components-with-the-wac-cli) section.
