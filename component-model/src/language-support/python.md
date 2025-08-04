# Python Tooling

## Building a Component with `componentize-py`

[`componentize-py`](https://github.com/bytecodealliance/componentize-py) is a tool that converts a Python
application to a WebAssembly component.

First, install [Python 3.10 or later](https://www.python.org/) and [pip](https://pypi.org/project/pip/) if you don't already have them. Then, install `componentize-py`:

```sh
pip install componentize-py
```

Next, create or download the WIT world you would like to target. For this example we will use an [`adder`
world][adder-wit] with an `add` function (e.g. `wit/component.wit`):

```wit
{{#include ../../examples/tutorial/wit/adder/world.wit}}
```

If you want to generate bindings produced for the WIT world (for an IDE or typechecker), you can generate them using the `bindings` subcommand. Specify the path to the WIT interface with the world you are targeting:

```console
componentize-py --wit-path wit --world adder bindings .
```

> [!NOTE]
> You do not need to generate the bindings in order to `componentize` in the next step. `componentize` will generate bindings on-the-fly and bundle them into the produced component.
>
> If you attempt to run bindings generation twice, it will fail if the `bindings` folder already exists.

Bindings are generated in a folder called `wit_world` by default:

```
<project folder>
├── wit
│   └── component.wit
└── wit_world
    ├── exports
    │   ├── add.py
    │   └── __init__.py
    ├── __init__.py
    └── types.py
```

The `wit_world/exports` folder contains an `Add` protocol which has an `add` method that we can implement,
which represents the export defined in the `adder` world WIT.

To implement the `adder` world (in particular the `add` interface that is exported), put the following code
in a file called `app.py`:

```py
from wit_world import exports

class Add(exports.Adder):
    def add(self, x: int, y: int) -> int:
        return x + y
```

We now can compile our application to a Wasm component using the `componentize` subcommand:

```console
componentize-py \
    --wit-path wit/component.wit \
    --world adder \
    componentize \
    app \
    -o add.wasm
Component built successfully
```

To test the component, run it using the [Rust `add` host](./rust.md#creating-a-command-component-with-cargo-component):

```sh
$ cd component-model/examples/add-host
$ cargo run --release -- 1 2 ../path/to/add.wasm
1 + 2 = 3
```

See [`componentize-py`'s examples](https://github.com/bytecodealliance/componentize-py/tree/main/examples) to try out build HTTP, CLI, and TCP components from Python applications.

## Running components from Python Applications

Wasm components can also be invoked from Python applications. This section walks through using tooling
to call the [pre-built `app.wasm` component][add-wasm] in the examples.

> `wasmtime-py` is only able to run components built with `componentize-py` when the `--stub-wasi` option is used at build time. This is because `wasmtime-py` does not yet support [resources](../design/wit.md#resources), and `componentize-py` by default generates components which use resources from the `wasi:cli` world.  See [this example](https://github.com/bytecodealliance/componentize-py/tree/main/examples/sandbox) of using the `--stub-wasi` option to generate a `wasmtime-py`-compatible component.

First, install [Python 3.11 or later](https://www.python.org/) and [pip](https://pypi.org/project/pip/) if you don't already have them. Then, install [`wasmtime-py`](https://github.com/bytecodealliance/wasmtime-py):

```sh
$ pip install wasmtime
```

First, generate the bindings to be able to call the component from a Python host application.

```sh
# Get an add component that does not import the WASI CLI world
$ wget https://github.com/bytecodealliance/component-docs/raw/main/component-model/examples/example-host/add.wasm
$ python3 -m wasmtime.bindgen add.wasm --out-dir add
```

The generated package `add` has all of the requisite exports/imports for the
component and is annotated with types to assist with type-checking and
self-documentation as much as possible. Inside the package is a `Root` class
with an `add` function that calls the component's exported `add` function. We
can now write a Python program that calls `add`:

```py
from add import Root
from wasmtime import Store

def main():
    store = Store()
    component = Root(store)
    print("1 + 2 =", component.add(store, 1, 2))

if __name__ == '__main__':
    main()
```

Run the Python host program:

```sh
$ python3 host.py
1 + 2 = 3
```

[add-wasm]: https://github.com/bytecodealliance/component-docs/blob/main/component-model/examples/example-host/add.wasm

[adder-wit]: https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/tutorial/wit/adder/world.wit

[!NOTE]: #
