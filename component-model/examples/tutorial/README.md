# Building a Calculator of Wasm Components

This tutorial walks through how to compose a component to build a Wasm calculator.
The WIT package for the calculator consists of a world for each mathematical operator
add an `op` enum that delineates each operator. The following example interface only
has an `add` operation:

```wit adder
package docs:adder@0.1.0;


interface add {
    add: func(x: u32, y: u32) -> u32;
}

world adder {
    export add;
}
```

```wit calculator
package docs:calculator@0.1.0;

interface calculate {
    enum op {
        add,
    }
    eval-expression: func(op: op, x: u32, y: u32) -> u32;
}

world calculator {
    export calculate;
    import docs:adder/add;
}

world app {
    import calculate;
}
```

To expand the exercise to add more components, add another operator world, expand the enum, and modify the `command` component to call it.

## Building and running the example

Use [`cargo-component`](https://github.com/bytecodealliance/cargo-component) and [`wac`](https://github.com/bytecodealliance/wac) to build and compose the calculator component.

```sh
(cd calculator && cargo component build --release)
(cd adder && cargo component build --release)
(cd command && cargo component build --release)
wac plug calculator/target/wasm32-wasip1/release/calculator.wasm --plug adder/target/wasm32-wasip1/release/adder.wasm -o composed.wasm
wac plug command/target/wasm32-wasip1/release/command.wasm --plug composed.wasm -o final.wasm
```

Now, run the component with Wasmtime:

```sh
wasmtime run final.wasm 1 2 add
1 + 2 = 3
```

## Composing with the WAC Language

`wac plug` is a convenience to achieve a common pattern in component compositions like above. However, composition can be arbitrarily complicated. In cases where `wac plug` is not sufficient, the WAC language can give us the ability to create arbitrarily complex compositions. To get more experience using the WAC language, let's look at how we could use it to create our composition.

`wac` can compose local components and components hosted in registries. To compose local components, first move the components to a `deps` folder, the default location in which `wac` looks for local components. `wac` infers the subpath to components from the package name of components defined in a WAC file. For example, if the instantiation expression for the adder component in the WAC file is `new docs:adder-impl{}`, the local component is expected to have the following path `deps/docs/adder-impl.wasm`. With this in mind, let's move all out components to a `deps/docs` folder and rename to ease clarifying WAC concepts.

```sh
mkdir -p deps/docs
cp adder/target/wasm32-wasip1/release/adder.wasm deps/docs/adder-impl.wasm
cp calculator/target/wasm32-wasip1/release/calculator.wasm deps/docs/calculator-impl.wasm
cp command/target/wasm32-wasip1/release/command.wasm deps/docs/command-impl.wasm
```

Now we are ready to construct a WAC file to define our composition. Ours instantiates our three components, declaring
which components satisfy each of their imports. It ends with an export of the `wasi:cli/run` interface from the command component. This is the export that the Wasmtime CLI requires in order to execute the final component on the command line.

```wac
// Provide a package name for the resulting composition
package example:composition;

// Instantiate the adder-impl component that implements the adder world.
// We are giving this instance the local name `adder-instance`.
let adder-instance = new docs:adder-impl { };

// Instantiate the calculator-impl component that implements the calculator world.
// In the `new` expression, specify the source of the `add` import to be `adder-instance`'s `add` export.  
let calculator-instance = new docs:calculator-impl { add: adder-instance.add };

// Instantiate a command-impl component that implements the app world.
// The command component might import other interfaces, such as WASI interfaces, but we want to leave  
// those as imports in the final component, so supply `...` to allow those other imports to remain unresolved.  
// The command's exports (in this case, `wasi:cli/run`) remain unaffected in the resulting instance.
let command-instance = new docs:command-impl { calculate: calculator-instance.calculate,... };

// Export the `wasi:cli/run` interface from the command instance
// This could also have been expressed using the postfix access expression `command-instance.run`
export command-instance["wasi:cli/run@0.2.0"];
```

Now, perform your composition by passing the WAC file to `wac compose`.

```sh
wac compose composition.wac -o final.wasm 
```

> Note, instead of moving all the components to a `deps/docs` directory, you can pass the paths to the components inline
> ```sh
> wac compose --dep docs:adder-impl=./adder/target/wasm32-wasip1/release/adder.wasm  --dep docs:calculator-impl=./calculator/target/wasm32-wasip1/release/calculator.wasm --dep docs:command-impl=./command/target/wasm32-wasip1/release/command.wasm  -o final.wasm composition.wac
> ```

Run the component with Wasmtime:

```sh
wasmtime run final.wasm 1 2 add
1 + 2 = 3
```
