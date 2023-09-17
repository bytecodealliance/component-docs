# JavaScript Tooling

[`jco`](https://github.com/bytecodealliance/jco) is a fully native JS tool for working with the
emerging WebAssembly Components specification in JavaScript.

### Building a Component with `jco`

A component can be created from a JS module using `jco componentize`. First, install `jco` and
`componentize-js`:

```sh
$ npm install @bytecodealliance/jco
$ npm install @bytecodealliance/componentize-js
```

Create a JavaScript module that implements the `add` function in [`add.wit`](https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/example-host/add.wit):

```js
export function add(x, y) {
  return x + y;
}
```

Now, use `jco` to create a component from the JS module:

```sh
$ jco componentize add.js --wit add.wit -n example -o add.wasm
OK Successfully written add.wasm with imports ().
```

Now, run the component using the [Rust `add` host](./rust.md#creating-a-command-component-with-cargo-component):

```sh
$ cd component-model/examples/add-host
$ cargo run --release -- 1 2 ../path/to/add.wasm
1 + 2 = 3
```

### Running a Component from JavaScript Applications

As the JavaScript runtime cannot yet execute Wasm components, a component must be transpiled into
JavaScript and a core module and then executed. `jco` automates this transpilation:

```sh
$ jco transpile add.wasm -o out-dir

Transpiled JS Component Files:

 - out-dir/add.core.wasm  6.72 MiB
 - out-dir/add.d.ts       0.05 KiB
 - out-dir/add.js          0.8 KiB
```

A core module and JavaScript bindings have been outputted to the `out-dir`.

Now, you can import the resultant `add.js` file and run it from a JavaScript application. This
example renames it and imports it as an ECMAScript module for ease of running locally with node:

```mjs
// app.mjs
import { add } from "./out-dir/add.mjs";

console.log("1 + 2 = " + add(1, 2));
```

The above example :

```sh
$ mv out-dir/add.js out-dir/add.mjs
$ node app.mjs
1 + 2 = 3
```
