# C/C++ Tooling

WebAssembly components can be built from C and C++ using [`clang`][clang],
the C language family frontend for [LLVM][llvm].

[`wit-bindgen`](https://github.com/bytecodealliance/wit-bindgen) is a tool
to generate guest language bindings from a given `.wit` file.
When compiling C or C++ code to WebAssembly components,
we say that C or C++ is the "guest" language,
and WebAssembly is the "host" language.
In this case, "bindings" are C or C++ declarations: type signatures that
correspond to WIT functions, and type definitions that correspond to WIT types.
The bindings generator only generates declarations; you have to write
the code that actually implements these declarations,
if you're developing your own `.wit` files.
For WIT interfaces that are built in to WASI, the code is part of the
WebAssembly runtime that you will be using.

C/C++ currently lacks an integrated toolchain like Rust's [`cargo-component`][cargo-component]).
However, `wit-bindgen` can generate source-level bindings for
Rust, C, Java ([TeaVM](https://teavm.org/)), and [TinyGo](https://tinygo.org/),
with the ability to add more language generators in the future.

`wit-bindgen` can be used to build C applications that can be compiled directly to WebAssembly modules using [`clang`][clang] with a [`wasm32-wasi`][clang-tgt-wasm32-wasi] target.

[clang]: https://clang.llvm.org/
[clang-tgt-wasm32-wasi]: https://clang.llvm.org/docs/ClangCommandLineReference.html#webassembly
[llvm]: https://llvm.org/
[wasi]: https://wasi.dev/
[cargo-component]: https://crates.io/crates/cargo-component
[rust]: https://www.rust-lang.org/learn/get-started
[sample-wit]: https://github.com/bytecodealliance/component-docs/blob/main/component-model/examples/tutorial/wit/adder/world.wit

## 1. Download dependencies

First, install the following dependencies:
1. [`wit-bindgen` CLI](https://github.com/bytecodealliance/wit-bindgen#cli-installation)
2. [`wasm-tools`](https://github.com/bytecodealliance/wasm-tools)
    * `wasm-tools` can be used to inspect compiled WebAssembly modules and components,
    as well as converting between preview1 modules and preview2 components in
    the optional manual workflow.
3. The [`WASI SDK`](https://github.com/webassembly/wasi-sdk)
    * WASI SDK is a WASI enabled C/C++ toolchain which includes a version of the C standard 
    library (`libc`) implemented with WASI interfaces,
      among other artifacts necessary to compile C/C++ to WebAssembly.
    * On a Linux system, you can skip to the ["Install"](https://github.com/webassembly/wasi-sdk?tab=readme-ov-file#install) section.
      To build from source, start from the beginning of the README.

A WASI SDK installation will include a local version of `clang` configured with a WASI sysroot.
(A sysroot is a directory containing header files and libraries
for a particular target platform.)
Follow [these instructions](https://github.com/WebAssembly/wasi-sdk#use) to configure WASI SDK for use.

> [!NOTE]
> You can also use your installed system or [Emscripten](https://emscripten.org/) `clang`
> by building with `--target=wasm32-wasi`, but you will need some artifacts from WASI SDK
> to enable and link that build target (see the text about `libclang_rt.*.a` objects in
> [the WASI SDK README](https://github.com/webassembly/wasi-sdk?tab=readme-ov-file#about-this-repository)).

## 2. Generate program skeleton from WIT

Start by pasting the contents of the [sample `adder/world.wit` file][sample-wit]
into a local file.
Then generate a C skeleton from `wit-bindgen` using this file:

```
> wit-bindgen c path/to/adder/world.wit
Generating "adder.c"
Generating "adder.h"
Generating "adder_component_type.o"
```

This command generates several files:

1. `adder.h` (based on the `adder` world). This header file contains, amidst some boilerplate,
the prototype of the `add` function, which should look like this.
(The name of the function has been prefixed with "`exports`".)

```c
  uint32_t exports_docs_adder_add_add(uint32_t x, uint32_t y);
  ```

2. `adder.c`, which interfaces with the component model ABI to call your function.
   This file contains an `extern` declaration that looks like:

   ```c
    extern void __component_type_object_force_link_adder(void);
    ```

3. `adder_component_type.o`, which contains object code, including
   the definition of the `__component_type_object_force_link_adder` function,
   which must be linked via `clang`.

## 3. Write program code

Next, create an `component.c` that implements the `adder` world:
that is, which provides definitions for the interface function declared in `adder.h`.

```c
{{#include ../../examples/tutorial/c/adder/component.c}}
```

## 4. Compile a WebAssembly Preview 2 component with `wasi-sdk`'s `wasm32-wasip2-clang`

"P1" refers to [WASI Preview 1](https://github.com/WebAssembly/WASI/blob/main/legacy/README.md),
the initial version of the WASI APIs.
"P2" refers to [WASI Preview 2](https://github.com/WebAssembly/WASI/blob/main/wasip2/README.md),
which introduced the component model.

We can build a P2 component quickly by using the `wasm32-wasip2-clang` binary
that was installed by the WASI SDK.
If necessary, change `/opt/wasi-sdk` to the path where you installed
the WASI SDK.

```console
/opt/wasi-sdk/bin/wasm32-wasip2-clang \
    -o adder.wasm \
    -mexec-model=reactor \
    component.c \
    adder.c \
    adder_component_type.o
```

Breaking down each part of this command:

* `adder.wasm` is the output file that will contain binary WebAssembly code.
* The `-mexec-model` flag controls the desired execution model of the
  generated code. The argument can be either `reactor` or `command`.
  In this case, we pass in `-mexec-model=reactor` to build a _reactor_ component.
  A reactor component is more like a library, while a command component
  is more like an executable.
* `component.c` contains the code you wrote to implement the `adder` world.
* `adder.c` and `adder_component_type.o` were generated by `wit-bindgen`.

After this command completes, you will have a new file named `adder.wasm`
available in the source folder.
You can see that `adder.wasm` contains a component with the following command:

```console
> wasm-tools print adder.wasm | head -1
(component
```

For use cases that require building a P1 module and/or
adapting an existing P1 module into a P2 module,
such as building for a platform that does not support P2,
details on a more manual approach using `wasi-sdk`'s `clang` and `wasm-tools`
can be found below:

<details>
<summary>Manual P1 and P2 build</summary>

### Compile the component code into a WebAssembly P1 module via `clang`:

Assuming you defined `WASI_SDK_PATH` according to
the ["Use" section](https://github.com/webassembly/wasi-sdk?tab=readme-ov-file#use)
in the WASI SDK README, execute:

```console
$WASI_SDK_PATH/bin/clang component.c adder.c adder_component_type.o \
    -o adder.wasm -mexec-model=reactor
```

You can see that this command created a module with the following command:

```console
> wasm-tools print adder.wasm | head -1
(module $adder.wasm
```

>
> Alternatively, you can also use the published [`ghcr.io/webassembly/wasi-sdk` container images][wasi-sdk-images]
> for performing builds.
>
> For example, to enter a container with `wasi-sdk` installed:
>
> ```console
> docker run --rm -it --mount type=bind,src=path/to/app/src,dst=/app \
>     ghcr.io/webassembly/wasi-sdk:wasi-sdk-25
> ```
>
> where `path/to/app/src` is replaced with the absolute path of the directory
> containing the code for your sample app.
>
> Then inside the container, after changing to the directory containing
> the code for your sample app, you can run:
>
> ```console
> /opt/wasi-sdk/bin/clang component.c adder.c adder_component_type.o \
> -o adder.wasm -mexec-model=reactor
> ```
>
> Using the Dockerfile avoids the need to install the WASI SDK on your system.
>
> See also: [`Dockerfile` in `wasi-sdk`][wasi-sdk-dockerfile]

[wasi-sdk-images]: https://github.com/WebAssembly/wasi-sdk/pkgs/container/wasi-sdk
[wasi-sdk-dockerfile]: https://github.com/WebAssembly/wasi-sdk/blob/main/docker/Dockerfile

### Transform the P1 component to a P2 component with `wasm-tools`

Next, we need to transform the P1 component to a P2 component.
To do this, we can use `wasm-tools component new`:

```console
wasm-tools component new adder.wasm -o adder.component.wasm
```

> [!NOTE]
> The `.component.` extension has no special meaning—`.wasm` files can be either modules or components.

### (optional) Build a WASI-enabled WebAssembly (P2) component with `wasm-tools`

Note that `wasm-tools component new` may fail if your code references any
[WASI][wasi] APIs that must be imported, for example via standard library imports like `stdio.h`.

Using WASI interfaces requires an additional step,
as the WASI SDK still references WASI Preview 1 APIs (those with `wasi_snapshot_preview1` in their names)
that are not compatible directly with components.

For example, if we modify the above code to reference `printf()`,
it would compile to a P1 component:

```c
{{#include ../../examples/tutorial/c/adder/component_with_printf.c}}
```

However, the module would fail to transform to a P2 component:

```
> wasm-tools component new adder.wasm -o adder.component.wasm
error: failed to encode a component from module

Caused by:
    0: failed to decode world from module
    1: module was not valid
    2: failed to resolve import `wasi_snapshot_preview1::fd_close`
    3: module requires an import interface named `wasi_snapshot_preview1`
```

To build a P2 component that uses [WASI][wasi] interfaces from a P1 component,
we'll need to make use of adapter modules.
An adapter module provides definitions for WASI Preview 1 API functions
in terms of WASI Preview 2 API functions.

Download the appropriate reactor adapter module [as documented here](https://github.com/bytecodealliance/wit-bindgen#creating-components-wasi)
and save it to the same directory that contains the `.c` and `.wasm` files you have been working with.

You can either get [the linked release][wasmtime-releases] of `wasi_snapshot_preview1.reactor.wasm`
and rename it to `wasi_snapshot_preview1.wasm`,
or build it directly from source in `wasmtime` following
the [instructions here](https://github.com/bytecodealliance/wasmtime/tree/main/crates/wasi-preview1-component-adapter)
(make sure you `git submodule update --init` first).

Now, you can adapt preview1 to preview2 to build a component:

```console
wasm-tools component new adder.wasm --adapt wasi_snapshot_preview1.wasm \
    -o adder.component.wasm
```

[wasmtime-releases]: https://github.com/bytecodealliance/wasmtime/releases

</details>

## 5. Inspect the built component

Finally, you can inspect the embedded WIT to see your component
(including any WASI imports if you used them):

```
> wasm-tools component wit adder.component.wasm
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

### 6. Run the component from the example host

The following section requires you to have [a Rust toolchain][rust] installed.

> [!WARNING]
> You must be careful to use a version of the adapter (`wasi_snapshot_preview1.wasm`) that is compatible with the version of
> `wasmtime` that will be used, to ensure that WASI interface versions (and relevant implementation) match.

This repository contains an [example WebAssembly host][example-host] written in Rust
that can run components that implement the `adder` world.

1. Check out the repository:

   `git clone https://github.com/bytecodealliance/component-docs.git`
2. `cd component-docs/component-model/examples/example-host`
3. `cargo run --release -- 1 2 <PATH>/adder.wasm`
* The double dashes separate the flags passed to `cargo` from
  the flags passed in to your code.
* The arguments 1 and 2 are the arguments to the adder.

In place of `<PATH>`, substitute the directory that contains your
generated `adder.wasm` file (or `adder.component.wasm` if you used
the manual instructions).

> [!NOTE]
> When hosts run components that use WASI interfaces, they must *explicitly*
> [add WASI to the linker][add-to-linker] to run the built component.

A successful run should show the following output
(of course, the paths to your example host and adder component will vary,
and you should substitute `adder.wasm` with `adder.component.wasm`
if you followed the manual instructions above):

```
cargo run --release -- 1 2 adder.wasm
   Compiling example-host v0.1.0 (/path/to/component-docs/component-model/examples/example-host)
    Finished `release` profile [optimized] target(s) in 7.85s
     Running `target/debug/example-host 1 2 /path/to/adder.wasm`
1 + 2 = 3
```

If *not* configured correctly, you may see errors like the following:

```
cargo run --release -- 1 2 adder.wasm
   Compiling example-host v0.1.0 (/path/to/component-docs/component-model/examples/example-host)
    Finished `release` profile [optimized] target(s) in 7.85s
     Running `target/release/example-host 1 2 /path/to/adder.component.wasm`
Error: Failed to instantiate the example world

Caused by:
    0: component imports instance `wasi:io/error@0.2.2`, but a matching implementation was not found in the linker
    1: instance export `error` has the wrong type
    2: resource implementation is missing
```

This kind of error normally indicates that the host in question does not satisfy WASI imports.

[host]: https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/example-host
[add-to-linker]: https://docs.wasmtime.dev/api/wasmtime_wasi/fn.add_to_linker_sync.html
[example-host]: https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/example-host

## 7. Run the component from C/C++ Applications

It is not yet possible to run a WebAssembly Component using the C API of `wasmtime` `c-api`.
See [`wasmtime` issue #6987](https://github.com/bytecodealliance/wasmtime/issues/6987) for more details.
The c-api is preferred over directly using the example host Rust crate in C++.

However, C/C++ language guest components can be composed with components written in any other language
and run by their toolchains,
or even composed with a C language command component and run via the `wasmtime` CLI
or any other host.

See the [Rust Tooling guide](../language-support/rust.md#running-a-component-from-rust-applications)
for instructions on how to run this component from the Rust `example-host`
(replacing the path to `add.wasm` with your `adder.wasm` or `adder.component.wasm` above).

[!NOTE]: #
[!WARNING]: #
