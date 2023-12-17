# Components in Rust

Rust has first-class support for the component model via [the `cargo component` tool](https://github.com/bytecodealliance/cargo-component). It is a `cargo` subcommand for
creating WebAssembly components using Rust as the component's implementation language.

## Installing `cargo component`

To install `cargo component`, run:

```sh
cargo install cargo-component
```

> You can find more details about `cargo component` in its [crates.io page](https://crates.io/crates/cargo-component).

## Building a Component with `cargo component`

Create a Rust program that implements the `add` function in the [`example`
world](https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/example-host/add.wit). Note that it imports the bindings that will be created by
`cargo-component`. First scaffold a project:

```sh
$ cargo component new add --reactor && cd add
```

Update `wit/world.wit` to match `add.wit` and modify the component package reference to change the
package name to `example`. The `component` section of `Cargo.toml` should look like the following:

```toml
[package.metadata.component]
package = "component:example"
```

`cargo-component` will generate bindings for the world specified in a package's `Cargo.toml`. In particular, it will create a `Guest` trait that a component should implement. Since our `example` world has no interfaces, the trait lives directly under the bindings module. Implement the `Guest` trait in `add/src/lib.rs` such that it satisfied the `example` world, adding an `add` function. It should look similar to the following:

```rs
cargo_component_bindings::generate!();
use bindings::Guest;

struct Component;

impl Guest for Component {
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
}
```

Now, build the component, being sure to optimize with a release build.

```sh
$ cargo component build --release
```

You can use `wasm-tools component wit` to output the WIT package of the component:

```sh
$ wasm-tools component wit add/target/wasm32-wasi/release/add.wasm
package root:component;

world root {
  export add: func(x: s32, y: s32) -> s32;
}
```

### Running a Component from Rust Applications

To verify that our component works, lets run it from a Rust application that knows how to import a
component of the [`example` world](https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/example-host/add.wit).

The application uses [`wasmtime`](https://github.com/bytecodealliance/wasmtime) crates to generate
Rust bindings, bring in WASI worlds, and execute the component.

```sh
$ cd examples/add-host
$ cargo run --release -- 1 2 ../add/target/wasm32-wasi/release/add.wasm
1 + 2 = 3
```

See [the language guide](../language-support.md#building-a-component-with-cargo-component).

## Exporting an interface with `cargo component`

The [sample `add.wit` file](https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/example-host/add.wit) exports a function. However, to use your component from another component, it must export an interface. This results in slightly fiddlier bindings. For example, to implement the following world:

```wit
package docs:adder@0.1.0;

interface add {
    add: func(a: u32, b: u32) -> u32;
}

world adder {
    export add;
}
```

you would write the following Rust code:

```rust
cargo_component_bindings::generate!();

// Separating out the interface puts it in a sub-module
use bindings::exports::docs::adder::add::Guest;

struct Component;

impl Guest for Component {
    fn add(a: u32, b: u32) -> u32 {
        a + b
    }
}
```

## Importing an interface with `cargo component`

The world file (`wit/world.wit`) generated for you by `cargo component new --reactor` doesn't specify any imports.

> `cargo component build`, by default, uses the Rust `wasm32-wasi` target, and therefore automatically imports any required WASI interfaces - no action is needed from you to import these. This section is about importing custom WIT interfaces from library components.

If your component consumes other components, you can edit the `world.wit` file to import their interfaces.

For example, suppose you have created and built an adder component as explained in the [exporting an interface section](#exporting-an-interface-with-cargo-component) and want to use that component in a calculator component. Here is a partial example world for a calculator that imports the add interface:

```wit
// in the 'calculator' project

// wit/world.wit
package docs:calculator;

interface calculate {
    eval-expression: func(expr: string) -> u32;
}

world calculator {
    export calculate;
    import docs:adder/add@0.1.0;
}
```

### Referencing the package to import

Because the `docs:adder` package is in a different project, we must first tell `cargo component` how to find it. To do this, add the following to the `Cargo.toml` file:

```toml
[package.metadata.component.target.dependencies]
"docs:adder" = { path = "../adder/wit" }  # directory containing the WIT package
```

Note that the path is to the adder project's WIT _directory_, not to the `world.wit` file. A WIT package may be spread across multiple files in the same directory; `cargo component` will look at all the files.

### Calling the import from Rust

Now the declaration of `add` in the adder's WIT file is visible to the `calculator` project. To invoke the imported `add` interface from the `calculate` implementation:

```rust
// src/lib.rs
cargo_component_bindings::generate!();

use bindings::exports::docs::calculator::calculate::Guest;

// Bring the imported add function into scope
use bindings::docs::adder::add::add;

struct Component;

impl Guest for Component {
    fn eval_expression(expr: String) -> u32 {
        // Cleverly parse `expr` into values and operations, and evaluate
        // them meticulously.
        add(123, 456)
    }
}
```

### Fulfilling the import

When you build this using `cargo component build`, the `add` interface remains imported. The calculator has taken a dependency on the `add` _interface_, but has not linked the `adder` implementation of that interface - this is not like referencing the `adder` crate. (Indeed, `calculator` could import the `add` interface even if there was no Rust project implementing the WIT file.) You can see this by running [`wasm-tools component wit`](https://github.com/bytecodealliance/wasm-tools/tree/main/crates/wit-component) to view the calculator's world:

```
# Do a release build to prune unused imports (e.g. WASI)
$ cargo component build --release

$ wasm-tools component wit ./target/wasm32-wasi/release/calculator.wasm
package root:component;

world root {
  import docs:adder/add@0.1.0;

  export docs:calculator/calculate@0.1.0;
}
```

As the import is unfulfilled, the `calculator.wasm` component could not run by itself in its current form. To fulfil the `add` import, so that the calculator can run, you would need to [compose the `calculator.wasm` and `adder.wasm` files into a single, self-contained component](../creating-and-consuming/composing.md).

## Creating a command component with `cargo component`

A _command_ is a component with a specific export that allows it to be executed directly by `wasmtime` (or other `wasm:cli` hosts). In Rust terms, it's the equivalent of an application (`bin`) package with a `main` function, instead of a library crate (`lib`) package.

To create a command with cargo component, run:

```sh
cargo component new <name>
```

Unlike library components, this does _not_ have the `--reactor` flag. You will see that the created project is different too:

- It doesn't contain a `.wit` file. `cargo component build` will automatically export the `wasm:cli/run` interface for Rust `bin` packages, and hook it up to `main`.
- Because there's no `.wit` file, `Cargo.toml` doesn't contain a `package.metadata.component.target` section.
- The Rust file is called `main.rs` instead of `lib.rs`, and contains a `main` function instead of an interface implementation.

You can write Rust in this project, just as you normally would, including importing your own or third-party crates.

> All the crates that make up your project are linked together at build time, and compiled to a _single_ Wasm component. In this case, all the linking is happening at the Rust level: no WITs or component composition is involved. Only if you import Wasm interfaces do WIT and composition come into play.

To run your command component:

```
cargo component build
wasmtime run --wasm component-model ./target/wasm32-wasi/debug/<name>.wasm
```

> **WARNING:** If your program prints to standard out or error, you may not see the printed output! Some versions of `wasmtime` have a bug where they don't flush output streams before exiting. To work around this, add a `std::thread::sleep()` with a 10 millisecond delay before exiting `main`.

### Importing an interface into a command component

As mentioned above, `cargo component build` doesn't generate a WIT file for a command component. If you want to import a Wasm interface, though, you'll need to create a WIT file and a world, plus reference the packages containing your imports:

1. Add a `wit/world.wit` to your project, and write a WIT world that imports the interface(s) you want to use. For example:

```wit
package docs:app;

world app {
    import docs:calculator/calculate@0.1.0;
}
```

> `cargo component` sometimes fails to find packages if versions are not set explicitly. For example, if the calculator WIT declares `package docs:calculator` rather than `docs:calculator@0.1.0`, then you may get an error even though `cargo component build` automatically versions the binary export.

2. Edit `Cargo.toml` to tell `cargo component` about the new WIT file:

```toml
[package.metadata.component.target]
path = "wit"
```

(This entry is created automatically for library components but not for command components.)

3. Edit `Cargo.toml` to tell `cargo component` where to find external package WITs:

```toml
[package.metadata.component.target.dependencies]
"docs:calculator" = { path = "../calculator/wit" }
"docs:adder" = { path = "../adder/wit" }
```

> If the external package refers to other packages, you need to provide the paths to them as well.

4. Use the imported interface in your Rust code:

```rust
use bindings::docs::calculator::calculate::eval_expression;

fn main() {
    let result = eval_expression("1 + 1");
    println!("1 + 1 = {result}");
    std::thread::sleep(Duration::from_millis(10));
}
```

5. [Compose the command component with the `.wasm` components that implement the imports.](../creating-and-consuming/composing.md)

6. Run the composed component:

```sh
$ wasmtime run --wasm component-model ./my-composed-command.wasm
1 + 1 = 579  # might need to go back and do some work on the calculator implementation
```
