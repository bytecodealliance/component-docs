# Python Tooling

### Building a Component with `componentize-py`

[`componentize-py`](https://github.com/dicej/componentize-py) is a tool that converts a Python
application to a WebAssembly component.

Create a Python program that implements the `add` function in the [`example`
world](https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/example-host/add.wit). Note that it imports the bindings that will be created by
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
$ componentize-py -d /path/to/examples/example-host/add.wit -w example componentize guest -o add.wasm
Component built successfully
```

To test the component, run it using the [Rust `add` host](./rust.md#creating-a-command-component-with-cargo-component):

```sh
$ cd component-model/examples/add-host
$ cargo run --release -- 1 2 ../path/to/add.wasm
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
    component = Root(store, RootImports(poll=None, monotonic_clock=None, wall_clock=None, streams=None, filesystem=None, random=None, environment=None, preopens=None, exit=None, stdin=None, stdout=None, stderr=None))
    print("1 + 2 = ", component.add(store, 1, 2))

if __name__ == '__main__':
    main()
```

Run the Python host program:

```sh
$ python3 host.py
1 + 2 = 3
```
