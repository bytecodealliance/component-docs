# Creating Runnable Components (Javascript)

## Exporting `wasi:cli/run` to create

Components created with `jco` can export the `wasi:cli/run` interface, similarly to WebAssembly components written in other languages.

Exporting the `wasi:cli/run` interfaces enables ecosystem tooling to interoperate with (and run) the component you've built
 (e.g. `wasmtime run`). Components that conform to `wasi:cli/run` can be very concise.

For example:

```js
export const run =  {
    run() {
       console.log("Hello World!");
    }
}
```

The above component can be made recognizable as "runnable" to `wasi:cli`-aware tooling with the following WIT:

```wit
package runnable:js-component;

world component {
    export wasi:cli/run@0.2.4;
}
```
