# Importing and Reusing components (Rust)

## Importing an interface

The world file (`wit/world.wit`) we generated doesn't specify any imports.
If your component consumes other components, you can edit the `world.wit` file to import their interfaces.

> [!NOTE]
> This section is about importing custom WIT interfaces from library components.
> By default, `cargo-component` imports any required [WASI interfaces](https://wasi.dev/interfaces)
> for us without needing to explicitly declare them.


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

Because the `docs:adder` package is in a different project, we must first tell `cargo` how to find it. To do this, we add a
custom `wkg.toml` to our project:

```toml
[overrides]
"docs:adder" = { path = "../adder/wit" }  # directory containing the WIT package
```

After adding this configuration file, when we run `wkg wit fetch`, `wkg` will assume that the package `docs:adder` can be found
at the path that is given, and will pull it's contents into the local project under `wit/deps`.


### Calling the import from Rust

Now the declaration of `add` in the adder's WIT file is visible to the `calculator` project. To invoke the imported `add` interface from the `calculate` implementation:

```rust
// src/lib.rs
mod bindings;

use bindings::exports::docs::calculator::calculate::Guest;

// Bring the imported add function into scope
use bindings::docs::calculator::add::add;

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

When you build this using `cargo build`, the `add` interface remains unsatisfied (i.e. imported).

The calculator has taken a dependency on the `add` _interface_, but has not linked the `adder` implementation of
that interface - this is not like referencing the `adder` crate (Indeed, `calculator` could import the `add` interface even if there was no Rust project implementing the WIT file) .

You can see this by running [`wasm-tools component wit`](https://github.com/bytecodealliance/wasm-tools/tree/main/crates/wit-component) to view the calculator's world:

```
# Do a release build to prune unused imports (e.g. WASI)
$ cargo build --target=wasm32-wasip2 --release

$ wasm-tools component wit ./target/wasm32-wasip1/release/calculator.wasm
package root:component;

world root {
  import docs:adder/add@0.1.0;

  export docs:calculator/calculate@0.1.0;
}
```

As the import is unfulfilled, the `calculator.wasm` component could not run by itself in its current form.

To fulfill the `add` import, so that only `calculate` is exported, you would
need to [compose the `calculator.wasm` with some `adder.wasm` into a single, self-contained component](../../composing-and-distributing/composing.md).

[!NOTE]: #
