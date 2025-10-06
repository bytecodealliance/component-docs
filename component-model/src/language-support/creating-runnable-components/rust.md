# Creating Runnable Components (Rust)

## Exporting the `wasi:cli/run` interface

Any reactor (library-like) component can *also* export the [`run` interface]wasi-cli-iface-run] inside [WASI CLI][wasi-cli],
and signal to ecosystem projects that it can be executed.

> [!WARNING]
> Reactor components can be reused, and while most platforms may *not* choose to reuse a component after `wasi:cli/run`
> has been called, there is no guarantee that they will or will not.

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

world greeter {
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

At this point, you should have a `wit` folder with a `deps` subfolder and yoru original `component.wit`.

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

export bindings::wasi::cli::run::Guest for Component {
    fn run(&self) -> Result<(), ()> {
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

### 5. Run the component with `wasmtime`

You can run the component with `wasmtime`, and unlike a generic reactor component, you do not need to specify
the interface and function to run (`wasi:cli/run` is detected and used automatically):

```console
$ wasmtime run target/wasm32-wasip2/runnable-example.wasm
Hello World!
```

## Creating a command component

A _command_ is a component with a specific export that allows it to be executed directly by `wasmtime`
(or other `wasi:cli` hosts). In Rust terms, it's the equivalent of an application (`bin`) package with
a `main` function, instead of a library crate (`lib`) package.

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

### 4. Run the component with `wasmtime`

To run your command component:

```sh
wasmtime run ./target/wasm32-wasip2/debug/runnable-example.wasm
```

> [!WARNING]
> If your program prints to standard out or error, you may not see the printed output!
>
> Some versions of `wasmtime` have a bug where they don't flush output streams before exiting. To work
> around this, add a `std::thread::sleep()` with a 10 millisecond delay before exiting `main`.
