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

1. Determining which interface our component will target (i.e. given a [WebAssembly Interface Types ("WIT")](../design/wit.md) world)
2. Creating the component by writing JavaScript that satisfies the interface
3. Compiling the interface-compliant JavaScript to WebAssembly

### What is WIT?

[WebAssembly Interface Types ("WIT")](../design/wit.md) is a featureful Interface Definition Language ("IDL")
for defining functionality, but most of the time, you shouldn't need to write WIT from scratch.
Often, it's sufficient to download a pre-existing interface that defines what your component should do.

The [`adder` world][adder-world]
contains an interface with a single `add` function that sums two numbers.
Create a new directory called `adder` and paste the following WIT code
into a file called `world.wit`.

```wit
{{#include ../../examples/tutorial/wit/adder/world.wit}}
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

> To learn more about the WIT syntax, check out the [WIT explainer](../design/wit.md).

[adder-world]: https://github.com/bytecodealliance/component-docs/blob/main/component-model/examples/tutorial/wit/adder/world.wit
[wit-example-world]: https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/example-host/add.wit

## Implementing a JS WebAssembly Component

To implement the [`adder` world][adder-world], we can write a [JavaScript ES module][mdn-js-module].
Paste the following code into a file called `adder.js` in your `adder` directory:

```js
{{#include ../../examples/tutorial/js/adder/adder.js}}
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

{{#include example-host-part1.md}}

The output looks like:

{{#include example-host-part2.md}}

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
{{#include ../../examples/tutorial/js/adder/run.js}}
```

Pasting this code into a file called `run.js` in your `adder` directory,
you can execute the JavaScript module with `node` directly.
First, you will need to create a `package.json` file
in the same directory:

```json
{{#include ../../examples/tutorial/js/adder/package.json}}
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

## Building Reactor Components with `jco`

Reactor components are WebAssembly components that are long-running
and meant to be called repeatedly over time.
Unlike "command" components, which are analogous to executables,
reactor components are analogous to libraries of functionality.

Components expose their interfaces via [WebAssembly Interface Types][docs-wit],
hand-in-hand with the [Component Model][docs-component-model]
which enables components to use higher-level types interchangeably.

[docs-wit]: ../design/wit.md
[docs-component-model]: ../design/why-component-model.md

### Exporting WIT Interfaces with `jco`

Packaging reusable functionality into WebAssembly components isn't useful
if we have no way to *expose* that functionality.
This section offers a slightly deeper dive into the usage of WIT in WebAssembly components.

As in the previous example, `export`ing WIT interfaces for other components (or a WebAssembly host) to use
is fundamental to developing WebAssembly programs.

Let's examine a [`jco` example project called `string-reverse`][jco-examples-string-reverse]
that exposes functionality for reversing a string.

To build a project like `string-reverse` from the ground up, first we'd start with a WIT like the following.
In a new directory called `string-reverse`, paste this code into a file called `wit/component.wit`:

```wit
{{#include ../../examples/tutorial/js/string-reverse/component.wit}}
```

As a slightly deeper crash course on [WIT](../design/wit.md), here's what the above code describes:

- We've defined a namespace called `example`.
- We've defined a package called `string-reverse` inside the `example` namespace.
- This WIT file corresponds to version `0.1.0` of the `example:string-reverse` package.
- We've defined an interface called `reverse` that contains *one* function called `reverse-string`.
- We specify that the `reverse` interface has existed *since* the `0.1.0` version.
- The `reverse-string` function (whose fully qualified name is
  `example:string-reverse/reverse.reverse-string`) takes a string and returns a string.
- We've defined a `world` called `string-reverse` that exports the functionality
  provided by the `reverse` interface.

> [!WARNING]
> How do we *know* that `reverse` actually reverses a string?
>
> Unfortunately, that problem is not really solvable at this level—this is between you
> and the writer of the component that implements the WIT interface.
>
> Of course, with WebAssembly, you *can* enforce static checks if you're so inclined, *before* you run any given binary.

OK now let's see what the JS code looks like to *implement* the `component` world.
Paste the following code into a file called `string-reverse.mjs`:

```mjs
{{#include ../../examples/tutorial/js/string-reverse/string-reverse.mjs}}
```

> This code uses `split()` to convert the string into an array of characters,
> reverses the array, and uses `join()` to convert the array back to a string,
> since JavaScript has no built-in string reverse method.

> [!NOTE]
> To view the full code listing along with instructions, see the [`examples/tutorials/jco/string-reverse` folder][jco-examples-string-reverse].

To use `jco` to compile this component, you can run the following inside your `string-reverse` directory:

```console
npx jco componentize \
    --wit wit/component.wit \
    --world-name string-reverse \
    --out string-reverse.wasm \
    --disable=all \
    string-reverse.mjs
```

You should see output like the following:

```
OK Successfully written string-reverse.wasm.
```

> [!NOTE]
> As with the previous example, we're not using any of the advanced [WebAssembly System Interface][wasi] features,
> so we `--disable` all of them.
>
> Rather than typing out the `jco componentize` command manually, you can also run
> the build command with `npm run build` if you use the code from
> [the `string-reverse` folder][string-reverse-package-json].


Now that we have a WebAssembly binary, we can *also* use `jco` to run it in a native JavaScript context by *transpiling*
the WebAssembly binary (which could have come from anywhere!) to a JavaScript module.

```console
npx jco transpile string-reverse.wasm -o dist/transpiled
```

You should see output that looks like this:

```
  Transpiled JS Component Files:

 - dist/transpiled/interfaces/example-string-reverse-reverse.d.ts   0.1 KiB
 - dist/transpiled/string-reverse.core.wasm                        10.1 MiB
 - dist/transpiled/string-reverse.d.ts                             0.15 KiB
 - dist/transpiled/string-reverse.js                               2.55 KiB
```

> [!TIP]
> A gentle reminder: transpilation *does* produce a [TypeScript declaration file][ts-decl-file],
> for use in TypeScript projects.

Now that we have a transpiled module, we can run it from any JavaScript context
that supports core WebAssembly (whether Node.js or the browser).

For Node.js, we can use code like this.
Paste the following code into a file called `run.js` in your `string-reverse` directory:

```mjs
{{#include ../../examples/tutorial/js/string-reverse/run.js}}
```

> [!NOTE]
> In the `jco` example project, you can run `npm run transpiled-js` to build the existing code.

As before, we also need a `package.json` file:

```json
{{#include ../../examples/tutorial/js/string-reverse/package.json}}
```

Then run:

```bash
node run.js
```

Assuming you have the `dist/transpiled` folder populated (by running `jco transpile` in the previous step),
you should see output like the following:

```
reverseString('!dlrow olleh') = hello world!
```

While it's somewhat redundant in this context, what we've done from NodeJS demonstrates the usefulness of WebAssembly and the `jco` toolchain.
With the help of `jco`, we have:

- Compiled JavaScript to a WebAssembly module (`jco compile`), adhering to an interface defined via WIT
- Converted the compiled WebAssembly module (which could be from *any* language) to a module that can be used from any compliant JS runtime (`jco transpile`)
- Run the transpiled WebAssembly component from a JavaScript native runtime (NodeJS)

[repo]: https://github.com/bytecodealliance/component-docs
[jco-examples-string-reverse]: https://github.com/bytecodealliance/jco/tree/main/examples/components/string-reverse
[ts-decl-file]: https://www.typescriptlang.org/docs/handbook/declaration-files/deep-dive.html#declaration-file-theory-a-deep-dive
[string-reverse-package-json]: https://github.com/bytecodealliance/jco/blob/main/examples/components/string-reverse/package.json#L6

### Advanced: Importing and Reusing WIT Interfaces via Composition

Just as `export`ing functionality is core to building useful WebAssembly components,
`import`ing and reusing functionality is key to using the strengths of WebAssembly.

Restated, **WIT and the Component Model enable WebAssembly to *compose***. This means we can build on top of functionality
that already exists and `export` *new* functionality that depends on existing functionality.

Let's say in addition to eversing the string (in the previous example),
we want to build shared functionality that *also* upper-cases the text it receives.

We can reuse the reversing functionality *and* export a new interface which enables us to reverse and upper-case.

Let's examine a [`jco` example project called `string-reverse-upper`][jco-examples-string-reverse-upper] that exposes
functionality for reversing *and* upper-casing a string.

Here's the WIT one might write to enable this functionality:

```wit
{{#include ../../examples/tutorial/js/string-reverse-upper/component.wit}}
```

This time, the `world` named `revup` that we are building *relies* on the interface `reverse`
in the package `string-reverse` from the namespace `example`.

We can make use of *any* WebAssembly component that matches that interface,
as long as we *compose* their functionality with the component that implements the `revup` world.

The `revup` world `import`s (and makes use) of `reverse` in order to `export` (provide) the `reversed-upper` interface,
which contains the `reverse-and-uppercase` function (in JavaScript, `reverseAndUppercase`).

> [!NOTE]
> Functionality is imported via the `interface`, *not* the `world`.
> `world`s can be included/used, but the syntax is slightly different for that.

The JavaScript to make this work ([`string-reverse-upper.mjs` in `jco/examples`][string-reverse-upper-mjs])
looks like this:

```js
{{#include ../../examples/tutorial/js/string-reverse-upper/string-reverse-upper.mjs}}
```

If we place the above WIT file in the `wit` subdirectory, we also need to create a
`wit/deps` subdirectory and copy `../string-reverse/wit/component.wit` into `wit/deps`.

We can build the component with `jco componentize`:

```console
npx jco componentize \
    string-reverse-upper.mjs \
    --wit wit/ \
    --world-name revup \
    --out string-reverse-upper.incomplete.wasm \
    --disable=all
```

> If you get an error message, verify that your `wit/component.wit` file
> begins with `package example:string-reverse-upper@0.1.0;`, and that your `wit/deps/` directory
> contains a file beginning with `package example:string-reverse@0.1.0;`.
> In general, your main package should be at the top level of your `wit` directory,
> and any dependencies should be in a subdirectory of that directory (normally `deps`).

Although we've successfully built a WebAssembly component, unlike with the other examples,
ours is *not yet complete*.

We can see that if we print the WIT of the generated component by running `jco wit`:

```console
npx jco wit string-reverse-upper.incomplete.wasm
```

You should see output like the following:

```
package root:component;

world root {
  import example:string-reverse/reverse@0.1.0;

  export example:string-reverse-upper/reversed-upper@0.1.0;
}
```

This tells us that the component still has *unfulfilled `import`s*:
we *use* the `reverseString` function that's in `reverse` as if it exists,
but it's not yet a real part of the WebAssembly component (hence we've named it `.incomplete.wasm`).

To compose the two components we built earlier (`string-reverse-upper/string-reverse-upper.incomplete.wasm` and `string-reverse/string-reverse.wasm`),
we'll need the [WebAssembly Composition tool (`wac`)][wac]. We can use `wac plug`:

```console
wac plug \
    -o string-reverse-upper.wasm \
    --plug ../string-reverse/string-reverse.wasm \
    string-reverse-upper.incomplete.wasm
```

> [!NOTE]
> You can also run this step with `npm run compose`, if using the full project from the `jco` repository.

A new component `string-reverse-upper.wasm` should now be present, which is a "complete" component.
We can check the output of `npx jco wit` to ensure that all the imports are satisfied:

```sh
npx jco wit string-reverse-upper.wasm
```

You should see output like the following:

```wit
package root:component;

world root {
  export example:string-reverse-upper/reversed-upper@0.1.0;
}
```

It's as-if we never imported any functionality at all—the functionality present in `string-reverse.wasm`
has been *merged into* `string-reverse-upper.wasm`, and it now simply `export`s the advanced functionality.

We can run this completed component with in any WebAssembly-capable native JavaScript environment
by using the transpiled result:

```console
npx jco transpile string-reverse-upper.wasm -o dist/transpiled
```

> [!NOTE]
> In the example project, you can run `npm run transpile` instead,
> which will also change the extension on `dist/transpiled/string-reverse-upper.js` to `.mjs`.

You should see output like the following:

```
  Transpiled JS Component Files:

 - dist/transpiled/interfaces/example-string-reverse-upper-reversed-upper.d.ts  0.12 KiB
 - dist/transpiled/string-reverse-upper.core.wasm                               10.1 MiB
 - dist/transpiled/string-reverse-upper.core2.wasm                              10.1 MiB
 - dist/transpiled/string-reverse-upper.d.ts                                    0.19 KiB
 - dist/transpiled/string-reverse-upper.js                                      6.13 KiB
```

> [!TIP]
> Notice that there are *two* core WebAssembly files. That's because two core WebAssembly modules were involved
> in creating the ultimate functionality we needed.

To run the transpiled component, we can write code like the following:

```js
{{#include ../../examples/tutorial/js/string-reverse-upper/run.js}}
```

> [!NOTE]
> In the [`jco` example project][jco-examples-string-reverse-upper],
> you can run `npm run transpiled-js`.

You should see output like the following:

```
reverseAndUppercase('!dlroW olleH') = HELLO WORLD!
```

[wac]: https://github.com/bytecodealliance/wac
[jco-examples-string-reverse-upper]: https://github.com/bytecodealliance/jco/tree/main/examples/components/string-reverse-upper
[string-reverse-upper-mjs]: https://github.com/bytecodealliance/jco/blob/main/examples/components/string-reverse-upper/string-reverse-upper.mjs

[!TIP]: #
[!NOTE]: #
[!WARNING]: #
