# Go Tooling

The [TinyGo compiler](https://tinygo.org/) v0.34.0 and above has native support for the WebAssembly Component Model and WASI 0.2.0.

This guide walks through building a component that implements `adder` world defined in the [`adder/world.wit` package][adder-wit].
The component will implement the `adder` world, which contains `add` interface with a `add` function.

[adder-wit]: https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/tutorial/wit/adder/world.wit

## 1. Install the tools

Follow the [TinyGo installation instructions](https://tinygo.org/getting-started/) to install the TinyGo compiler.

Additionally, install the `wasm-tools` CLI tool from the [wasm-tools repository](https://github.com/bytecodealliance/wasm-tools/releases).

> [!WARNING]
> Due to some upstream issues, only `wasm-tools` versions 1.225.0 or earlier can be used with `wit-bindgen-go`
>
> If using the Rust toolchain to install `wasm-tools`, it can be installed like so:
> `cargo install --locked wasm-tools@1.225.0 --force`

To verify the installation, run the following commands:

```
$ tinygo version
tinygo version 0.34.0 ...
$ wasm-tools -V
wasm-tools 1.255.0 ...
```

Optional: Install the [`wkg`][wkg] CLI tool to resolve the imports in the WIT file. The `wkg` CLI is a part of the [Wasm Component package manager](https://github.com/bytecodealliance/wasm-pkg-tools/releases)

[wkg]: https://github.com/bytecodealliance/wasm-pkg-tools/tree/main/crates/wkg

## 2. Create your Go project

Now, create your Go project:

```console
mkdir add && cd add
go mod init example.com
```

Ensure that the following `tool`s are installed:

```
tool (
	go.bytecodealliance.org/cmd/wit-bindgen-go
)
```

> [!NOTE]
> `go tool` was introduced in [Golang 1.24][go-1-24-release] and can be used to manage tooling in Go projects.

Consider also running `go mod tidy` after adding the above tool.

[go-1-24-release]: https://go.dev/blog/go1.24

## 2. Determine which World the Component will Implement

Since we will be implementing the [`adder` world][adder-wit], we can copy the WIT to our project,
under the `wit` folder (e.g. `wit/component.wit`):

```wit
package docs:adder@0.1.0;

interface add {
    add: func(x: u32, y: u32) -> u32;
}

world adder {
    export add;
}
```

The `wasip2` target of TinyGo assumes that the component is targeting `wasi:cli/command@0.2.0` world
(part of [`wasi:cli`][wasi-cli]) so it requires the imports of `wasi:cli/imports@0.2.0`.

We need to include those interfaces as well in `component.wit`, by editing the `adder` world:

```wit
world adder {
  include wasi:cli/imports@0.2.0;
  export add;
}
```

### Using `wkg` to automatically resolve and download imports

Tools like [`wkg`][wkg] can be convenient to build a complete WIT package by resolving the imports.

Running the `wkg wit fetch` command will resolve the imports and populate your `wit` folder with all relevant
imported namespaces and packages.

```
$ wkg wit build
WIT package written to docs:adder@0.1.0.wasm
```

[wasi-cli]: https://github.com/WebAssembly/wasi-cli

## 3. Generate bindings for the Wasm component

Now that we have our WIT definitions bundled together into a WASM file,
we can generate the bindings for our Wasm component, by adding a build directive:

```console
go tool wit-bindgen-go generate --world adder --out internal ./docs:adder@0.1.0.wasm
```

> [!NOTE]
> The `go tool` directive (added in [Golang 1.24][go-1-24-release]) installs and enables use of `wit-bindgen-go`,
> part of the Bytecode Alliance suite of Golang tooling.

The `internal` directory will contain the generated Go code that WIT package.

```console
$ tree internal
internal
├── docs
│   └── adder
│       ├── add
│       │   ├── add.exports.go
│       │   ├── add.wasm.go
│       │   ├── add.wit.go
│       │   └── empty.s
│       └── adder
│           └── adder.wit.go
└── wasi
    ├── cli
    │   ├── environment
    │   │   ├── empty.s
    │   │   ├── environment.wasm.go
    │   │   └── environment.wit.go
    │   ├── exit
    │   │   ├── empty.s
    │   │   ├── exit.wasm.go
    │   │   └── exit.wit.go
    │   ├── stderr
    │   │   ├── empty.s
    │   │   ├── stderr.wasm.go
    │   │   └── stderr.wit.go
    │   ├── stdin
    │   │   ├── empty.s
    │   │   ├── stdin.wasm.go
    │   │   └── stdin.wit.go
    │   ├── stdout
    │   │   ├── empty.s
    │   │   ├── stdout.wasm.go
    │   │   └── stdout.wit.go
    │   ├── terminal-input
    │   │   ├── empty.s
    │   │   ├── terminal-input.wasm.go
    │   │   └── terminal-input.wit.go
    │   ├── terminal-output
    │   │   ├── empty.s
    │   │   ├── terminal-output.wasm.go
    │   │   └── terminal-output.wit.go
    │   ├── terminal-stderr
    │   │   ├── empty.s
    │   │   ├── terminal-stderr.wasm.go
    │   │   └── terminal-stderr.wit.go
    │   ├── terminal-stdin
    │   │   ├── empty.s
    │   │   ├── terminal-stdin.wasm.go
    │   │   └── terminal-stdin.wit.go
    │   └── terminal-stdout
    │       ├── empty.s
    │       ├── terminal-stdout.wasm.go
    │       └── terminal-stdout.wit.go
    ├── clocks
    │   ├── monotonic-clock
    │   │   ├── empty.s
    │   │   ├── monotonic-clock.wasm.go
    │   │   └── monotonic-clock.wit.go
    │   └── wall-clock
    │       ├── empty.s
    │       ├── wall-clock.wasm.go
    │       └── wall-clock.wit.go
    ├── filesystem
    │   ├── preopens
    │   │   ├── empty.s
    │   │   ├── preopens.wasm.go
    │   │   └── preopens.wit.go
    │   └── types
    │       ├── abi.go
    │       ├── empty.s
    │       ├── types.wasm.go
    │       └── types.wit.go
    ├── io
    │   ├── error
    │   │   ├── empty.s
    │   │   ├── error.wasm.go
    │   │   └── error.wit.go
    │   ├── poll
    │   │   ├── empty.s
    │   │   ├── poll.wasm.go
    │   │   └── poll.wit.go
    │   └── streams
    │       ├── empty.s
    │       ├── streams.wasm.go
    │       └── streams.wit.go
    ├── random
    │   ├── insecure
    │   │   ├── empty.s
    │   │   ├── insecure.wasm.go
    │   │   └── insecure.wit.go
    │   ├── insecure-seed
    │   │   ├── empty.s
    │   │   ├── insecure-seed.wasm.go
    │   │   └── insecure-seed.wit.go
    │   └── random
    │       ├── empty.s
    │       ├── random.wasm.go
    │       └── random.wit.go
    └── sockets
        ├── instance-network
        │   ├── empty.s
        │   ├── instance-network.wasm.go
        │   └── instance-network.wit.go
        ├── ip-name-lookup
        │   ├── abi.go
        │   ├── empty.s
        │   ├── ip-name-lookup.wasm.go
        │   └── ip-name-lookup.wit.go
        ├── network
        │   ├── abi.go
        │   ├── empty.s
        │   ├── network.wasm.go
        │   └── network.wit.go
        ├── tcp
        │   ├── abi.go
        │   ├── empty.s
        │   ├── tcp.wasm.go
        │   └── tcp.wit.go
        ├── tcp-create-socket
        │   ├── empty.s
        │   ├── tcp-create-socket.wasm.go
        │   └── tcp-create-socket.wit.go
        ├── udp
        │   ├── abi.go
        │   ├── empty.s
        │   ├── udp.wasm.go
        │   └── udp.wit.go
        └── udp-create-socket
            ├── empty.s
            ├── udp-create-socket.wasm.go
            └── udp-create-socket.wit.go

39 directories, 91 files
```

The `adder.exports.go` file contains the exported functions that need to be implemented in the Go code called `Exports`.

## 4. Implement the `add` Function

```Go
//go:generate go tool wit-bindgen-go generate --world adder --out internal ./docs:adder@0.1.0.wasm

package main

import (
	"example.com/internal/docs/adder/add"
)

func init() {
	add.Exports.Add = func(x uint32, y uint32) uint32 {
		return x + y
	}
}

// main is required for the `wasi` target, even if it isn't used.
func main() {}
```

Go's `init` functions are used to do initialization tasks that
should be done before any other tasks. In this case, we are using it to export the `Add` function.

## 5. Build the Component

We can build our component using TinyGo by specifying the wit-package to be `add.wit` and the WIT world to be `adder`.

Under the hood, TinyGo invokes `wasm-tools` to embed the WIT file to the module and componentize it.

```console
tinygo build -target=wasip2 -o add.wasm --wit-package docs:adder@0.1.0.wasm --wit-world adder main.go
```

> **WARNING:** By default, tinygo includes all debug-related information in your .wasm file. That is desirable when prototyping or testing locally to obtain useful backtraces in case of errors (for example, with `wasmtime::WasmBacktraceDetails::Enable`). To remove debug data and optimize your binary file, build with `-no-debug`. The resulting .wasm file will be considerably smaller (up to 75% reduction in size).

We now have an add component that satisfies our `adder` world, exporting the `add` function, which

We can confirm using the `wasm-tools component wit` command:

```console
$ wasm-tools component wit add.wasm
package root:component;

world root {
  import wasi:io/error@0.2.0;
  import wasi:io/streams@0.2.0;
  import wasi:cli/stdout@0.2.0;
  import wasi:random/random@0.2.0;

  export add: func(x: s32, y: s32) -> s32;
}
...
```

## 5. Testing the `add` Component

To run our add component, we need to use a host program with a WASI runtime that understands the
`example` world -- we've provided an [`example-host`][example-host] that does just that.

The example host calls the `add` function of a passed in component providing two operands.

To use the example host, clone this repository and run the Rust program:

```console
git clone git@github.com:bytecodealliance/component-docs.git
cd component-docs/component-model/examples/example-host
cargo run --release -- 1 2 /path/to/add.wasm
```

[example-host]: https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/example-host

[!NOTE]: #
[!WARNING]: #
