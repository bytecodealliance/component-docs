# Tutorial

If you like to learn by doing, this tutorial will walk through how to build, compose, and run
components through a calculator example. Calculators can conduct many operations: add, subtract,
multiply, and so on. In this example, each operation will be a component, that will be composed with
an `eval-expression` component that will evaluate the expression using the expected operator. With
one operation per component, this calculator is exaggeratedly granular to show how independent logic
of an application can be contained in a component. In production, components will likely have a
larger scope than a simple mathematical operation.

Our eventual solution will involve three components: one for the calculator engine, one for the
addition operation, and one for the command-line interface. Once we have built these as separate
Wasm components, we will compose them into a single runnable component, and test it using the
`wasmtime` CLI.

## The calculator interface

For tutorial purposes, we are going to define all our interfaces in one WIT package (in fact, one
`.wit` file).  This file defines:

* An interface for the calculator itself.  We'll use this later to carry out calculations. It
  contains an evaluate function, and an enum that delineates the operations that can be involved in
  a calculation. In this tutorial, the only operation is `add`.
* Interfaces for the various operations the calculator might need to carry out as part of a
  calculation. For the tutorial, again, the only interface we define is for the "add" operation.
* A world describing the calculator component. This world exports the calculator interface, meaning
  that other components can call it to perform calculations. It imports the operation interfaces
  (such as "add"), meaning it relies on other components to perform those operations.
* A world describing each operator component. Again, there's just the "adder" world right now, and
  this exports the "add" interface, meaning that components such as the calculator can call it when
  they need to add numbers.
* A world describing the "primary" app component, which imports the "calculate" interface. This is
  the component will take in command line arguments and pass them to the "eval-expression" function
  of the calculator component.

```wit
// calculator.wit
package docs:calculator@0.1.0;

interface calculate {
    enum op {
        add,
    }
    eval-expression: func(op: op, x: u32, y: u32) -> u32;
}

interface add {
    add: func(a: u32, b: u32) -> u32;
}

world adder {
    export add;
}

world calculator {
    export calculate;
    import add;
}

world app {
    import calculate;
}

```

## Create an `add` component

Reference the [language guide](language-support.md) and [authoring components
documentation](creating-and-consuming/authoring.md) to create a component that implements the
`adder` world of `calculator.wit`. For reference, see the completed
[example](https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/tutorial/adder/).

## Create a `calculator` component

Reference the [language guide](language-support.md) and [authoring components
documentation](creating-and-consuming/authoring.md) to create a component that implements the
`calculator` world of `calculator.wit`. For reference, see the completed
[example](https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/tutorial/calculator/). The component should import the `add` function from the
`adder` world and call it if the `op` enum matches `add`.

## Crate a `command` component

A _command_ is a component with a specific export that allows it to be executed directly by
`wasmtime` (or other `wasm:cli` hosts). The host expects it to export the [`wasi:cli/run`
interface](https://github.com/WebAssembly/wasi-cli/blob/main/wit/run.wit), which is the equivalent
of the `main` function to WASI. `cargo-component` will automatically resolve a Rust `bin` package
with a `main` function to a component with `wasi:cli/run` exported. Scaffold a new Wasm application
with a `command` component:

```sh
cargo component new command --command
```

This component will implement the [`app`](https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/tutorial/wit/calculator.wit) world, which
imports the `calculate` interface. In `Cargo.toml`, point `cargo-component` to the WIT file and
specify that it should pull in bindings for the `app` world:

```toml
[package.metadata.component.target]
path = "../path/to/calculator.wit"
world = "app"
```

Now, implement a command line application that:

1. takes in three arguments: two operands and the name of an operator ("1 2 add")
2. parses the operator name and ensures it is supported in the `op` enum
3. calls the `calculate` interface's `eval_expression`, passing in the arguments.

For reference, see a completed [example](https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/tutorial/command/).

## Composing the calculator

Now, we are ready to bring our components together into one runnable calculator component, using
`wasm-tools`. We will first compose the calculator component with the add component to satisfy it's
imports. We then compose that resolved calculator component with the command component to satisfy
its `calculate` imports. The result is a command component that has all its imports satisfied and
exports the `wasi:cli/run` function, which can be executed by `wasmtime`.

```sh
wasm-tools compose calculator.wasm -d adder.wasm -o calculator.wasm
wasm-tools compose command.wasm -d composed.wasm -o command.wasm
```

> If you'd prefer to take a more visual approach to composing components, see the [documentation on
> composing components with
> wasmbuilder.app](creating-and-consuming/composing.md#composing-components-with-a-visual-interface).

## Running the calculator

Now it all adds up! Run the command component with the `wasmtime` CLI, ensuring you are using a
[`v14.0.0 or greater release](https://github.com/bytecodealliance/wasmtime/releases), as earlier releases of
the `wasmtime` command line do not include component model support.

```sh
wasmtime run --wasm component-model command.wasm 1 2 add
1 + 2 = 3
```

## To infinity and beyond!

To expand the exercise to add more components, modify `calculator.wit` to add another operator world
and expand the `op` enum. Then, modify the `command` and `calculator` components to support the
expanded enum.

Another extension of this tutorial could be to remove the `op` enum and instead modify
`eval-expression` to take in a string that can then be parsed to determine which operator component
to call. Maybe this parser is a component of its own?!
