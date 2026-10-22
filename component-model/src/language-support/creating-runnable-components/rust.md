# Creating Runnable Components (Rust)

<div class="version-notice">

This page has content for both **WASI 0.2** and **WASI 0.3**. Use the tabs below to switch between versions where they differ.

> **WASI 0.3 toolchain note.** The 0.3 examples on this page use Rust's Tier 2 [`wasm32-wasip3` target](https://doc.rust-lang.org/nightly/rustc/platform-support/wasm32-wasip3.html), which targets WASI 0.3.0. The target is not yet available on stable Rust, so the 0.3 commands on this page use the nightly toolchain. Install the target with:
>
> ```sh
> rustup target add wasm32-wasip3 --toolchain nightly
> ```
>
> Once Rust 1.100 is released (scheduled for 2026-11-12), nightly is no longer required: install the target with `rustup target add wasm32-wasip3` and drop `+nightly` from the `cargo` commands.
>
> Running the resulting components requires Wasmtime 46 or later.

</div>

## Creating a command component

A _command_ is a component with a specific export that allows it to be executed directly by `wasmtime`
(or other `wasi:cli` hosts). In Rust terms, it's the equivalent of an application (`bin`) package with
a `main` function, instead of a library crate (`lib`) package.

Command components work by including a WebAssembly core export `_start` that indicates the component
has a natural `main`-like starting point.

### 1. Create a new Rust binary project

To create a command with cargo, run:

```sh
cargo new runnable-example
```

Unlike library components, this does _not_ have the `--lib` flag (`--bin` is the default for `cargo new`).

The created Rust source file is called `main.rs` instead of `lib.rs`, and contains a `main` function.

You can write Rust in this project, just as you normally would, including importing your own or third-party crates.

> All the crates that make up your project are linked together at build time, and compiled to a _single_ Wasm component. In this case, all the linking is happening at the Rust level: no WITs or component composition is involved. Only if you import Wasm interfaces do WIT and composition come into play.

### 2. Write the relevant Rust

The following code can be inserted into `runnable-example/src/main.rs`:

```rust
pub fn main() {
    eprintln!("Hello World!");
}
```

### 3. Build the component

{{#tabs global="wasi-version" }}
{{#tab name="WASI 0.2" }}
To build the component, use `cargo`:

```sh
cargo build --target=wasm32-wasip2
```

The component can also be built in release mode:

```sh
cargo build --target=wasm32-wasip2 --release
```
{{#endtab }}
{{#tab name="WASI 0.3" }}
To build the component, use `cargo` with the `wasm32-wasip3` target on the nightly toolchain:

```sh
cargo +nightly build --target=wasm32-wasip3
```

The component can also be built in release mode:

```sh
cargo +nightly build --target=wasm32-wasip3 --release
```

> Nightly is only needed until Rust 1.100 is released (scheduled for 2026-11-12); from then on, `cargo build --target=wasm32-wasip3` works on stable.
{{#endtab }}
{{#endtabs }}

### 4. Run the component with `wasmtime`

To run your command component:

{{#tabs global="wasi-version" }}
{{#tab name="WASI 0.2" }}
```sh
wasmtime run ./target/wasm32-wasip2/debug/runnable-example.wasm
```
{{#endtab }}
{{#tab name="WASI 0.3" }}
With Wasmtime 46 or later:

```sh
wasmtime run ./target/wasm32-wasip3/debug/runnable-example.wasm
```
{{#endtab }}
{{#endtabs }}

## Enabling a library component to be run via the `wasi:cli/run` interface

Sometimes, it is useful to create a component that can *both* be used as a library (via
exported interface) and executed standalone like a command component.

While reactor (library-like) components export interfaces that are meant to be used directly,
they can *also* export the [`wasi:cli/run` interface][wasi-cli-iface-run] from [WASI CLI][wasi-cli],
and signal to consumers that the library can also be run similarly to a binary that would run via a
command line interface.

Unlike command components, library components have no `_start`, but by exporting the `wasi:cli/run` interface,
tooling that recognizes these exports can easily execute a given WebAssembly binary (e.g. `wasmtime run`).

[wasi-cli-iface-run]: https://github.com/WebAssembly/WASI/blob/main/proposals/cli/wit/run.wit
[wasi-cli]: https://github.com/WebAssembly/WASI/tree/main/proposals/cli

### 1. Create a new Rust library project

To build a simple component that exports `wasi:cli/run`, first create a new Rust project:

```sh
cargo new --lib runnable-example
```

After creating the new project, ensure it is a `cdylib` crate by updating `runnable-example/Cargo.toml` and adding
the following lines:

```toml
[lib]
crate-type = ['cdylib']
```

We'll also be generating Rust bindings from WIT interfaces, so add `wit-bindgen`:

{{#tabs global="wasi-version" }}
{{#tab name="WASI 0.2" }}
```sh
cargo add wit-bindgen
```
{{#endtab }}
{{#tab name="WASI 0.3" }}
```sh
cargo add wit-bindgen --features async
```

The `async` feature enables `wit-bindgen`'s async code generation, which is needed for the 0.3 `async func run()` signature. Recent `wit-bindgen` releases enable it by default; it is listed explicitly here for older ones.
{{#endtab }}
{{#endtabs }}

### 2. Add the appropriate WIT interfaces

Then, add the appropriate WIT interfaces. For example a simple component that prints "Hello World", add the following
contents to `runnable-example/wit/component.wit`:

{{#tabs global="wasi-version" }}
{{#tab name="WASI 0.2" }}
```wit
package example:runnable;

interface greet {
    greet: func(name: string) -> string;
}

world greeter {
    export greet;
    export wasi:cli/run@0.2.7;
}
```
{{#endtab }}
{{#tab name="WASI 0.3" }}
```wit
package example:runnable;

interface greet {
    greet: func(name: string) -> string;
}

world greeter {
    export greet;
    export wasi:cli/run@0.3.0;
}
```
{{#endtab }}
{{#endtabs }}

Building a library component this way does two things:

- Enables *other* components/hosts to use the `greet` interface
- Exposes an interface (`wasi:cli/run`) that indicates this component can be run like a CLI
  - Note that no guarantees are made about what the component *does* when it runs

While we created `greet`, `wasi:cli` is a well-known interface. We can resolve this interface to local WIT by
using `wkg`:

```sh
wkg wit fetch
```

> For WASI 0.3, use `wkg` 0.15 or later. Earlier versions fail to decode the `wasi:cli@0.3.0` package.

At this point, you should have a `wit` folder with a `deps` subfolder and your original `component.wit`.

The component we will create to satisfy the WIT above can be used as a library, as other components
or platforms can use the `greet` interface export. More importantly, the component can *also* be
recognized as a generically runnable component thanks to `wasi:cli/run`, so it can work
with any tooling (ex. `wasmtime run`) that supports/recognizes the `wasi:cli` interface.

### 3. Write the code for the component

The following code can be inserted into `runnable-example/src/lib.rs`:

{{#tabs global="wasi-version" }}
{{#tab name="WASI 0.2" }}
```rust
mod bindings {
    use super::Component;

    wit_bindgen::generate!();

    export!(Component);
}

/// Component off of which implementation will hang (this can be named anything)
struct Component;

/// Implementation for the `greet` interface export
impl bindings::exports::example::runnable::greet::Guest for Component {
    fn greet(name: String) -> String {
        format!("Hello {name}!")
    }
}

/// Implementation for `wasi:cli/run` interface export
impl bindings::exports::wasi::cli::run::Guest for Component {
    fn run() -> Result<(), ()> {
        eprintln!("Hello World!");
        Ok(())
    }
}
```
{{#endtab }}
{{#tab name="WASI 0.3" }}
```rust
mod bindings {
    use super::Component;

    wit_bindgen::generate!();

    export!(Component);
}

/// Component off of which implementation will hang (this can be named anything)
struct Component;

/// Implementation for the `greet` interface export
impl bindings::exports::example::runnable::greet::Guest for Component {
    fn greet(name: String) -> String {
        format!("Hello {name}!")
    }
}

/// Implementation for `wasi:cli/run` interface export.
/// In WASI 0.3 the `run` function is asynchronous.
impl bindings::exports::wasi::cli::run::Guest for Component {
    async fn run() -> Result<(), ()> {
        eprintln!("Hello World!");
        Ok(())
    }
}
```
{{#endtab }}
{{#endtabs }}

### 4. Build the component

{{#tabs global="wasi-version" }}
{{#tab name="WASI 0.2" }}
To build the component, use `cargo`:

```sh
cargo build --target=wasm32-wasip2
```

The component can also be built in release mode:

```sh
cargo build --target=wasm32-wasip2 --release
```
{{#endtab }}
{{#tab name="WASI 0.3" }}
To build the component, use `cargo` with the `wasm32-wasip3` target on the nightly toolchain:

```sh
cargo +nightly build --target=wasm32-wasip3
```

The component can also be built in release mode:

```sh
cargo +nightly build --target=wasm32-wasip3 --release
```

> Nightly is only needed until Rust 1.100 is released (scheduled for 2026-11-12); from then on, `cargo build --target=wasm32-wasip3` works on stable.
{{#endtab }}
{{#endtabs }}

### 5. Run the component with `wasmtime`

{{#tabs global="wasi-version" }}
{{#tab name="WASI 0.2" }}
You can run the component with `wasmtime`, and unlike a generic reactor component, you do not need to specify
the interface and function to run (`wasi:cli/run` is detected and used automatically):

```console
$ wasmtime run target/wasm32-wasip2/debug/runnable_example.wasm
Hello World!
```

> Cargo converts hyphens to underscores in library crate names, so the artifact is `runnable_example.wasm` even though the package was created as `runnable-example`.
{{#endtab }}
{{#tab name="WASI 0.3" }}
You can run the component with Wasmtime 46 or later, which enables WASI 0.3 by default.
`wasi:cli/run` is detected and used automatically:

```console
$ wasmtime run target/wasm32-wasip3/debug/runnable_example.wasm
Hello World!
```

> Cargo converts hyphens to underscores in library crate names, so the artifact is `runnable_example.wasm` even though the package was created as `runnable-example`.
{{#endtab }}
{{#endtabs }}

> **Version pinning.** WASI 0.3 tools (`wit-bindgen`, Wasmtime, jco, and so on) must all target the same WIT version. Wasmtime 46 and later implement `0.3.0`; Wasmtime 43–45 implement the earlier `0.3.0-rc-2026-03-15` snapshot and cannot run components built against `0.3.0`.
