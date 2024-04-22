# Distributing Components

Modern applications rely extensively on third-party packages - so extensively that distributing packages is almost an industry in itself. Traditionally, these have been specific to a language. For example, JavaScript developers are used to using packages from NPM, and Rust developers use `crates.io`. Some runtimes support binary distribution and linking, enabling limited cross-language interop; for example, Maven packages can be written in any language that targets the Java runtime. Services like this are variously referred to as "package managers" or "registries."

Today, the primary way to distribute components is via [OCI](https://opencontainers.org/) and Warg registries.  The Warg registry protocol, developed as a Bytecode Alliance project, was designed specifically for Wasm components. You can learn more about the project and codebase [here](https://github.com/bytecodealliance/registry). If you'd like to start publishing and exploring components with Warg, visit wa.dev.

## Using Warg registries for WIT packages with the `wit` CLI

One of the primary use cases of a Warg registry is publishing and downloading WIT packages.  The easiest way to create and use WIT packages is with the [wit](https://github.com/bytecodealliance/cargo-component/tree/main/crates/wit) CLI.  After installing the CLI, you can start a project by simply typing `wit init`, and then writing any valid WIT file.  If you want your WIT package to reference an interface defined in another WIT package, simply use `wit add <package_namespace>:<package_name>/<interface_name>`.  You can produce a binary whenever you're finished authoring your package, via `wit build`, and can publish to a registry via `wit publish`.

## Using Warg registries to author and distribute rust components
Another tool that is set up to interact with warg registries is `cargo-component`.  You can read about how to build and use published packages in your rust projects in the [rust](../language-support/rust.md) section.

## Using Warg registries in your component compositions
The `wac` CLI is yet another tool with warg registry integration.  Learn more about how to use registries and wac in the [composition](./composing.md#composing-components-with-the-wac-cli) section.
