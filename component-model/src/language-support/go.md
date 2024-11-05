# Go tooling

The [TinyGo compiler](https://tinygo.org/) v0.33.0 and above has native support for the WebAssembly Component Model and WASI 0.2.0. This guide walks through building a component that implements `example` world defined in the [`add.wit`
package](../../examples/example-host/add.wit). The component will implement a simple add function.

## 1. Install the TinyGo compiler and `wasm-tools`

Follow the [TinyGo installation instructions](https://tinygo.org/getting-started/) to install the TinyGo compiler. Additionally, install the `wasm-tools` CLI tool from the [wasm-tools repository](https://github.com/bytecodealliance/wasm-tools/releases).

To verify the installation, run the following commands:

```sh
$ tinygo version
$ wasm-tools -V
```

## 2. Determine which world the component will implement

The `wasip2` target of TinyGo requires the imports of `wasi:cli/imports@0.2.0` and thus we need to include them in the `add.wit`. Below is the minimal `add.wit` file that includes the required imports:

```wit
package docs:adder@0.1.0;

world adder {
  import wasi:io/error@0.2.0;
  import wasi:io/streams@0.2.0;
  import wasi:cli/stdout@0.2.0;
  import wasi:random/random@0.2.0;

  export add: func(x: s32, y: s32) -> s32;
}

package wasi:io@0.2.0 {
  interface error {
    resource error;
  }
  interface streams {
    use error.{error};

    resource output-stream {
      blocking-write-and-flush: func(contents: list<u8>) -> result<_, stream-error>;
    }

    variant stream-error {
      last-operation-failed(error),
      closed,
    }
  }
}

package wasi:cli@0.2.0 {
  interface stdout {
    use wasi:io/streams@0.2.0.{output-stream};

    get-stdout: func() -> output-stream;
  }
}


package wasi:random@0.2.0 {
  interface random {
    get-random-u64: func() -> u64;
  }
}
```

Now, create your Go project:

```sh
$ mkdir add && cd add
$ go mod init example.com
```

Next, we can generate the bindings for the `add.wit` file:

```sh
$ go run github.com/bytecodealliance/wasm-tools-go/cmd/wit-bindgen-go generate -o internal/ ./add.wit
```

The `internal` directory will contain the generated Go code for the `add.wit` file.

```sh
$ tree internal
internal
├── docs
│   └── adder
│       └── adder
│           ├── adder.exports.go
│           ├── adder.wasm.go
│           ├── adder.wit
│           ├── adder.wit.go
│           └── empty.s
└── wasi
    ├── cli
    │   └── stdout
    │       ├── empty.s
    │       ├── stdout.wasm.go
    │       └── stdout.wit.go
    ├── io
    │   ├── error
    │   │   ├── empty.s
    │   │   ├── error.wit.go
    │   │   └── ioerror.wasm.go
    │   └── streams
    │       ├── empty.s
    │       ├── streams.wasm.go
    │       └── streams.wit.go
    └── random
        └── random
            ├── empty.s
            ├── random.wasm.go
            └── random.wit.go
```

The `adder.exports.go` file contains the exported functions that need to be implemented in the Go code called `Exports`.

## 3. Implement the `add` function

```Go
package main

import (
	"example.com/internal/example/component/example"
)

func init() {
	example.Exports.Add = func(x int32, y int32) int32 {
		return x + y
	}
}

// main is required for the `wasi` target, even if it isn't used.
func main() {}
```

Go's `init` functions are used to do initialization tasks that
should be done before any other tasks. In this case, we are using it to export the `Add` function and
make it callable using the generated C bindings (`adder.c`). After populating the `init` function,

## 4. Build the component

We can build our component using TinyGo by specifying the wit-package to be `add.wit` and the WIT world to be `adder`. 

TinyGo will invoke `wasm-tools` to embed the WIT file to the module and componentize it.

```sh
$ tinygo build -target=wasip2 -o add.wasm --wit-package add.wit --wit-world adder main.go
```

We can confirm using the `wasm-tools component wit` command:

```sh
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
`example` world. We've provided an [`example-host`](../../examples/example-host/README.md) to do
just that. It calls the `add` function of a passed in component providing two operands. To use it,
clone this repository and run the Rust program:

```sh
git clone git@github.com:bytecodealliance/component-docs.git
cd component-docs/component-model/examples/example-host
cargo run --release -- 1 2 /path/to/add.wasm
```
