# Wasm Language Support

WebAssembly can be targeted by the majority of top programming
languages; however, the level of
support varies. This document details the subset of languages that target WASI and support
components. This is a living document, so if you are aware of advancements in a toolchain, please do
not hesitate to [contribute documentation](contributing.md). Each section covers how to build and
run components for a given toolchain.

One of the benefits of components is their portability across host runtimes. The runtime only needs
to know what world the component is targeting in order to import or execute the component. This
language guide hopes to demonstrate that with a prevailing `example` world defined in
[`../examples/add.wit`](../examples/add.wit). Furthermore, an example host that understands the `example`
world has been provided in [`../examples/add-host`](../examples/add-host/) for running components. Each
toolchain section walks through creating a component of this world, which can be run either in the
example host or from an application of that toolchain. This aims to provide a full story for using
components within and among toolchains.
 
- [Wasm Language Support](#wasm-language-support)
  - [Language Agnostic Tooling](#language-agnostic-tooling)
    - [Building a Component with `wasm-tools`](#building-a-component-with-wasm-tools)
    - [Running a Component with Wasmtime](#running-a-component-with-wasmtime)
  - [Rust Tooling](#rust-tooling)
    - [Building a Component with `cargo component`](#building-a-component-with-cargo-component)
    - [Running a Component from Rust Applications](#running-a-component-from-rust-applications)
  - [JavaScript Tooling](#javascript-tooling)
    - [Building a Component with `jco`](#building-a-component-with-jco)
    - [Running a Component from JavaScript Applications](#running-a-component-from-javascript-applications)
  - [Python Tooling](#python-tooling)
    - [Building a Component with `componentize-py`](#building-a-component-with-componentize-py)
    - [Running components from Python Applications](#running-components-from-python-applications)

## Language Agnostic Tooling

### Building a Component with `wasm-tools`

[`wasm-tools`](https://github.com/bytecodealliance/wasm-tools) provides a suite of subcommands for
working with WebAssembly modules and components.

`wasm-tools` can be used to create a component from WebAssembly Text (WAT). This walks through creating a component from WAT that implements the [`example` world](../examples/add.wit) and simply adds two numbers.

1. Install [`wasm-tools`](https://github.com/bytecodealliance/wasm-tools/tree/main#installation), a
   tool for low-level manipulation of Wasm modules and components.
2. The `add` function is defined inside the following `example` world:

    ```wit
    package example:component

    world example {
        export add: func(x: s32, y: s32) -> s32
    }
    ```

3. Define an `add` core module in WAT that exports an `add` function that adds two parameters:
  
    ```wat
    (module
      (func $add (param $lhs i32) (param $rhs i32) (result i32)
          local.get $lhs
          local.get $rhs
          i32.add)
      (export "add" (func $add))
    )
    ```

4. Use `wasm-tools` to create a component from the core module, first embedding component metadata
   inside the core module and then encoding the WAT to a Wasm binary.

    ```sh
    $ wasm-tools component embed add.wit add.wat -o add.wasm
    $ wasm-tools component new add.wasm -o add.component.wasm
    ```

### Running a Component with Wasmtime

Coming soon. Work is currently underway to enable running components from the [`wasmtime`](https://github.com/bytecodealliance/wasmtime) CLI.

## Rust Tooling

### Building a Component with `cargo component`

[`cargo-component`](https://github.com/bytecodealliance/cargo-component) is a `cargo` subcommand for
creating WebAssembly components using Rust as the component's implementation language.

Let's create the same `add` component using the `cargo-component` tooling. First scaffold a project:
```sh
$ cargo component new add --reactor && cd add
```

Update `wit/world.wit` to match `add.wit` and modify the component package reference to change the
package name to `example`. The `component` section of `Cargo.toml` should look like the following:

```toml
[package.metadata.component]
package = "component:example"
```

Implement the `add` function in `add/src/lib.rs`. It should look similar to the following:

```rs
impl Example for Component {
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
}
```

Now, build the component:

```sh
$ cargo component build --release
```

You can use `wasm-tools component wit` to output the WIT package of the component:

```sh
$ wasm-tools component wit add/target/wasm32-wasi/release/add.wasm
package root:component

world root {
  export add: func(x: s32, y: s32) -> s32
}
```

### Running a Component from Rust Applications

To verify that our component works, lets run it from a Rust application that knows how to import a
component of the [`example` world](../examples/add.wit).

The application uses [`wasmtime`](https://github.com/bytecodealliance/wasmtime) crates to generate
Rust bindings, bring in WASI worlds, and execute the component.

```sh
$ cd examples/add-host
$ cargo run -- 1 2 ../add/target/wasm32-wasi/release/add.wasm
1 + 2 = 3
```

## JavaScript Tooling

[`jco`](https://github.com/bytecodealliance/jco) is a fully native JS tool for working with the
emerging WebAssembly Components specification in JavaScript.

### Building a Component with `jco`

A component can be created from a JS module using `jco componentize`. First, install `jco` and
`componentize-js`:

```sh
$ npm install @bytecodealliance/jco
$ npm install @bytecodealliance/componentize-js
```

Create a JavaScript module that implements the `add` function in [`add.wit`](../examples/add.wit):

```js 
export function add (x, y) {
    return x + y;
}
```

Now, use `jco` to create a component from the JS module:

```sh
$ jco componentize add.js --wit add.wit -n example -o add.wasm
OK Successfully written add.wasm with imports ().
```

Now, run the component using the Rust `add` host:

> Note: it can take over 20 seconds to complete

```sh
$ cd component-model/examples/add-host
$ cargo run -- 1 2 ../path/to/add.wasm
1 + 2 = 3
```

### Running a Component from JavaScript Applications

As the JavaScript runtime cannot yet execute Wasm components, a component must be transpiled into
JavaScript and a core module and then executed. `jco` automates this transpilation:

```sh
$ jco transpile add.wasm -o out-dir

Transpiled JS Component Files:

 - out-dir/add.core.wasm  6.72 MiB
 - out-dir/add.d.ts       0.05 KiB
 - out-dir/add.js          0.8 KiB
```

A core module and JavaScript bindings have been outputted to the `out-dir`.

Now, you can import the resultant `add.js` file and run it from a JavaScript application. This
example renames it and imports it as an ECMAScript module for ease of running locally with node:

```mjs
// app.mjs
import { add } from './out-dir/add.mjs';

console.log("1 + 2 = " + add(1, 2));
```

The above example :

```sh
$ mv out-dir/add.js out-dir/add.mjs
$ node app.mjs
1 + 2 = 3
```

## Python Tooling

### Building a Component with `componentize-py`

[`componentize-py`](https://github.com/dicej/componentize-py) is a tool that converts a Python
application to a WebAssembly component.

Create a Python program that implements the `add` function in the [`example`
world](../examples/add.wit). Note that it imports the bindings that will be created by
`componentize-py`:

```sh
$ cat<<EOT >> guest.py 
import example

class Example(example.Example):
    def add(x: int, y: int) -> int:
        return x + y
EOT
```

[Install `componentize-py`](https://github.com/dicej/componentize-py#installing-from-pypi) and
generate a component from `guest.py`.

```sh
$ pip install componentize-py
$ componentize-py -d ../examples/add.wit -w example componentize guest -o add.wasm 
Component built successfully
```

To test the component, run it using the Rust `add` host:

> Note: it can take over 30 seconds to complete

```sh
$ cd component-model/examples/add-host
$ cargo run -- 1 2 ../path/to/add.wasm
1 + 2 = 3
```

### Running components from Python Applications

Wasm components can also be invoked from Python applications. This walks through the tooling needed
to call the `app.wasm` component from the previous section from a Python application. First, install
`wasmtime-py`, being sure to use a version [this PR has
merged](https://github.com/bytecodealliance/wasmtime-py/pull/171) or working off that branch.

> Note: be sure to use at least Python 3.11 

```sh
$ git clone https://github.com/dicej/wasmtime-py
$ (cd wasmtime-py && python ci/download-wasmtime.py && python ci/build-rust.py && pip install .)
```

Now, generate the bindings to be able to call the component from a Python host application.

```sh
$ python3 -m wasmtime.bindgen add.wasm --out-dir add
```

The generated package `add` has all of the requisite exports/imports for the component and is
annotated with types to assist with type-checking and self-documentation as much as possible.

Now, create a Python program to run the component. Note that imports for WASI preview 2 are
explicitly set to null. This is because when creating a component from a Python module,
`componentize-py` pulls in extra WASI Preview 2 imports, even if they are not used by the component.
Currently, language toolchains are likely to pull in more than a component declares in WAT.

```py
from add import Root, RootImports
from wasmtime import Store

def main():
    store = Store()
    component = Root(store, RootImports(language=Host(), poll=None, monotonic_clock=None, wall_clock=None, streams=None, filesystem=None, random=None, environment=None, preopens=None, exit=None, stdin=None, stdout=None, stderr=None))
    print("1 + 2 = ", component.add(store, 1, 2))

if __name__ == '__main__':
    main()
```

Run the Python host program:

```sh
$ python3 host.py
1 + 2 = 3
```
