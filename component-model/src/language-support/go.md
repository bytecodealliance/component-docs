# Go Tooling

The [TinyGo toolchain](https://tinygo.org/docs/guides/webassembly/wasi/) has native support for WASI
and can build Wasm core modules. With the help of some component model tooling, we can then take
that core module and embed it in a component. To demonstrate how to use the tooling, this guide
walks through building a component that implements the `example` world defined in the [`add.wit`
package](../../examples/example-host/add.wit). The component will implement a simple add function.

## Overview of Building a Component with TinyGo

There are several steps to building a component in TinyGo:

1. Determine which world the component will implement
2. Build a Wasm core module using the native TinyGo toolchain
3. Convert the Wasm core module to a component using
   []`wasm-tools`](https://github.com/bytecodealliance/wasm-tools)

The next section will walk through steps 1-2, producing a core Wasm module that targets WASI preview 1.
Then, the following section will walk through converting this core module to a component that
supports WASI preview 2.

## Creating a TinyGo Core Wasm Module

The TinyGo toolchain natively supports compiling Go programs to core Wasm modules. Let's create one that implements the `add` function in the [`example`
world](https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/example-host/add.wit).

First, implement a simple add function in `add.go`:

```go
package main

//go:wasm-module yourmodulename
//export add
func add(x, y int32) int32 {
	return x + y
}

// main is required for the `wasi` target, even if it isn't used.
func main() {}
```

Note, we must still provide a `main` function. This is a limitation of TinyGo's support of WASI as it currently only supports `main` packages - commands that run start-to-finish and
then exit. Our example program, however, is more like a library which exports an add function that
can be called multiple times; and nothing will ever call its `main` function.

Now, we can use TinyGo to build our core Wasm module:

```sh
tinygo build -o add.wasm -target=wasi add.go
```

You should now have an `add.wasm` module. But at the moment, this is a core module. In the next section, we will convert it into a component.

## Converting a Wasm Core Module to a Component

In the previous step, we produced a core module that implements our `example` world. We now want to
convert to a component to gain the benefits of the component model, such as the ability to compose
with it with other components as done in the [`calculator` component in the
tutorial](../tutorial.md#the-calculator-interface). 
TinyGo is actively developing a `wasip2` target (in this [PR](https://github.com/tinygo-org/tinygo/pull/4027)), but for now we must take additional steps to convert the module to a component.

We will use
[`wasm-tools`](https://github.com/bytecodealliance/wasm-tools), a low level tool for manipulating
Wasm modules. Download the
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

## Targetting Worlds with Interfaces with TinyGo and Wit-Bindgen

The [`example`
world](https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/example-host/add.wit) we were using in the previous sections simply exports a function. However, to use your component from another component, it must export an interface. This means we will need to use a tool to generate bindings to use as glue code, and adds a couple more steps (2-3) to building Wasm components with TinyGo:

1. Determine which world the component will implement
2. Generate bindings for that world using
   [`wit-bindgen`](https://github.com/bytecodealliance/wit-bindgen#creating-a-component)
3. Implement the interface defined in the bindings
4. Build a Wasm core module using the native TinyGo toolchain
5. Convert the Wasm core module to a component using
   []`wasm-tools`](https://github.com/bytecodealliance/wasm-tools)

For this example, we will use the following world, which moves the add function behind an `add` interface:

```wit
package docs:adder@0.1.0;

interface add {
    add: func(a: u32, b: u32) -> u32;
}

world adder {
    export add;
}
```

Our new steps use a low-level tool, [`wit-bindgen`](https://github.com/bytecodealliance/wit-bindgen?tab=readme-ov-file#guest-tinygo) to generate bindings, or wrapper code, for implementing the desired world.

First, install [a release of `wit-bindgen`](https://github.com/bytecodealliance/wit-bindgen/releases), updating the environment variables for your desired version, architecture and OS:

```sh
export VERSION=0.24.0 ARCH=aarch64 OS=macos
wget https://github.com/bytecodealliance/wit-bindgen/releases/download/v$VERSION/wit-bindgen-$VERSION-$ARCH-$OS.tar.gz
tar -xzf wit-bindgen-$VERSION-$ARCH-$OS.tar.gz
mv wit-bindgen-$VERSION-$ARCH-$OS/wit-bindgen ./
rm -rf wit-bindgen-$VERSION-$ARCH-$OS.tar.gz wit-bindgen-$VERSION-$ARCH-$OS
```

Now, create your Go project:

```sh
mkdir add && cd add
go mod init example.com
```

Next, run `wit-bindgen`, specifying TinyGo as the target language, the path to the
[`add.wit`](../../examples/example-host/add.wit) package, the name of the world in that package to
generate bindings for (`example`), and a directory to output the generated code (`gen`):

```sh
wit-bindgen tiny-go ./add.wit --world example --out-dir=gen
```

The `gen` directory now contains several files:

```sh
$ tree gen
gen
├── adder.c
├── adder.go
└── adder.h
```

The `adder.go` file defines an `ExportsDocsAdder0_1_0_Add` interface that matches the structure of our `add`
interface. The name of the interface is taken from the WIT package name (`docs:adder@0.1.0`) combined with the interface name (`add`). In our Go module, first implement the `ExportsDocsAdder0_1_0_Add` interface by defining the `Add` function.

```go
package main

import (
	. "example.com/gen"
)

type AdderImpl struct {
}

// Implement the `ExportsDocsAdder0_1_0_Add` interface to ensure the component satisfies the
// `adder` world
func (i AdderImpl) Add(x, y uint32) uint32 {
	return x + y
}

// main is required for the `wasi` target, even if it isn't used.
func main() {}
```

After implementing the adder world, we need to load it by passing it to the `SetExportsDocsAdder0_1_0_Add`
function from our bindings (`adder.go`). Since our component is a library, `main` will not be called. However, only Go
programs with `main` can target WASI currently. As a loophole, we will initialize our `AdderImpl`
type inside an `init` function. Go's `init` functions are used to do initialization tasks that
should be done before any other tasks. In this case, we are using it to export the `Add` function and
make it callable using the generated C bindings (`adder.c`). After populating the `init` function,
our complete implementation looks similar to the following:

```go
package main

import (
	. "example.com/gen"
)

type AdderImpl struct {
}

// Implement the ExportsDocsAdder0_1_0_Add interface to ensure the component satisfies the
// `adder` world
func (i AdderImpl) Add(x, y uint32) uint32 {
	return x + y
}

// To enable our component to be a library, implement the component in the
// `init` function which is always called first when a Go package is run.
func init() {
	example := AdderImpl{}
	SetExportsDocsAdder0_1_0_Add(example)
}

// main is required for the `WASI` target, even if it isn't used.
func main() {}
```

Once again, we can build our core module using TinyGo, componentize it, and adapt it for WASI 0.2:
```sh
export COMPONENT_ADAPTER_REACTOR=/path/to/wasi_snapshot_preview1.reactor.wasm
tinygo build -o add.wasm -target=wasi add.go
wasm-tools component embed --world example ./add.wit add.wasm -o add.embed.wasm
wasm-tools component new -o add.component.wasm --adapt wasi_snapshot_preview1="$COMPONENT_ADAPTER_REACTOR" add.embed.wasm
```

We now have an add component that satisfies our `adder` world, exporting the `add` function, which
we can confirm using the `wasm-tools component wit` command:

```sh
wasm-tools component wit add.component.wasm 
package root:component;

world root {
  import wasi:io/error@0.2.0;
  import wasi:io/streams@0.2.0;
  import wasi:cli/stdin@0.2.0;
  import wasi:cli/stdout@0.2.0;
  import wasi:cli/stderr@0.2.0;
  import wasi:clocks/wall-clock@0.2.0;
  import wasi:filesystem/types@0.2.0;
  import wasi:filesystem/preopens@0.2.0;

  export docs:adder/add@0.1.0;
}
```
