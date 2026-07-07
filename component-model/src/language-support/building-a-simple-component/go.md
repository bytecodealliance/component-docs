# Go Tooling

This guide walks through building a Go component that implements
the `adder` world defined in the [`adder/world.wit` package][docs-adder].
The component will implement the `adder` world,
which contains an `add` interface with an `add` function.

Keep in mind that this is a basic intro. For more examples, please see the [componentize-go][componentize-go] repository examples.

If you still have questions, feel free to reach out to to the componentize-go maintainers on [Zulip](https://bytecodealliance.zulipchat.com/).

## 1. Install the tools

- [Go](https://go.dev/) 1.25.5+
- [componentize-go][componentize-go] Latest version

## 2. Create your Go project

With the [`component-docs` repository][repo-component-docs] cloned locally, run the following:

```sh
mkdir go-adder && cd go-adder
```

## 2. Determine which world the component will implement

Since we will be implementing the [`adder` world][docs-adder], we can copy the WIT to our project.
Create a subdirectory called `wit` and paste the following code
into a file called `wit/adder.wit`:

```wit
{{#include ../../../examples/tutorial/wit/adder/world.wit}}
```

## 3. Generate bindings for the Wasm component

Run the following commands to generate bindings for the component:

```console
componentize-go --world adder bindings
go mod tidy
```

The project directory will look like this:

```
$ tree
.
├── go.mod
├── go.sum
├── wit
│   └── adder.wit
└── wit_exports.go
```

Here's a breakdown of what each of these files do:
- `go.mod` contains a required library of shared WIT types (see [go.bytecodealliance.org/pkg](https://go.bytecodealliance.org/pkg))
- `wit_exports.go` defines the `//go:wasmexport` methods for the Go WebAssembly compiler.

If you try to compile this, you'll get an error from the `wit_exports.go` file:
```
could not import wit_component/export_docs_adder_add (no required module provides package "wit_component/export_docs_adder_add")
```

We'll create this module in the next section.

## 4. Implement the `add` Function

In your `add` directory, create a file called `export_docs_adder_add/exports.go`
and paste the following code into it:

```Go
{{#include ../../../examples/tutorial/go/adder/exports.go}}
```

## 5. Build the Component

To compile your Go application to WebAssembly, run the following command:

```sh
componentize-go build
```

This will output a `main.wasm` file that will be run in the next section.

## 5. Testing the `add` Component

To verify that our component works, let's run it from a Rust application that knows how to run a
component targeting the [`adder` world][docs-adder]. Be sure to have the [Rust toolchain](https://rust-lang.org/tools/install/)
installed.

The application uses [`wasmtime`][crates-wasmtime] to generate Rust "host"/"embedder" bindings,
bring in WASI worlds, and execute the component.

From within the `go-adder` directory, run the following:

```console
$ cd ../component-model/examples/example-host
$ cargo run --release -- 1 2 ../../../go-adder/main.wasm
1 + 2 = 3
```

With this, we have successfully built and run a basic WebAssembly component with Go 🎉

[crates-wasmtime]: https://crates.io/crates/wasmtime
[repo-component-docs]: https://github.com/bytecodealliance/component-docs
[docs-adder]: https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/tutorial/wit/adder/world.wit
[componentize-go]: https://github.com/bytecodealliance/componentize-go

