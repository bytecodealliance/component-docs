## Language Agnostic Tooling

[`wasm-tools`](https://github.com/bytecodealliance/wasm-tools) provides a suite of subcommands for
working with WebAssembly modules and components.

### WAT (WebAssembly Text Format)

WAT (WebAssembly Text Format) is a text-based language
that can be compiled to the WebAssembly binary format
by `wasm-tools` and other tools.
It's useful for writing small examples for testing and experimentation.

Here's an example of a module expressed in WAT:
```wat
{{#include ../../examples/tutorial/wat/adder/add.wat}}
```

The module contains two top-level declarations, a function and an export.

The function declaration declares a function named `$add`
with two arguments, `$lhs` and `$rhs`.
(Variable names in WAT always start with a `$`.)
Argument and result types need to be provided explicitly.
In this case, the types of both arguments and the result
are `i32` (32-bit integer).
The body of the function is a list of WebAssembly instructions.
The two `local.get` instructions push the values of `$lhs` and `$rhs`
onto the stack.
The `i32.add` instruction pops the two top values off the stack
and adds them, leaving the result on the stack.

The `export` declaration connects the function that was just declared
to a name that should be used for calling it externally.
We want to use this WAT code to implement the interface specified in a WIT file,
so the external name has to follow a certain convention.
The name `"docs:adder/add@0.1.0#add"` can be broken down as follows:
* `docs` is the package name.
* `adder` is the name of a world inside the `docs` package.
* `add` is the name of an interface defined in that world.
* 0.1.0 is a version number.
* Separately, `add` is the name of a function defined in the `add` interface.
All of these pieces come from the specific `.wit` file we are using
(see below).

There's much more than WAT can do;
see the Mozilla Developer Network's [a detailed guide to WAT](https://developer.mozilla.org/en-US/docs/WebAssembly/Guides/Understanding_the_text_format)
for more information.

The [wat2wasm](https://github.com/WebAssembly/wabt) tool converts
from WAT to the binary `.wasm` format,
but it does not create components.

### Building a Component from WAT with `wasm-tools`

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
   Create a file called `add.wat` whose contents are as follows
   (the same as the example in the WAT section):

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

This example was tested with `wasmtime` 34.0.1.
Earlier versions of `wasmtime` may not support the `--invoke` option.
Any other compliant WebAssembly runtime that supports components
can also run this component.
