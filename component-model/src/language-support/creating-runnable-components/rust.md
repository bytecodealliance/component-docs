# Creating Runnable Components (Rust)

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

To build the component, use `cargo`:

```sh
cargo build --target=wasm32-wasip2
```

The component can also be built in release mode:

```console
cargo build --target=wasm32-wasip2 --release
```

### 4. Run the component with `wasmtime`

To run your command component:

```sh
wasmtime run ./target/wasm32-wasip2/debug/runnable-example.wasm
```

## Enabling a library component to be run via the `wasi:cli/run` interface

While reactor (library-like) components export interfaces that are meant to be used directly,
they can *also* export the [`wasi:cli/run` interface][wasi-cli-iface-run] from [WASI CLI][wasi-cli],
and signal to consumers that the library can also be run similarly to a binary that would run via a
command line interface.

Unlike command components, library components have no `_start`, but by exporting the `wasi:cli/run` interface,
tooling that recognizes these exports can easily execute a given WebAssembly binary (e.g. `wasmtime run`).

[wasi-cli-iface-run]: https://github.com/WebAssembly/wasi-cli/tree/main/wit/run.wit
[wasi-cli]: https://github.com/WebAssembly/wasi-cli

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

```sh
cargo add wit-bindgen
```

### 2. Add the appropriate WIT interfaces

Then, add the appropriate WIT interfaces. For example a simple component that prints "Hello World", add the following
contents to `runnable-example/wit/component.wit`:

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

Building a library component this way does two things:

- Enables *other* components/hosts to use the `greet` interface
- Exposes an interface (`wasi:cli/run`) that indicates this component can be run like a CLI
  - Note that no guarantees are made about what the component *does* when it runs

While we created `greet`, `wasi:cli` is a well-known interface. We can resolve this interface to local WIT by
using `wkg`:

```sh
wkg wit fetch
```

At this point, you should have a `wit` folder with a `deps` subfolder and your original `component.wit`.

The component we will create to satisfy the WIT above can be used as a library, as other components
or platforms can use the `greet` interface export. More importantly, the component can *also* be
recognized as a generically runnable component thanks to `wasi:cli/run`, so it can work
with any tooling (ex. `wasmtime run`) that supports/recognizes the `wasi:cli` interface.

[!WARNING]: #

### 3. Write the code for the component

The following code can be inserted into `runnable-example/src/lib.rs`:

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

### 4. Build the component

To build the component, use `cargo`:

```sh
cargo build --target=wasm32-wasip2
```

The component can also be built in release mode:

```console
cargo build --target=wasm32-wasip2 --release
```

### 5. Run the component with `wasmtime`

You can run the component with `wasmtime`, and unlike a generic reactor component, you do not need to specify
the interface and function to run (`wasi:cli/run` is detected and used automatically):

```console
$ wasmtime run target/wasm32-wasip2/runnable-example.wasm
Hello World!
```
