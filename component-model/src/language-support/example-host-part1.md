This repository contains an [example WebAssembly host][example-host] written in Rust
that can run components that implement the `adder` world.

1. `git clone https://github.com/bytecodealliance/component-docs.git`
2. `cd component-docs/component-model/examples/example-host`
3. `cargo run --release -- 1 2 <PATH>/adder.wasm`
* The double dashes separate the flags passed to `cargo` from
  the flags passed in to your code.
* The arguments 1 and 2 are the arguments to the adder.
* In place of `<PATH>`, substitute the directory that contains your
  generated `adder.wasm` file.

> Note:
> When hosts run components that use WASI interfaces, they must *explicitly*
> [add WASI to the linker][add-to-linker] to run the built component.
