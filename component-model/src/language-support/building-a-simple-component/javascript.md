# JavaScript Tooling

[WebAssembly][mdn-wasm] was originally developed as a technology for running non-JavaScript
workloads in the browser at near-native speed.

JavaScript WebAssembly component model support is provided by a combination of tools:

- [StarlingMonkey][sm], a WebAssembly component-aware JavaScript engine
- [`componentize-js`][componentize-js], a tool for building WebAssembly components from JavaScript files
- [`jco`][jco], a multi-tool for componentizing, type generation, and running components in Node.js and browser contexts

Note that [TypeScript][ts] can *also* be used, given that it is transpiled to JS first by relevant tooling (`tsc`).
`jco` generates [type declaration files (`.d.ts`)][ts-decl-file] by default,
and also has a `jco types` subcommand which generates typings that can be used with a TypeScript codebase.

> [!WARNING]
> While popular projects like [`emscripten`][emscripten] also build WebAssembly modules,
> those modules are not Component Model aware.
>
> Core WebAssembly modules do not contain the advanced features
> (rich types, structured language interoperation, composition)
> that the component model makes available.

[emscripten]: https://emscripten.org/
[ts]: https://typescriptlang.org
[mdn-wasm]: https://developer.mozilla.org/en-US/docs/WebAssembly
[jco]: https://github.com/bytecodealliance/jco
[componentize-js]: https://github.com/bytecodealliance/componentize-js
[sm]: https://github.com/bytecodealliance/StarlingMonkey

## Installing `jco`

[`jco`][jco] (which uses [`componentize-js`][componentize-js] can be installed through
the Node Package Manager (`npm`):

```console
npm install -g @bytecodealliance/jco
```

> [!NOTE]
> `jco` and `componentize-js` can be installed in a project-local manner with `npm install -D`.

## Overview of Building a Component with JavaScript

Building a WebAssembly component with JavaScript often consists of:

1. Determining which interface our component will target (i.e. given a [WebAssembly Interface Types ("WIT")][docs-wit] world)
2. Creating the component by writing JavaScript that satisfies the interface
3. Compiling the interface-compliant JavaScript to WebAssembly

### Building Reactor Components with `jco`

Reactor components are WebAssembly components that are long-running
and meant to be called repeatedly over time.
Unlike "command" components, which are analogous to executables,
reactor components are analogous to libraries of functionality.

Components expose their interfaces via [WebAssembly Interface Types][docs-wit],
hand-in-hand with the [Component Model][docs-component-model]
which enables components to use higher-level types interchangeably.

[docs-wit]: ../../design/wit.md
[docs-component-model]: ../../design/why-component-model.md

### What is WIT?

[WebAssembly Interface Types ("WIT")][docs-wit] is a featureful Interface Definition Language ("IDL")
for defining functionality, but most of the time, you shouldn't need to write WIT from scratch.
Often, it's sufficient to download a pre-existing interface that defines what your component should do.

The [`adder` world][adder-world]
contains an interface with a single `add` function that sums two numbers.
Create a new directory called `adder` and paste the following WIT code
into a file called `world.wit`.

```wit
{{#include ../../../examples/tutorial/wit/adder/world.wit}}
```

The `export add;` declaration inside the `adder` world means that
environments that interact with the resulting WebAssembly component
will be able to _call_ the `add` function.
The fully qualified name of the `add` interface in this context is `docs:adder/add.add@0.1.0`.
The parts of this name are:
* `docs:adder` is the namespace and package, with `docs` being the namespace and `adder` being the package.
* `add` is the name of the interface containing the `add` function.
* `add` also happens to be the name of the function itself.
* `@0.1.0` is a version number that must match the declared version number of the package.

> To learn more about the WIT syntax, check out the [WIT explainer][docs-wit].

[adder-world]: https://github.com/bytecodealliance/component-docs/blob/main/component-model/examples/tutorial/wit/adder/world.wit
[wit-example-world]: https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/example-host/add.wit

## Implementing a JS WebAssembly Component

To implement the [`adder` world][adder-world], we can write a [JavaScript ES module][mdn-js-module].
Paste the following code into a file called `adder.js` in your `adder` directory:

```js
{{#include ../../../examples/tutorial/js/adder/adder.js}}
```

> [!WARNING]
> If you create a JavaScript project using this file,
> make sure you set the [`"type":"module"` option][package-json] in `package.json`,
> as `jco` works exclusively with JavaScript modules.

In the code above:

- The JavaScript module (file) itself is analogous to the `adder` world
- The exported `add` object corresponds to the `export`ed `add` interface in WIT
- The `add` function defined inside the `add` object corresponds to
  the `add` function inside the `add` interface

With the WIT and JavaScript in place, we can use [`jco`][jco] to create a WebAssembly component from the JS module, using `jco componentize`.

> [!NOTE]
> You can also call [`componentize-js`][componentize-js] directly—it can be used
> both through an API and through the command line.

Our component is *so simple* (reminiscent of [Core WebAssembly][wasm-core], which deals only in numeric values)
that we're actually *not using* any of the [WebAssembly System Interface][wasi] functionality
(access to files, networking, and other system capabilities).
This means that we can `--disable` all unneeded WASI functionality when we invoke `jco componentize`.

Inside your `adder` directory, execute:

```console
jco componentize \
    --wit world.wit \
    --world-name adder \
    --out adder.wasm \
    --disable=all \
    adder.js
```

> [!NOTE]
> If you're using `jco` as a project-local dependency, you can run `npx jco`.

You should see output like the following:

```
OK Successfully written adder.wasm.
```

You should now have an `adder.wasm` file in your `adder` directory.
You can verify that this file contains a component with:

```console
$ wasm-tools print adder.wasm | head -1
(component
```

> [!WARNING]
> By using `--disable=all`, your component won't get access to any WASI interfaces that
> might be useful for debugging or logging.
>
> For example, you can't `console.log(...)` or `console.error(...)` without `stdio`;
> you can't use `Math.random()` without `random`;
> and you can't use `Date.now()` or `new Date()` without `clocks`.
>
> Please note that calls to `Math.random()` or `Date.now()` will return seemingly valid
> outputs, but without actual randomness or timestamp correctness.

[mdn-js-module]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Modules
[package-json]: https://nodejs.org/api/packages.html#type

## Running the Component in the `example-host`

> [!NOTE]
> The [`example-host` Rust project][example-host] uses the [Rust toolchain][rust-toolchain],
> in particular [`cargo`][cargo],
> so to run the code in this section you may need to install some more dependencies (like the Rust toolchain).

To run the component we've built, we can use the [`example-host` project][example-host]:

{{#include ../example-host-part1.md}}

The output looks like:

{{#include ../example-host-part2.md}}

While the output isn't exciting, the code contained in `example-host` does a lot to make it happen:

- Loads the WebAssembly binary at the provided path (in the command above, `/path/to/adder.wasm`)
- Calls the `export`ed `add` function inside the `add` interface with arguments
- Prints the result

The important Rust code looks something like this:

```rust
let component = Component::from_file(&engine, path).context("Component file not found")?;

let (instance, _) = Example::instantiate_async(&mut store, &component, &linker)
    .await
    .context("Failed to instantiate the example world")?;

instance
    .call_add(&mut store, x, y)
    .await
    .context("Failed to call add function")
```

A quick reminder on the power and new capabilities afforded by WebAssembly:
we've written, loaded, instantiated and executed JavaScript from Rust with a strict interface,
without the need for [foreign function interfaces][ffi], subprocesses or a network call.

[rust-toolchain]: https://www.rust-lang.org/tools/install
[example-host]: https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/example-host
[nodejs]: https://nodejs.org/en
[cargo]: https://doc.rust-lang.org/cargo
[wasi]: https://wasi.dev/
[wasm-core]: https://webassembly.github.io/spec/core/
[ffi]: https://en.wikipedia.org/wiki/Foreign_function_interface

## Running a Component from JavaScript Applications (including the Browser)

While JavaScript runtimes available in browsers can execute WebAssembly core modules,
they cannot yet execute WebAssembly *components*, so WebAssembly components (JavaScript or otherwise)
must be "transpiled" into a JavaScript wrapper and one or more [WebAssembly core modules][wasm-core-module]
which *can* be run by browsers.

Given an existing WebAssembly component (e.g. `adder.wasm` which implements the [`adder` world][adder-world]),
we can transpile the WebAssembly component into runnable JavaScript by using `jco transpile`.
In your `adder` directory, execute:

```console
jco transpile adder.wasm -o dist/transpiled
```

You should see output similar to the following:

```
 Transpiled JS Component Files:

 - dist/transpiled/adder.core.wasm                 10.6 MiB
 - dist/transpiled/adder.d.ts                      0.11 KiB
 - dist/transpiled/adder.js                        21.1 KiB
 - dist/transpiled/interfaces/docs-adder-add.d.ts   0
```

> [!NOTE]
> For a complete project containing JS and WIT files similar to the ones you already created,
> see the [`jco` example `adder` component][jco-example].
>
> With this project pulled locally, you also run `npm run transpile`, which outputs to `dist/transpiled`.

Thanks to `jco` transpilation, you can import the resulting `dist/transpiled/adder.js` file
and run it from any JavaScript application
using a runtime that supports the [core WebAssembly specification][core-wasm] as implemented for JavaScript.

To use this component from [Node.js][nodejs], you can write code like the following:

```mjs
{{#include ../../../examples/tutorial/js/adder/run.js}}
```

Pasting this code into a file called `run.js` in your `adder` directory,
you can execute the JavaScript module with `node` directly.
First, you will need to create a `package.json` file
in the same directory:

```json
{{#include ../../../examples/tutorial/js/adder/package.json}}
```

> [!NOTE]
> Without creating the `package.json` file, or if you omit the `"type": "module"` property,
> you will see an error message like:
>
> `SyntaxError: Cannot use import statement outside a module`.

Then you can run the module with:

```console
node run.js
```

You should see output like the following:

```
1 + 2 = 3
```

This is directly comparable to the Rust host code mentioned in the previous section.
Here, we are able to use Node.js as a host for running WebAssembly,
thanks to `jco`'s ability to transpile components.

With `jco transpile`, any WebAssembly binary (compiled from any language) can be run natively in JavaScript.

[jco-example]: https://github.com/bytecodealliance/jco/tree/main/examples/components/adder
[wasm-core-module]: https://webassembly.github.io/spec/core/binary/modules.html
[core-wasm]: https://webassembly.github.io/spec/core/

[!TIP]: #
[!NOTE]: #
[!WARNING]: #
