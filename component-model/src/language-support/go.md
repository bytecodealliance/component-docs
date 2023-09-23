# Go Tooling

The [TinyGo toolchain](https://tinygo.org/docs/guides/webassembly/wasi/) has native support for WASI and with the help of the WASI preview 1 adapter we can create Go components.

## Building a Component with TinyGo

The TinyGo toolchain currently only supports `main` packages even if the exported `main` function is not used. Let's create one that implements the `add` function in the [`example`
world](https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/example-host/add.wit). First create your Go project:

```sh
mkdir add && cd add
go mod init github.com/yourusername/yourproject
```

Now, let's generate our TinyGo bindings using `wit-bindgen`:

```sh
wit-bindgen tiny-go ./add.wit -w example --out-dir=gen
```

Now, we can implement the generated `SetExample` type (in `gen/example.go`) that is exported by our component:

> Note: to resolve the local path to import the bindings from `gen`, update `go.mod`:
> ```sh
> echo "replace github.com/yourusername/yourproject => /Path/to/add" >> go.mod
> ```

```go
package main

//go:generate wit-bindgen tiny-go ./add.wit -w example --out-dir=gen

import (
    . "github.com/yourusername/yourproject/gen"
)

type AddImpl struct {
}

func init() {
    a := AddImpl{}
    SetExample(a)
}

func (i AddImpl) Add(x, y int32) int32 {
    return x + y
}

// main is required for the `WASI` target, even if it isn't used.
func main() {}
```

Now, we can build our Wasm module, targeting WASI:

```sh
    tinygo build -o add.wasm -target=wasi add.go
```

TinyGo (similar to C) targets preview 1 of WASI which does not support the component model (`.wit` files). Fortunately, [Wasmtime provides adapters](https://github.com/bytecodealliance/wit-bindgen#creating-components-wasi) for adapting preview 1 modules to components. There is an adaptor for both [reactor and command components](../creating-and-consuming/authoring.md#command-and-reactor-components). Our `add.wit` world defines a reactor component, so download the `wasi_snapshot_preview1.reactor.wasm` adaptor from [a Wasmtime release](https://github.com/bytecodealliance/wasmtime/releases).

We will use `wasm-tools`to componetize our Wasm module, first embedding component metadata inside the core module and then encoding the module as a component using the WASI preview 1 adapter.

```sh
wasm-tools component embed --world example ./add.wit add.wasm -o add.embed.wasm
wasm-tools component new -o add.component.wasm --adapt wasi_snapshot_preview1="$COMPONENT_ADAPTOR_REACTOR" add.embed.wasm
```

We now have an add component!

```sh
$ wasm-tools component wit add.component.wit
package root:component

world root {
  import wasi:io/streams
  import wasi:filesystem/types
  import wasi:filesystem/preopens
  import wasi:cli/stdin
  import wasi:cli/stdout
  import wasi:cli/stderr
  import wasi:cli/terminal-input
  import wasi:cli/terminal-output
  import wasi:cli/terminal-stdin
  import wasi:cli/terminal-stdout
  import wasi:cli/terminal-stderr

  export add: func(x: s32, y: s32) -> s32
}
```
