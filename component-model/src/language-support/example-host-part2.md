
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
[add-to-linker]: https://docs.wasmtime.dev/api/wasmtime_wasi/p2/fn.add_to_linker_sync.html
[example-host]: https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/example-host
