# Rust Host Application for Example Components

This is a native Rust CLI application that can run [WebAssembly Components][wasm-components] of the `adder` world,
defined in [`examples/tutorial/wit/adder/world.wit`][adder-wit], using [WebAssembly Interface Types ("WIT")][wit].

The `adder` world exports an interface called `add` which defines an function that takes two unsigned and adds them:

```wit
package docs:adder@0.1.0;

interface add {
    add: func(x: u32, y: u32) -> u32;
}

world adder {
    export add;
}
```

The application uses WebAssembly ecosystem crates (e.g. [`wasmtime`][wasmtime]) to generate Rust bindings, instantiate WASI worlds, and
executes the exported `add` function (`docs:adder/add.add`) of a provided component.

This host binary takes in two unsigned 32bit integers (`u32`) operands and a path to a component. This host then:

1. Loads the component from the given path
2. Instantiates it as an implementer of the `adder` world
3. Executes the `add` function exported by the component
4. Prints the result

If running with [`cargo`][cargo] (part of the [Rust toolchain][rust-toolchain]), then you should see output like the following:

```
$ cargo run --release -- 1 2 add.wasm
1 + 2 = 3
```

> [!NOTE]
> `add.wasm` is available in this folder, but can be replaced with your own built WebAssembly component
> at any time (written in any language that supports WebAssembly Components), given that it satisfies
> the `adder` world described above.

[wasmtime]: https://github.com/bytecodealliance/wasmtime
[wasm-components]: https://component-model.bytecodealliance.org/design/components.html
[adder-wit]: https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/tutorial/wit/adder/world.wit
[wit]: https://component-model.bytecodealliance.org/design/wit.html
[cargo]: https://doc.rust-lang.org/cargo/
[rust-toolchain]: https://www.rust-lang.org/tools/install
