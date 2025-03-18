## C/C++ Tooling

### Building a Component with `wit-bindgen` and `wasm-tools`

[`wit-bindgen`](https://github.com/bytecodealliance/wit-bindgen) is a tool to generate guest language bindings from a given `.wit` file. Although it is less integrated into language toolchains than other tools such as `cargo-component`, it can currently generate source-level bindings for `Rust`, `C`, `Java (TeaVM)`, and `TinyGo`, with the ability for more language generators to be added in the future.

`wit-bindgen` can be used to generate C applications that can be compiled directly to Wasm modules using `clang` with a `wasm32-wasi` target.

First, install the CLI for [`wit-bindgen`](https://github.com/bytecodealliance/wit-bindgen#cli-installation), [`wasm-tools`](https://github.com/bytecodealliance/wasm-tools), and the [`WASI SDK`](https://github.com/webassembly/wasi-sdk).

The WASI SDK will install a local version of `clang` configured with a wasi-sysroot. Follow [these instructions](https://github.com/WebAssembly/wasi-sdk#use) to configure it for use. Note that you can also use your installed system or emscripten `clang` by building with `--target=wasm32-wasi` but you will need some artifacts from WASI SDK to enable and link that build target (more information is available in WASI SDK's docs).

Start by generating a C skeleton from `wit-bindgen` using the [sample `adder/world.wit` file](https://github.com/bytecodealliance/component-docs/tree/main/examples/tutorial/wit/adder/world.wit):

```
> wit-bindgen c path/to/adder/world.wit
Generating "example.c"
Generating "example.h"
Generating "example_component_type.o"
```

This has generated several files - an `example.h` (based on the name of your `world`) with the prototype of the `add` function (prefixed by `exports_`) - `int32_t exports_example_add(int32_t x, int32_t y);`, as well as some generated code in `example.c` that interfaces with the component model ABI to call your function. Additionally, `example_component_type.o` contains object code referenced in `example.c` from an `extern` that must be linked via clang.

Next, create an `add.c` that implements your function defined in `example.h`:

```c
#include "example.h"

int32_t exports_example_add(int32_t x, int32_t y)
{
	return x + y;
}
```

Now, you can compile the function into a Wasm module via clang:

```console
clang add.c example.c example_component_type.o -o add-core.wasm -mexec-model=reactor
```

> Use the `clang` included in the WASI SDK installation, for example at `<WASI_SDK_PATH>/bin/clang`.

Next, you need to transform the module into a component.  For this example, you can use `wasm-tools component new`:

```console
wasm-tools component new ./add-core.wasm -o add-component.wasm
```

Do note this will fail if your code references any WASI APIs that must be imported. This requires an additional step as the WASI SDK still references `wasi_snapshot_preview1` APIs that are not compatible directly with components.

For example, modifying the above to reference `printf()` would compile:

```c
#include "example.h"
#include <stdio.h>

int32_t exports_example_add(int32_t x, int32_t y)
{
	int32_t result = x + y;
	printf("%d", result);
	return result;
}
```

However, the module would fail to transform to a component:

```
>wasm-tools component new ./add-core.wasm -o add-component.wasm
error: failed to encode a component from module

Caused by:
    0: failed to decode world from module
    1: module was not valid
    2: module requires an import interface named `wasi_snapshot_preview1`
```

Install the appropriate reactor adapter module [as documented here](https://github.com/bytecodealliance/wit-bindgen#creating-components-wasi) - you can either get the linked release of `wasi_snapshot_preview1.reactor.wasm` and rename it to `wasi_snapshot_preview1.wasm`, or build it directly from source in `wasmtime` following the [instructions here](https://github.com/bytecodealliance/wasmtime/tree/main/crates/wasi-preview1-component-adapter) (make sure you `git submodule update --init` first).

Now, you can adapt preview1 to preview2 to build a component:

```console
wasm-tools component new add-core.wasm --adapt wasi_snapshot_preview1.wasm -o add-component.wasm
```

Finally, you can inspect the embedded wit to see your component (including any WASI imports if necessary):

```
>wasm-tools component wit add-component.wasm
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

You must use the `wasi_snapshot_preview1.wasm` from the same version of wasmtime that the host is using to ensure the WASI interface versions match. Additionally, the host must explicitly [add WASI to the linker](https://docs.wasmtime.dev/api/wasmtime_wasi/fn.add_to_linker_sync.html) to run the app. If these are not configured correctly, you may see errors like the following:

```
cargo run --release -- 1 2 add-component.wasm
   Compiling example-host v0.1.0 (/Users/sean/code/component-docs/component-model/examples/example-host)
    Finished `release` profile [optimized] target(s) in 7.85s
     Running `target/release/example-host 1 2 add-component.wasm`
Error: Failed to instantiate the example world

Caused by:
    0: component imports instance `wasi:io/error@0.2.2`, but a matching implementation was not found in the linker
    1: instance export `error` has the wrong type
    2: resource implementation is missing
```

### Running a Component from C/C++ Applications

It is not yet possible to run a Component using the `wasmtime` `c-api` - [see this issue](https://github.com/bytecodealliance/wasmtime/issues/6987). The c-api is preferred to trying to directly use the Rust crate in C++.

However, C/C++ language guest components can be composed with components written in any other language and run by their toolchains, or even composed with a C language command component and run via the `wasmtime` CLI or any other host.

See the [Rust Tooling guide](../language-support/rust.md#running-a-component-from-rust-applications) for instructions on how to run this component from the Rust `example-host` (replacing the path to `add.wasm` with your `add-component` above).
