# Browser & Node Demo

## Calculator with `jco`

Using [`jco`](https://github.com/bytecodealliance/jco/blob/main/docs/src/example.md)
See the [`jco` example](https://github.com/bytecodealliance/jco/blob/main/docs/src/example.md) that was used to create this minimal demo.

```sh
(cd calculator && cargo component build --release)
(cd adder && cargo component build --release)
(cd command && cargo component build --release)
wasm-tools compose calculator/target/wasm32-wasi/release/calculator.wasm -d adder/target/wasm32-wasi/release/adder.wasm -o composed.wasm
wasm-tools compose command/target/wasm32-wasi/release/command.wasm -d composed.wasm -o command.wasm

# We need to transpile to extract/generate bindings for JS
# We do want to *ommit* anything related to syscalls, that wasi wants
# Thus use the composed, not the command output.
jco transpile composed.wasm -o www
# Serve required files (index.html & jco genereated files minimally)
npx live-server www/

# Run CLI example
node www/cli-calc.js
```
