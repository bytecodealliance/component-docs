## Language Agnostic Tooling

### Building a Component with `wasm-tools`

[`wasm-tools`](https://github.com/bytecodealliance/wasm-tools) provides a suite of subcommands for
working with WebAssembly modules and components.

`wasm-tools` can be used to create a component from WAT.
Here's how to create a component from WAT
that implements the [`adder` world](https://github.com/bytecodealliance/component-docs/blob/main/component-model/examples/tutorial/wit/adder/world.wit)
and simply adds two numbers.

1. Install [`wasm-tools`](https://github.com/bytecodealliance/wasm-tools/tree/main#installation), a
   tool for low-level manipulation of Wasm modules and components.
2. The `add` function is defined inside the following world.
   Create a file called `adder.wit` whose contents are as follows:

   ```wit
   package docs:adder@0.1.0;

   interface add {
       add: func(x: u32, y: u32) -> u32;
   }

   world adder {
       export add;
   }
   ```

3. Define an `add` core module in WAT that exports an `add` function that adds two parameters.
   Create a file called `add.wat` whose contents are as follows:

   ```wat
   (module
     (func $add (param $lhs i32) (param $rhs i32) (result i32)
         local.get $lhs
         local.get $rhs
         i32.add)
     (export "docs:adder/add@0.1.0#add" (func $add))
   )
   ```

4. Use `wasm-tools` to create a binary core module with component metadata embedded inside it:

   ```sh
   $ wasm-tools component embed adder.wit add.wat -o add.wasm
   ```

5. Use `wasm-tools` to create a new component `.wasm` file
   from the binary core module you just created:

   ```sh
   $ wasm-tools component new add.wasm -o add.component.wasm
   ```

### Running a Component with Wasmtime

You can "run" a component by calling one of its exports. Hosts and runtimes often only support
running components with certain exports. The [`wasmtime`](https://github.com/bytecodealliance/wasmtime) CLI can only run "command" components, so in
order to run the `add` function above, it first must be composed with a primary "command" component
that calls it. See [documentation on running components](../running-components/wasmtime.md) for more details.

