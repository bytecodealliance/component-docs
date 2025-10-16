# Building a simple component (Rust)

[Rust][rust] has first-class support for WebAssembly core and WebAssembly components via the
available targets in the toolchain:

- `wasm32-unknown-unknown` ([WebAssembly core][wasm-core])
- `wasm32-wasip1` ([WASI P1][wasip1])
- `wasm32-wasip2` ([WASI P2][wasip2])

[wasm-core]: https://webassembly.github.io/spec/core/
[wasip1]: https://github.com/WebAssembly/WASI/tree/main/legacy
[wasip2]: https://github.com/WebAssembly/WASI/tree/main/wasip2

> [!NOTE]
> To use the targets above, ensure that they are enabled via the Rust toolchain (e.g. `rustup`).
>
> For example, to add the `wasm32-wasip2` target (`rustup toolchain list` can be used to show all available toolchains):
> ```
> rustup target add wasm32-wasip2
> ```

With built-in support, Rust code (and the standard library) can compile to WebAssembly with native
tooling:

```sh
cargo build --target wasm32-wasip2
```

> [!WARNING]
> While in the past the use of [`cargo-component`][cargo-component] was recommended,
> the project is in the process of being deprecated as native tooling can be used directly.

[rust]: https://rust-lang.org
[cargo-component]: https://crates.io/crates/cargo-component

## 1. Setup

Install [`wasm-tools`][wasm-tools] to enable introspection and manipulation of WebAssembly binaries:

```sh
cargo install --locked wasm-tools
```

Install [`wasmtime`][wasmtime], a fast and secure runtime for WebAssembly binaries:

```sh
curl https://wasmtime.dev/install.sh -sSf | bash
```

[wasm-tools]: https://github.com/bytecodealliance/wasm-tools#installation
[wasmtime]: https://github.com/bytecodealliance/wasmtime#installation

## 2. Creating a WebAssembly project in Rust

Create a new project in Rust with `cargo new`:

```console
$ cargo new --lib adder
    Creating library `adder` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

When building Rust WebAssembly projects, it is possible to create *either* a binary crate or
a library crate that can compile to WebAssembly (producing a "command" component or a "reactor"
component respectively), in this case opt for a reactor component.

> [!NOTE]
> The distinction between a command component and a reactor component isn't important yet,
> but they can be considered similar to the difference between a binary and a shared library.

One thing that we *will* have to add to our Rust project is in `Cargo.toml`, setting the `crate-type`:

```toml
[lib]
crate-type = ["cdylib"]
```

As we are building a reactor component (the "equivalent" of a library of functions), we must use
the [`cdylib`](https://doc.rust-lang.org/reference/linkage.html#r-link.cdylib) (stands for "c dynamic library") crate type that Rust provides.

## 3. Adding the `add` Interface

We will create a component in Rust that implements the `add` interface exported by
the [`adder` world][docs-adder] world in the `docs:adder` [WebAssembly Interface types (WIT) package][wit-docs-packages].

Create a file called `wit/world.wit` and fill it with the following content:

```wit
{{#include ../../../examples/tutorial/wit/adder/world.wit}}
```

The (WIT) types in this file represent the interface of our component must satisfy (the `adder world`).
We say that our component "exports" the `add` interface (which itself contains a single function `add`).

Working with these types is similar to other Interface Definition Language (IDL) toolchains (e.g. protobuf),
in that we will need some language level bindings that make the interface easy to implement.

[wit-docs-packages]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md#package-names

## 4. Generating Bindings for the `adder` Interface

While the Rust toolchain can compile WebAssembly binaries natively, it cannot (yet) automatically
generate bindings that match our intended (WIT) interface types (`wit/world.wit`).

We can use [`wit-bindgen`][crates-wit-bindgen] to generate bindings:

```sh
cargo add wit-bindgen
```

> [!NOTE]
> The command above should be run from inside the `adder` directory that was created by
> `cargo new` so as to be sure to add `wit-bindgen` to the dependencies of the right project. Alternatively, you can directly add `wit-bindgen` to the dependencies section of the `Cargo.toml`.
>
> It is also possible to use `wit-bindgen` as a binary via the [`wit-bindgen-cli`][crates-wit-bindgen-cli]
> crate, but here we will focus on a code-first binding build approach.

Once you have `wit-bindgen` as a part of your Rust project (i.e. in `Cargo.toml`), we can use it to generate Rust code bindings for our WIT interface. Update your `src/lib.rs` file to look like the following:

```rust
mod bindings {
    //! This module contains generated code for implementing
    //! the `adder` world in `wit/world.wit`.
    //!
    //! The `path` option is actually not required,
    //! as by default `wit_bindgen::generate` will look
    //! for a top-level `wit` directory and use the files
    //! (and interfaces/worlds) there-in.
    wit_bindgen::generate!({
        path: "wit/world.wit",
    });
}
```

Here we create a module called `bindings` that contains the code output by the [`wit_bindgen::generate` macro][rustdoc-wit-bindgen-generate].
Various `struct`s, `interface`s, `enum`s and more might be generated by `wit_bindgen`, so it's often desirable to sequester those new
types to a module that can be referred to later.

At present, the code won't *do* much, but that's because we haven't added our implementation yet.

[crates-wit-bindgen]: https://crates.io/crates/wit-bindgen
[crates-wit-bindgen-cli]: https://crates.io/crates/wit-bindgen-cli
[rustdoc-wit-bindgen-generate]: https://docs.rs/wit-bindgen/latest/wit_bindgen/macro.generate.html

## 5. Implementing the `adder` world via the generated `Guest` Trait

We can fill in functionality of the component by implementing `bindings::Guest` trait in `src/lib.rs`.
Your code should look something like the following:

```rs
{{#include ../../../examples/tutorial/adder/src/lib.rs}}
```

There are a few points of note in the code listing above:

1. The `AdderComponent` struct is introduced, but is only useful as an implementer of the `Guest` trait.
2. The `bindings::exports::docs::adder::add::Guest` trait mirrors the `docs:adder/add` interface that is exported.
3. Given (1) and (2), `AdderComponent` implements (in the WIT sense) the `adder` world, via the generated bindings.
4. The [`export!()` macro][export-macro] is generated by `wit_bindgen::generate!` macro, and does important setup.
    - `export!` is easiest used from inside the `bindings` module, *but* we need to refer to the `super::AdderComponent` struct

> [!NOTE]
> To dive into the code generated by the `wit_bindgen::generate!` macro, you can use the [`cargo-expand` crate][crates-cargo-expand]

[export-macro]: https://docs.rs/wit-bindgen/latest/wit_bindgen/macro.generate.html#exports-the-export-macro
[crates-cargo-expand]: https://crates.io/crates/cargo-expand

## 6. Building a Component

Now, let's build our component, using the native Rust toolchain to build a WASI P2 component.

```sh
cargo build --target=wasm32-wasip2
```

This performs a debug build, which produces a WebAssembly component to `target/wasm32-wasip2/debug/adder.wasm`:

```console
du -hs target/wasm32-wasip2/debug/adder.wasm
3.3M    target/wasm32-wasip2/debug/adder.wasm
```

3 megabytes is *large* for a WebAssembly component for a compiled language like Rust. Let's compile in release mode,
performing more optimizations:

```sh
cargo build --target=wasm32-wasip2 --release
```

After compiling in release mode, we get a much smaller binary:

```console
$ du -hs target/wasm32-wasip2/release/adder.wasm
16K     target/wasm32-wasip2/release/adder.wasm
```

Note that you can use many of the optimization options normally available with the Rust toolchain to control binary output.

> [!WARNING]
> Building with `--release` removes all debug-related information from the resulting `.wasm` file.
>
> When prototyping or testing locally, you might want to avoid `--release` to
> obtain useful backtraces in case of errors (for example, with
> [`wasmtime::WasmBacktraceDetails::Enable`](https://docs.rs/wasmtime/latest/wasmtime/enum.WasmBacktraceDetails.html#variant.Enable)).
> Note: the resulting `.wasm` file will be considerably larger (likely 4MB+).

## 7. Inspecting the built component

Now that we have a WIT binary, we can introspect it using WebAssembly component tooling.

For example, we can `wasm-tools` to output the WIT package of the component, because WebAssembly
components are self-documenting, and contain this information:

```sh
wasm-tools component wit target/wasm32-wasip2/release/adder.wasm
```

The command above should produce the output below:

```wit
package root:component;

world root {
  export docs:adder/add@0.1.0;
}
package docs:adder@0.1.0 {
  interface add {
    add: func(x: u32, y: u32) -> u32;
  }
}
```

## 8. Running the `adder` Component

To verify that our component works, let's run it from a Rust application that knows how to run a
component targeting the [`adder` world](#adding-the-wit-world).

The application uses [`wasmtime`][crates-wasmtime] to generate Rust "host"/"embedder" bindings,
bring in WASI worlds, and execute the component.

With the [`component-docs` repository][repo-component-docs] cloned locally, run the following:

```console
$ cd examples/example-host
$ cargo run --release -- 1 2 ../add/target/wasm32-wasip1/release/adder.wasm
1 + 2 = 3
```

With this, we have successfully built and run a basic WebAssembly component with Rust 🎉

[crates-wasmtime]: https://crates.io/crates/wasmtime
[repo-component-docs]: https://github.com/bytecodealliance/component-docs
[docs-adder]: https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/tutorial/wit/adder/world.wit
[!NOTE]: #
[!WARNING]: #
