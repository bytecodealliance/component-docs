# Tutorial

If you like to learn by doing, this tutorial will walk through how to build, compose, and run
components through a calculator example. Calculators can conduct many operations: add, subtract,
multiply, and so on.

In this example, each operation will be a component, that will be composed with
an `eval-expression` component that will evaluate the expression using the expected operator. With
one operation per component, this calculator is exaggeratedly granular to show how independent logic
of an application can be contained in a component.

In production, components will likely have a larger scope than a simple mathematical operation.

Our eventual solution will involve three components:

1. A calculator engine,
2. An addition operation
3. A command-line interface.

Once we have built these as separate Wasm components, we will compose them into a single runnable
component, and test it using the [`wasmtime` CLI][wasmtime].

[wasmtime]: https://wasmtime.dev/

## The calculator interface

For tutorial purposes, we are going to put our "calculator engine" and "addition operation" interfaces into two separate WIT packages, each containing one WIT file.

This setup may seem excessive, but it illustrates a real-world use case where components come
from different authors and packages.

These files can be found in the component book repository in the [`examples/tutorial/wit` directory](https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/tutorial/wit) under `wit/adder/world.wit` and `wit/calculator/world.wit`:

  ```wit
  // wit/adder/world.wit
  package docs:adder@0.1.0;

  interface add {
      add: func(x: u32, y: u32) -> u32;
  }

  world adder {
      export add;
  }
```

```wit
  // wit/calculator/world.wit
  package docs:calculator@0.1.0;

  interface calculate {
      enum op {
          add,
      }
      eval-expression: func(op: op, x: u32, y: u32) -> u32;
  }

  world calculator {
      export calculate;
      import docs:adder/add@0.1.0;
  }

  world app {
      import calculate;
  }
  ```

These files define:
* A world `adder` that exports the `add` interface. Again, components such as the calculator can call it when
  they need to add numbers.
* A world `calculator` describing the calculator component. This world exports the calculator interface, meaning
  that other components can call it to perform calculations. It imports the operation interfaces
  (such as `add`), meaning it relies on other components to perform those operations.
* An interface `calculate` that contains an evaluate function and an enum that delineates
  the operations that can be involved in a calculation. In this tutorial, the only operation is `add`.
* A world `app` describing the "primary" app component, which imports the `calculate` interface.
  This component will take in command line arguments and pass them to the `eval-expression` function
  of the calculator component.

## Create an `add` component

Reference the [language guide](language-support.md) to create a component that implements the
`adder` world of `adder/wit/world.wit`.

For reference, see the completed
[example](https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/tutorial/adder/).

## Create a `calculator` component

Reference the [language guide](language-support.md) to create a component that implements the
`calculator` world of `wit/calculator/world.wit`.

For reference, see the completed
[example](https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/tutorial/calculator/).

Once complete, the component should import the `add` function from the `adder` world and call it if the `op` enum matches `add`.

## Create a `command` component

A _command_ is a component with a specific export that allows it to be executed directly by
`wasmtime` (or other `wasi:cli` hosts).

The WebAssembly host expects it to export the [`wasi:cli/run`
interface](https://github.com/WebAssembly/wasi-cli/blob/main/wit/run.wit), which is the equivalent
of the [`main` function][wiki-entrypoint] to WASI.

To build a command component, [`cargo`][cargo] should be configured to build a `--lib` crate which
adheres to the `wasi:cli` interface. Crucially the component must export `wasi:cli/run`

To scaffold a new runnable Wasm application:

```console
cargo new --lib example
cd example
```

This component will implement the [`app`](https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/tutorial/wit/calculator/world.wit) world, which
imports the `calculate` interface.

In `Cargo.toml`, configure this crate as a `cdylib`:

```toml
[lib]
crate-type = "cdylib"
```
In addition, we'll need to modify the WIT interface that *imports* the calculation function and makes use of it to be runnable:

```diff
world app {
    import calculate;
+   export wasi:cli/run@0.2.7;
}
```

Since the calculator world imports the `add` interface, the command component needs to pull in the `adder` WIT as a dependency, as well.

To do this, we can [`wkg`][wkg] from [`wasm-pkg-tools`][wasm-pkg-tools]. We must first configure wkg to find the `docs:adder` repository:

```toml
# $XDG_CONFIG_HOME/wasm-pkg/config.toml
default_registry = "ghcr.io"

[namespace_registries]
# Tell wkg that the component-book WITs can be found at ghcr.io/bytecodealliance/docs
docs = { registry = "docs",  metadata = { preferredProtocol = "oci", "oci" = {registry = "ghcr.io", namespacePrefix = "bytecodealliance/" } } }
```

> [!NOTE]
> We have published the [`docs:adder` WIT package](https://github.com/orgs/bytecodealliance/packages/container/package/docs%2Fadder) to GHCR ahead of time, so it is easy to find/access via the configuration above.
>
> To make your own custom WITs available, please use the [`wkg publish`](./composing-and-distributing/distributing.md#distributing-wit-and-components-by-package-name-with-wkg-publish) command.

With the project scaffolded, now we must actually *implement* a command line application that:

1. Takes in three arguments: two operands and the name of an operator ("1 2 add")
2. Parses the operator name and ensures it is supported in the `op` enum
3. Calls the `calculate` interface's `eval_expression`, passing in the arguments.

For reference, see the [completed code listing](https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/tutorial/command/).

[wkg]: https://github.com/bytecodealliance/wasm-pkg-tools/tree/main/crates/wkg
[wasm-pkg-tools]: https://github.com/bytecodealliance/wasm-pkg-tools/tree/main
[wiki-entrypoint]: https://en.wikipedia.org/wiki/Entry_point
[cargo]: https://doc.rust-lang.org/cargo

## Composing the calculator

Now, we are ready to bring our components together into one runnable calculator component, using
`wac`.

We will:

1. Compose the calculator component with the add component to satisfy the calculator component's `adder` import
2. Compose that resolved calculator component once more with the command component to satisfy the command component's `calculate` import.

The result is a fully-formed command component that has all its imports satisfied and has a single
export (the `wasi:cli/run` interface), which can be executed by [`wasmtime`][wasmtime].

```sh
wac plug calculator.wasm --plug adder.wasm -o composed.wasm
wac plug command.wasm --plug composed.wasm -o final.wasm
```

## Running the calculator

Now it all adds up! Run the final component with the `wasmtime` CLI, ensuring you are using a
[recent release][wasmtime-releases] (`v14.0.0` or greater), as earlier releases of
the `wasmtime` CLI do not include component model support.

```
wasmtime run final.wasm 1 2 add
1 + 2 = 3
```

[wasmtime-releases]: https://github.com/bytecodealliance/wasmtime/releases

## To infinity and beyond!

To expand the exercise to add more components, modify `calculator.wit` to add another operator world
and expand the `op` enum. Then, modify the `command` and `calculator` components to support the
expanded enum.

Another extension of this tutorial could be to remove the `op` enum and instead modify
`eval-expression` to take in a string that can then be parsed to determine which operator component
to call. Maybe this parser is a component of its own?!

[!NOTE]: #
