# Creating Components with Rust

## Creating a starter component

### 1. Install `cargo component`

`cargo component` is a Cargo add-in for creating, maintaining, and publish Wasm components.

To install `cargo component`, run:

```
cargo install --git https://github.com/bytecodealliance/cargo-component --locked cargo-component
```

> You cannot currently install `cargo component` from `crates.io`. You must install it from GitHub as shown above.

### 2. Create a Rust component project

Run `cargo component new --reactor <name>` (where `<name>` is the project name you want to give to the component).

> `--reactor` generates a "library" component, that exports an interface that other components can call. If you don't pass `--reactor`, it generates a _command_ component - that is, a program that runs and then exits.

This creates a Rust library project and a [WIT](../wit-overview.md) file.

* The `wit/world.wit` file defines the _interface_ of your new component. As scaffolded by `cargo component`, it exports one function, `hello-world`. That function - and only that function - will be available to consumers of your library component. In Rust terms, it is as if that was the one and only `pub` function exported by your crate.
* The `src/lib.rs` file refers to the `wit/world.wit` file, and contains a Rust struct that implements the _interface_ in the WIT.

### 3. Build the Wasm component

In the newly created project directory, run `cargo component build`. This compiles the Rust code in `src/lib.rs` into a Wasm component file.

The Wasm file is output to `target/wasm32-wasi/debug`.

### 4. Try out the new component

_TODO: HOW?_

## TODO

_TODO: describe the bindings generation stuff (but don't need to go into too much detail)_

_TODO: how to build out a component from a WIT interface_
