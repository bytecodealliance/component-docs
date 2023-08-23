# Tutorial

If you like to learn by doing, this tutorial will walk through how to build, compose, and run
components through an example of building a calculator component. Calculators can conduct many
operations: add, subtract, multiply, and so on. In this example, each operation will be a component,
that will be composed with an `eval-expression` component that will evaluate the expression using
the expected operator, calling that operator component's function. For starters, the calculator will
have one operator, `add`; however, once the tutorial is complete, you can continue to add more.

## The calculator interface

The WIT package for the calculator consists of a world for each mathematical operator add an `op`
enum that delineates each operator. The following example interface only has an `add` operation:

```wit
package docs:calculator@0.1.0

interface calculate {
    enum op {
        add,
    }
    eval-expression: func(op: op, x: u32, y: u32) -> u32
}

interface add {
    add: func(a: u32, b: u32) -> u32
}

world adder {
    export add
}

world calculator {
    export calculate
    import add
}
```

// TODO: break down the WIT types and point to related documentation

## Create an `add` component

Reference the [language guide](language-support.md) and [authoring components
documentation](creating-and-consuming/authoring.md) to create a component that implements the
`adder` world of `calculator.wit`. For reference, see the completed
[example](../examples/tutorial/adder/).

## Create a `calculator` component

Reference the [language guide](language-support.md) and [authoring components
documentation](creating-and-consuming/authoring.md) to create a component that implements the
`calculator` world of `calculator.wit`. For reference, see the completed
[example](../examples/tutorial/calculator/). The component should import the `add` function from the
`adder` world and call it if the `op` enum matches `add`.

## Crate a `command` component

In order to easily execute the final component with `wasmtime`, we need to create a component that
exports `wasi:cli/command`. This can be done by using `cargo-component` to create a component that
exports the `run` function, which is equivalent of `main` to WASI.

```sh
cargo component new command
```

This component will implement the `app` world, which imports the `calculate` interface from the
`calculator` world.
```sh
package demo:app@0.1.0

world app {
    import docs:calculator/calculate@0.1.0
}                
```

Add this WIT file to a `wit` directory in the project:

```sh
mkdir wit
# add app.wit to /wit
```

Add the calculator world to the component's dependencies in `Cargo.toml`:

```toml
[package.metadata.component.target]
path = "wit"
[package.metadata.component.target.dependencies]
"docs:calculator" = { path = "../wit/calculator.wit" }  
```

Now, implement a command line application that can parse an operator name, ensure it is a supported
`op` and call the `calculator` world's `eval_expression`. For reference, see the completed
[example](../examples/tutorial/command/).


## Composing the calculator

Reference the [documentation on composing components](creating-and-consuming/composing.md) to
compose a command line calculator component with an add operator. Using `wasm-tools`, the three
components can be composed as follows:

```sh
wasm-tools compose calculator.wasm -d adder.wasm -o composed.wasm
wasm-tools compose command.wasm -d composed.wasm -o command.wasm
```

## Running the calculator

Now it all adds up! Run the command component with `wasmtime` CLI, ensuring you are using a dev
release of `wasmtime`, as early releases of the `wasmtime` command line do not include component
model support.

```sh
wasmtime run --wasm-features component-model command.wasm 1 2 add
1 + 2 = 3
```

## To infinity and beyond!

To expand the exercise to add more components, modify the `calculator.wit` to add another operator
world and expand the `op` enum. Then, modify the `command` component to call it.
