## Language Agnostic Tooling

### Building a Component from WebAssembly Text Format (WAT) with `wasm-tools`

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
  {{#include ../../examples/tutorial/wit/adder/world.wit}}
  ```

3. Define an `add` core module in WAT that exports an `add` function that adds two parameters.
   Create a file called `add.wat` whose contents are as follows:

  ```wat
  {{#include ../../examples/tutorial/wat/adder/add.wat}}
  ```

4. Use `wasm-tools` to create a binary core module with component metadata embedded inside it:

   ```sh
   wasm-tools component embed adder.wit add.wat -o add.wasm
   ```

5. Use `wasm-tools` to create a new component `.wasm` file
   from the binary core module you just created:

   ```sh
   wasm-tools component new add.wasm -o add.component.wasm
   ```

   The suffix `.component.wasm` is just a convention.
   You could also name the output file `add_component.wasm` or anything else
   with the `.wasm` suffix.

### Running a Component with Wasmtime

You can "run" a component by calling one of its exports.
Hosts and runtimes often only support running components with certain exports.

Using the [`wasmtime`](https://github.com/bytecodealliance/wasmtime) CLI,
we can execute the `add` function in the component you just built,
passing in arguments:

```sh
wasmtime run --invoke 'add(1, 2)' add.component.wasm
```

The output is ```3```.
You can try passing other arguments to `add()`
by changing the arguments inside the parentheses.

> [!NOTE]
>
> This example was tested with `wasmtime` 34.0.1.
> Earlier versions of `wasmtime` may not support the `--invoke` option.

Any other compliant WebAssembly runtime that supports components
can also run this component.
