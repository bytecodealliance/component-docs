# Building a Calculator of Wasm Components

This tutorial walks through how to compose a component to build a Wasm calculator.
It is a rich example of using multiple components, targeting distinct worlds described across multiple WIT packages

The first package consists of addition operations

```wit
//adder.wit
package component-book:adder@0.1.0;

interface add {
    add: func(a: u32, b: u32) -> u32;
}

world adder {
    export add;
}
```

The WIT package for the calculator consists of a world for each mathematical operator
add an `op` enum that delineates each operator. The following example interface only
has an `add` operation:

```wit
package component-book:calculator@0.1.0;

interface calculate {
    enum op {
        add,
    }
    eval-expression: func(op: op, x: u32, y: u32) -> u32;
}

world calculator {
    export calculate;
    import component-book:adder/add@0.1.0;
}
```

To expand the exercise to add more components, add another operator world, expand the enum, and modify the `command` component to call it.

## Building and running the example

To compose a calculator component with an add operator, run the following:

```sh
(cd calculator && cargo component build --release)
(cd adder && cargo component build --release)
(cd command && cargo component build --release)
wasm-tools compose calculator/target/wasm32-wasi/release/calculator.wasm -d adder/target/wasm32-wasi/release/adder.wasm -o composed.wasm
wasm-tools compose command/target/wasm32-wasi/release/command.wasm -d composed.wasm -o command.wasm
```

You can also use `wac` instead of `wasm-tools compose` if you would like to fetch these components from a registry instead:

```
wac plug component-book:calculator-impl --plug component-book:adder-impl -o composed.wasm && wac plug component-book:command-impl --plug ./composed.wasm -o command.wasm
```

Now, run the component with wasmtime:

```sh
wasmtime run command.wasm 1 2 add
1 + 2 = 3
```
