# Go Tooling

The [TinyGo toolchain](https://tinygo.org/docs/guides/webassembly/wasi/) has native support for WASI
and can build Wasm core modules. With the help of some component model tooling, we can then take
that core module and embed it in a component. To demonstrate how to use the tooling, this guide
walks through building a component that implements the `example` world defined in the [`add.wit`
package](../../examples/example-host/add.wit). The component will implement a simple add function.

## Overview of Building a Component with TinyGo

There are several steps to building a component in TinyGo:

1. Determine which world the component will implement
2. Generate bindings for that world using
   [`wit-bindgen`](https://github.com/bytecodealliance/wit-bindgen#creating-a-component)
3. Implement the interface defined in the bindings
4. Build a Wasm core module using the native TinyGo toolchain
5. Convert the Wasm core module to a component using
   []`wasm-tools`](https://github.com/bytecodealliance/wasm-tools)

The next section will walk through steps 1-4, producing a core Wasm module that targets WASI preview 1.
Then, the following section will walk through converting this core module to a component that
supports WASI preview 2.

## Creating a TinyGo Core Wasm Module

The TinyGo toolchain natively supports compiling Go programs to core Wasm modules. It does have one
key limitation. It currently only supports `main` packages - commands that run start-to-finish and
then exit. Our example program, however, is more like a library which exports an add function that
can be called multiple times; and nothing will ever call its `main` function. To produce a library
("reactor") module from TinyGo requires a little bit of trickery with the Go `init` function, which
we will see shortly. Let's create one that implements the `add` function in the [`example`
world](https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/example-host/add.wit).
First create your Go project:

```sh
mkdir add && cd add
go mod init github.com/yourusername/add
```

Since component creation is not supported natively in TinyGo, we need to generate TinyGo source code
to create bindings to the APIs described by our add WIT package. We can use
[`wit-bindgen`](https://github.com/bytecodealliance/wit-bindgen#creating-a-component) to generate
these bindings for WIT packages. First, install the latest [release of
`wit-bindgen`](https://github.com/bytecodealliance/wit-bindgen/releases). Now, run `wit-bindgen`,
specifying TinyGo as the target language, the path to the
[`add.wit`](../../examples/example-host/add.wit) package, the name of the world in that package to
generate bindings for (`example`), and a directory to output the generated code (`gen`):

```sh
wit-bindgen tiny-go ./add.wit -w example --out-dir=gen
```

The `gen` directory now contains several files:

```sh
$ tree gen
gen
├── example.c
├── example.go
├── example.h
└── example_component_type.o
```

The `example.go` file defines an `Example` interface that matches the structure of our `example`
world. In our Go module, first implement the `Example` interface by defining the `Add` function.

> Note: to resolve the local path to import the bindings from `gen`, update `go.mod` to point to the
> local path to your add Go module:
> ```sh
> echo "replace github.com/yourusername/add => /Path/to/add" >> go.mod
> ```

```go
package main

import (
    . "github.com/yourusername/add/gen"
)

type ExampleImpl struct {
}

// Implement the Example interface to ensure the component satisfies the
// `example` world
func (i ExampleImpl) Add(x, y int32) int32 {
    return x + y
}

// main is required for the `WASI` target, even if it isn't used.
func main() {}
```

Now that we have implemented the example world, we can load it by passing it to the `SetExample`
function. Since our component is a reactor component, `main` will not be called. However, only Go
programs with `main` can target WASI currently. As a loophole, we will initialize our `ExampleImpl`
type inside an `init` function. Go's `init` functions are used to do initialization tasks that
should be done before any other tasks. In this case, we are using it to export the add function and
make it callable using the generated C bindings (`example.c`). After populating the `init` function,
our complete implementation looks similar to the following:

> Note: to resolve the local path to import the bindings from `gen`, update `go.mod` to point to the
> local path to your add Go module:
> ```sh
> echo "replace github.com/yourusername/add => /Path/to/add" >> go.mod
> ```

```go
package main

import (
    . "github.com/yourusername/add/gen"
)

type ExampleImpl struct {
}

// Implement the Example interface to ensure the component satisfies the
// `example` world
func (i ExampleImpl) Add(x, y int32) int32 {
    return x + y
}

// To enable our component to be a `reactor`, implement the component in the 
// `init` function which is always called first when a Go package is run.
func init() {
    example := ExampleImpl{}
    SetExample(example)
}

// main is required for the `WASI` target, even if it isn't used.
func main() {}
```

Now, we can build our core Wasm module, targeting WASI preview 1 using the TinyGo compiler.

```sh
tinygo build -o add.wasm -target=wasi add.go
```

## Converting a Wasm Core Module to a Component

In the previous step, we produced a core module that implements our `example` world. We now want to
convert to a component to gain the benefits of the component model, such as the ability to compose
with it with other components as done in the [`calculator` component in the
tutorial](../tutorial.md#the-calculator-interface). To do this conversion, we will use
[`wasm-tools`](https://github.com/bytecodealliance/wasm-tools), a low level tool for manipulating
Wasm modules. In the future, hopefully most of the functionality of this tool will be embedded
directly into language toolchains (such as is the case with with the Rust toolchain). Download the
latest release from the [project's
repository](https://github.com/bytecodealliance/wasm-tools/releases/tag/wasm-tools-1.0.44).

We also need to download the WASI preview 1 adapter. TinyGo (similar to C) targets preview 1 of WASI
which does not support the component model (`.wit` files). Fortunately, [Wasmtime provides
adapters](https://github.com/bytecodealliance/wit-bindgen#creating-components-wasi) for adapting
preview 1 modules to preview 2 components. There are adapters for both [reactor and command
components](../creating-and-consuming/authoring.md#command-and-reactor-components). Our `add.wit`
world defines a reactor component, so download the `wasi_snapshot_preview1.reactor.wasm` adapter
from [the latest Wasmtime release](https://github.com/bytecodealliance/wasmtime/releases).

Now that we have all the prerequisites downloaded, we can use the `wasm-tools component` subcommand
to componentize our Wasm module, first embedding component metadata inside the core module and then
encoding the module as a component using the WASI preview 1 adapter.

```sh
export COMPONENT_ADAPTER_REACTOR=/path/to/wasi_snapshot_preview1.reactor.wasm
wasm-tools component embed --world example ./add.wit add.wasm -o add.embed.wasm
wasm-tools component new -o add.component.wasm --adapt wasi_snapshot_preview1="$COMPONENT_ADAPTER_REACTOR" add.embed.wasm
```

We now have an add component that satisfies our `example` world, exporting the `add` function, which
we can confirm using another `wasm-tools` command:

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

## Testing an `add` Component

To run our add component, we need to use a host program with a WASI runtime that understands the
`example` world. We've provided an [`example-host`](../../examples/example-host/README.md) to do
just that. It calls the `add` function of a passed in component providing two operands. To use it,
clone this repository and run the Rust program:

```sh
git clone git@github.com:bytecodealliance/component-docs.git
cd component-docs/component-model/examples/example-host
cargo run --release -- 1 2 /path/to/add.component.wasm
```
