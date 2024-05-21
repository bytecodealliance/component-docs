# Building a Calculator of Wasm Components

This tutorial walks through how to compose a component to build a Wasm calculator.
This example uses multiple components that target distinct worlds defined across multiple WIT packages.

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

The second WIT package defines the calculator which consists of a world for each mathematical operator
and an `op` enum that delineates each operator. The following example interface only
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

To compose a calculator component with an add operator, you'll first need to install [wac](https://github.com/bytecodealliance/wac), and then you can run the following:

```sh
(cd calculator && cargo component build --release)
(cd adder && cargo component build --release)
(cd command && cargo component build --release)
wac plug local/calculator/target/wasm32-wasi/release/calculator.wasm --plug local/adder/target/wasm32-wasi/release/adder.wasm -o composed.wasm
wac plug command/target/wasm32-wasi/release/command.wasm --plug composed.wasm -o command.wasm
```

For the `wac` commands, if you'd like to fetch example components from the registry for your composition, you can use the following instead:

```
wac plug component-book:calculator-impl --plug component-book:adder-impl -o composed.wasm
wac plug component-book:command-impl --plug ./composed.wasm -o command.wasm
```

Now, run the component with wasmtime:

```sh
wasmtime run command.wasm 1 2 add
1 + 2 = 3
```
