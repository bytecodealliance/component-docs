# C/C++ Tooling

WebAssembly components can be built from C and C++ using [`clang`][clang], the C language family frontend for [LLVM][llvm].

[`wit-bindgen`](https://github.com/bytecodealliance/wit-bindgen) is a tool to generate guest language bindings from a
given `.wit` file.

Although `wit-bindgen` is a standalone tool (whereas some languages have more integrated toolchains like Rust's [`cargo-component`][cargo-component]),
`wit-bindgen` can generate source-level bindings for `Rust`, `C`, `Java (TeaVM)`, and `TinyGo`, with the ability for more
language generators to be added in the future.

`wit-bindgen` can be used to build C applications that can be compiled directly to Wasm modules using [`clang`][clang] with a [`wasm32-wasi`][clang-tgt-wasm32-wasi] target.

[clang]: https://clang.llvm.org/
[clang-tgt-wasm32-wasi]: https://clang.llvm.org/docs/ClangCommandLineReference.html#webassembly
[llvm]: https://llvm.org/
[wasi]: https://wasi.dev/
[cargo-component]: https://crates.io/crates/cargo-component

## 1. Download dependencies

First, install the CLI for [`wit-bindgen`](https://github.com/bytecodealliance/wit-bindgen#cli-installation), [`wasm-tools`](https://github.com/bytecodealliance/wasm-tools), and the [`WASI SDK`](https://github.com/webassembly/wasi-sdk).

The WASI SDK will install a local version of `clang` configured with a wasi-sysroot. Follow [these instructions](https://github.com/WebAssembly/wasi-sdk#use) to configure it for use. Note that you can also use your installed system or emscripten `clang` by building with `--target=wasm32-wasi` but you will need some artifacts from WASI SDK to enable and link that build target (more information is available in WASI SDK's docs).

## 2. Generate program skeleton from WIT

Start by generating a C skeleton from `wit-bindgen` using the [sample `adder/world.wit` file](https://github.com/bytecodealliance/component-docs/tree/main/examples/tutorial/wit/adder/world.wit):

```
> wit-bindgen c path/to/adder/world.wit
Generating "adder.c"
Generating "adder.h"
Generating "adder_component_type.o"
```

This has generated several files:

1.`adder.h` (based on the `adder` world) with the prototype of the `add` function (prefixed by `exports_`) - `uint32_t exports_docs_adder_add_add(uint32_t x, uint32_t y);`.
2. `adder.c` that interfaces with the component model ABI to call your function.
3. `adder_component_type.o` which contains object code referenced in `adder.c` from an `extern` that must be linked via `clang`.

## 3. Write program code

Next, create an `component.c` that implements the `adder` world (i.e. the interface defined in `adder.h`):

```c
#include "adder.h"

uint32_t exports_docs_adder_add_add(uint32_t x, uint32_t y)
{
	return x + y;
}
```

## 4. Compile a WebAssembly module (P1) with `clang`

Now, you can compile the function into a Wasm module via clang:

```console
clang component.c adder.c adder_component_type.o -o adder.wasm -mexec-model=reactor
```

> Use the `clang` included in the WASI SDK installation, for example at `<WASI_SDK_PATH>/bin/clang`.
>
> Alternatively, you can also use the published [`ghcr.io/webassembly/wasi-sdk` container images][wasi-sdk-images]
> for performing builds.
>
> For example, to enter a container with `wasi-sdk` installed:
>
> ```
> docker run --rm -it --mount type=bind,src=path/to/app/src,dst=/app ghcr.io/webassembly/wasi-sdk:wasi-sdk-25
> ```
>
> See also: [`Dockerfile` in `wasi-sdk`][wasi-sdk-dockerfile]

[wasi-sdk-images]: https://github.com/WebAssembly/wasi-sdk/pkgs/container/wasi-sdk
[wasi-sdk-dockerfile]: https://github.com/WebAssembly/wasi-sdk/blob/main/docker/Dockerfile

## 5. Convert the P1 component to a P2 component with `wasm-tools`

Next, we need to transform the P1 component to a P2 component. To do this, we can use `wasm-tools component new`:

```console
wasm-tools component new ./adder.wasm -o adder.component.wasm
```

> [!NOTE]
> The `.component.` extension has no special meaning -- `.wasm` files can be either modules or components.

## 6. (optional) Build a WASI-enabled WebAssembly (P2) component with `wasm-tools`

Do note `wasm-tools component new` may fail if your code references any [WASI][wasi] APIs that must be imported, for
example via standard library imports like `stdio.h`.

Using WASI interfaces requires an additional step as the WASI SDK still references `wasi_snapshot_preview1` APIs that are not compatible directly with components.

For example, modifying the above to reference `printf()` would compile:

```c
#include "adder.h"
#include <stdio.h>

uint32_t exports_docs_adder_add_add(uint32_t x, uint32_t y)
{
	uint32_t result = x + y;
	printf("%d", result);
	return result;
}
```

However, the module would fail to transform to a component:

```
>wasm-tools component new ./adder.wasm -o adder.component.wasm
error: failed to encode a component from module

Caused by:
    0: failed to decode world from module
    1: module was not valid
    2: module requires an import interface named `wasi_snapshot_preview1`
```

To build a P2 component that uses [WASI][wasi] interfaces from a P1 component, we'll need to make use of adapter modules.

Install the appropriate reactor adapter module [as documented here](https://github.com/bytecodealliance/wit-bindgen#creating-components-wasi).

You can either get [the linked release][wasmtime-releases] of `wasi_snapshot_preview1.reactor.wasm` and rename it to `wasi_snapshot_preview1.wasm`, or build it directly from source in `wasmtime` following the [instructions here](https://github.com/bytecodealliance/wasmtime/tree/main/crates/wasi-preview1-component-adapter) (make sure you `git submodule update --init` first).

Now, you can adapt preview1 to preview2 to build a component:

```console
wasm-tools component new adder.wasm --adapt wasi_snapshot_preview1.wasm -o adder.component.wasm
```

[wasmtime-releases]: https://github.com/bytecodealliance/wasmtime/releases

## 7. Inspect the built component

Finally, you can inspect the embedded wit to see your component (including any WASI imports if necessary):

```
>wasm-tools component wit adder.component.wasm
package root:component;

world root {
  import wasi:io/error@0.2.2;
  import wasi:io/streams@0.2.2;
  import wasi:cli/stdin@0.2.2;
  import wasi:cli/stdout@0.2.2;
  import wasi:cli/stderr@0.2.2;
  import wasi:cli/terminal-input@0.2.2;
  import wasi:cli/terminal-output@0.2.2;
  import wasi:cli/terminal-stdin@0.2.2;
  import wasi:cli/terminal-stdout@0.2.2;
  import wasi:cli/terminal-stderr@0.2.2;
  import wasi:clocks/wall-clock@0.2.2;
  import wasi:filesystem/types@0.2.2;
  import wasi:filesystem/preopens@0.2.2;

  export add: func(x: s32, y: s32) -> s32;
}
...
```

### 8. Running the component from the example host

> [!WARNING]
> You must be careful to use a version of the adapter (`wasi_snapshot_preview1.wasm`) that is compatible with the version of
> `wasmtime` that will be used, to ensure that WASI interface versions (and relevant implementation) match.

This repository contains an [example WebAssembly host][example-host] written in Rust that can run components that implement the `adder` world.

> [!NOTE]
> When hosts run components that use WASI interfaces, they must *explicitly* [add WASI to the linker][add-to-linker] to run the built component.

A successful run should show the following output:

```
cargo run --release -- 1 2 adder.component.wasm
   Compiling example-host v0.1.0 (/path/to/component-docs/component-model/examples/example-host)
    Finished `release` profile [optimized] target(s) in 7.85s
     Running `target/debug/example-host 1 2 /tmp/docs/c/adder.component.wasm`
1 + 2 = 3
```

If *not* configured correctly, you may see errors like the following:

```
cargo run --release -- 1 2 adder.component.wasm
   Compiling example-host v0.1.0 (/path/to/component-docs/component-model/examples/example-host)
    Finished `release` profile [optimized] target(s) in 7.85s
     Running `target/release/example-host 1 2 adder.component.wasm`
Error: Failed to instantiate the example world

Caused by:
    0: component imports instance `wasi:io/error@0.2.2`, but a matching implementation was not found in the linker
    1: instance export `error` has the wrong type
    2: resource implementation is missing
```

This kind of error normally indicates that the host in question does not contain satisfy WASI imports.

[host]: https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/example-host
[add-to-linker]: https://docs.wasmtime.dev/api/wasmtime_wasi/fn.add_to_linker_sync.html
[example-host]: https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/example-host

## 9. Running a Component from C/C++ Applications

It is not yet possible to run a WebAssembly Component using the C API of `wasmtime` `c-api`. See [`wasmtime` issue #6987](https://github.com/bytecodealliance/wasmtime/issues/6987) for more details.
The c-api is preferred over directly using the example host Rust crate in C++.

However, C/C++ language guest components can be composed with components written in any other language and
run by their toolchains, or even composed with a C language command component and run via the `wasmtime` CLI
or any other host.

See the [Rust Tooling guide](../language-support/rust.md#running-a-component-from-rust-applications) for instructions on how to run this component from
the Rust `example-host` (replacing the path to `add.wasm` with your `add-component` above).
