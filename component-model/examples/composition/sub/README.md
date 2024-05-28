tinygo build -o sub.wasm -target=wasi sub.go

# convert to component using wasm-tools using the below 3 commands:
```
export COMPONENT_ADAPTER_REACTOR=../wasi_snapshot_preview1.reactor.wasm
wasm-tools component embed --world subtractor wit/world.wit sub.wasm -o sub.embed.wasm
wasm-tools component new -o sub-component.wasm --adapt wasi_snapshot_preview1="$COMPONENT_ADAPTER_REACTOR" sub.embed.wasm

```

#  Validate the component by checking its imports:
```wasm-tools component wit sub.component.wasm```