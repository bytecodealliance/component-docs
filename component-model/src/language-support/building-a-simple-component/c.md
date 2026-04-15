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

C/C++ currently lacks an integrated toolchain.
However, `wit-bindgen` can generate source-level bindings for
Rust, C, Java ([TeaVM](https://teavm.org/)), and [TinyGo](https://tinygo.org/),
with the ability to add more language generators in the future.

`wit-bindgen` can be used to build C applications that can be compiled directly to WebAssembly modules using [`clang`][clang] with a [`wasm32-wasi`][clang-tgt-wasm32-wasi] target.

[clang]: https://clang.llvm.org/
[clang-tgt-wasm32-wasi]: https://clang.llvm.org/docs/ClangCommandLineReference.html#webassembly
[llvm]: https://llvm.org/
[wasi]: https://wasi.dev/
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

## 2. Generate program skeleton from WIT

Start by pasting the contents of the [sample `adder/world.wit` file][sample-wit]
into a local file.
Then generate a C skeleton from `wit-bindgen` using this file:

```
$ wit-bindgen c path/to/adder/world.wit
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

Next, create a file named `component.c` with code that implements the `adder` world:
that is, code which fulfills the definition of the interface function declared in `adder.h`.

```c
{{#include ../../../examples/tutorial/c/adder/component.c}}
```

## 4. Compile a WebAssembly Preview 2 component with `wasi-sdk`'s `wasm32-wasip2-clang`

"P1" refers to [WASI Preview 1](https://github.com/WebAssembly/WASI/blob/main/legacy/README.md),
the initial version of the WASI APIs.
"P2" refers to [WASI Preview 2](https://github.com/WebAssembly/WASI/blob/main/wasip2/README.md),
which introduced the component model.

While in the past building a P2 component required conversion from a P1 component,
we can now build a P2 component directly by using the `wasm32-wasip2-clang` binary
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

* `-o adder.wasm` configures the output file that will contain binary WebAssembly code.
* `-mexec-model=reactor` controls the desired execution model of the
  generated code. The argument can be either `reactor` or `command`.
  In this case, we pass in `-mexec-model=reactor` to build a _reactor_ component.
  A reactor component is more like a library, while a command component
  is more like an executable.
* `component.c` contains the code you wrote to implement the `adder` world.
* `adder.c` and `adder_component_type.o` were generated by `wit-bindgen` and contain
  necessary scaffolding (e.g. function exports) to enable building `component.c` into a WebAssembly
  binary.

After this command completes, you will have a new file named `adder.wasm`
available in the source folder.

You can verify that `adder.wasm` is a valid WebAssembly component with the following command:

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
$WASI_SDK_PATH/bin/clang \
    -o adder.wasm \
    -mexec-model=reactor \
    component.c \
    adder.c \
    adder_component_type.o
```

You can verify that `adder.wasm` is a valid WebAssembly P1 component (i.e. a WebAssembly core module) with the following command:

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
> docker run --rm -it \
>     --mount type=bind,src=path/to/app/src,dst=/app \
>     ghcr.io/webassembly/wasi-sdk:wasi-sdk-27
> ```
>
> Replace `path/to/app/src` with the absolute path of the directory
> containing the code for your sample app.
>
> Inside the container your source code will be available at `/app`. After changing
> to that directory, you can run:
>
> ```console
> /opt/wasi-sdk/bin/clang \
>     -o adder.wasm \
>     -mexec-model=reactor \
>     component.c \
>     adder.c \
>     adder_component_type.o
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
[WASI][wasi] APIs that must be imported:
for example, via standard library imports like `stdio.h`.

Using WASI interfaces requires an additional step,
as the WASI SDK still references WASI Preview 1 APIs (those with `wasi_snapshot_preview1` in their names)
that are not compatible directly with components.

For example, if we modify the above code to reference `printf()`,
it would compile to a P1 component:

```c
{{#include ../../../examples/tutorial/c/adder/component_with_printf.c}}
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
wasm-tools component new \
    adder.wasm \
    --adapt wasi_snapshot_preview1.wasm \
    -o adder.component.wasm
```

[wasmtime-releases]: https://github.com/bytecodealliance/wasmtime/releases

</details>

## 5. Inspect the built component

Finally, you can inspect a WIT representation of the imports and exports of your component
(including any WASI imports if you used them):

```
$ wasm-tools component wit adder.component.wasm
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

## 6. Run the component with `wasmtime --invoke`

If you want to quickly run the `add` export without writing a host application that embeds Wasmtime,
you can invoke it directly with the Wasmtime CLI.

```console
wasmtime run --invoke 'add(2, 2)' adder.wasm
```

Depending on your Wasmtime version, the shorthand form may also work:

```console
wasmtime --invoke 'add(2,2)' adder.wasm
```

## 7. Run the component from the example C host

This repository includes a C application that can execute components that implement the add interface. This application embeds Wasmtime using the Wasmtime C API:
`component-model/examples/example-c-host/host.c`.

The application expects three arguments: the two numbers to add and the Wasm component that executed the addition. For example:

```sh
./adder-host <x> <y> <path-to-component.wasm>
```

You can either use a Dockerfile to execute your add component with the C application or directly run the application.

### Option A: Compile and run the host directly

If the Wasmtime C API headers and library are installed on your system,
you can compile and run the host directly:

On Linux, the following commands install the C API artifacts in `/usr/local`
using the same approach as `Dockerfile.host`:

```console
sudo apt-get update
sudo apt-get install -y --no-install-recommends \
  gcc libc6-dev curl xz-utils ca-certificates

WASMTIME_VERSION=42.0.1
case "$(uname -m)" in
  x86_64) WASMTIME_ARCH=x86_64 ;;
  aarch64|arm64) WASMTIME_ARCH=aarch64 ;;
  *) echo "unsupported architecture: $(uname -m)" >&2; exit 1 ;;
esac

curl -sL "https://github.com/bytecodealliance/wasmtime/releases/download/v${WASMTIME_VERSION}/wasmtime-v${WASMTIME_VERSION}-${WASMTIME_ARCH}-linux-c-api.tar.xz" \
  | sudo tar xJ --strip-components=1 -C /usr/local

sudo ldconfig
```

```console
cd component-model/examples/example-c-host
gcc -o adder-host host.c -lwasmtime
./adder-host 1 2 /absolute/path/to/adder.wasm
```

If `libwasmtime.so` is not in a default library path on Linux,
set `LD_LIBRARY_PATH` before running:

```console
LD_LIBRARY_PATH=/path/to/wasmtime/lib ./adder-host 1 2 /absolute/path/to/adder.wasm
```

Expected output:

```sh
1 + 2 = 3
```

### Option B: Run with Docker (`Dockerfile.host`)

Instead of installing the Wasmtime C API, you can use the provided Dockerfile which builds the C application. 

From `component-model/examples/example-c-host`:

```console
cp ../../examples/example-host/add.wasm ./adder.wasm

docker build \
  -f Dockerfile.host \
  -t example-c-host:latest \
  .
```

Then run the container, passing in the component as a volume.

```console
docker run --rm \
  -v /absolute/path/to/component-docs/component-model/examples/example-c-host/adder.wasm:/component/add.wasm:ro \
  example-c-host:latest 1 2 /component/add.wasm
```

Expected output:

```sh
1 + 2 = 3
```

`Dockerfile.guest_and_host` is also provided in the same directory if you want
an all-in-one image that builds both the guest component and the C host.

Its guest build path follows the same sequence described in steps 2-4:
`wit-bindgen c ...`, then `/opt/wasi-sdk/bin/wasm32-wasip2-clang ...`.
The Dockerfile additionally automates fetching `world.wit` and `component.c`
from this repository and pins tool versions for reproducibility.
At the time of writing, `Dockerfile.guest_and_host` is Linux `x86_64`-specific.
