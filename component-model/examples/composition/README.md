## Summary:
This directory contains an example where component can be written and built separately in different languages and then composed together.
There are multiple components in this repo :
add, calculator and app are components written in RUST.
sub is a component written in tinygo.

calculator is a component that composes the components add and sub at compile time.
calculator is a library component and cannot be used as a standalone app.
Hence, the app component is built as a command component that composes the calculator component.

## Prerequisites:

### Install Wasmtime
curl https://wasmtime.dev/install.sh -sSf | bash

### Update PATH
Wasmtime binary gets deployed at ~/.wasmtime directory.

Update bash profile with wasmtime
```export PATH="~/.wasmtime/bin:$PATH"```


### Install cargo component 
```cargo install cargo-component```


## Build and test

### Build the components 

```
cd calculator 
cargo component build --release
cd ..
cd add 
cargo component build --release
cd ..

cd app 
cargo component build --release
cd ..
```

### Subtractor component is a go component. Build it according to the readme in the sub directory to generate output component- sub-component.wasm.
```
cd sub
```

### Compose the components add, sub and calculator
```
cd ..
wasm-tools compose calculator/target/wasm32-wasi/release/calculator.wasm -d add/target/wasm32-wasi/release/add.wasm -d sub/sub-component.wasm -o composed-add-sub.wasm
```

### Generate app component using composed-add-sub.wasm
```
wasm-tools compose app/target/wasm32-wasi/release/app.wasm -d composed-add-sub.wasm -o command.wasm
```


### Run the command component using wasmtime to test it 
``` 
wasmtime run command.wasm 1 2 add

wasmtime run command.wasm 1 2 sub
```
