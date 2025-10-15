# Importing and Reusing components (Javascript)

## Composing existing code for use in a Javascript component

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
{{#include ../../../examples/tutorial/js/string-reverse-upper/component.wit}}
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
{{#include ../../../examples/tutorial/js/string-reverse-upper/string-reverse-upper.mjs}}
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
{{#include ../../../examples/tutorial/js/string-reverse-upper/run.js}}
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

## Using `jco transpile` to run components from Javsacript

Packaging reusable functionality into WebAssembly components isn't useful
if we have no way to *expose* that functionality.
This section offers a slightly deeper dive into the usage of WIT in WebAssembly components.

`export`ing WIT interfaces for other components (or a WebAssembly host) to use
is fundamental to developing WebAssembly programs.

Let's examine a [`jco` example project called `string-reverse`][jco-examples-string-reverse]
that exposes functionality for reversing a string.

To build a project like `string-reverse` from the ground up, first we'd start with a WIT like the following.
In a new directory called `string-reverse`, paste this code into a file called `wit/component.wit`:

```wit
{{#include ../../../examples/tutorial/js/string-reverse/component.wit}}
```

As a slightly deeper crash course on [WIT][docs-wit], here's what the above code describes:

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
{{#include ../../../examples/tutorial/js/string-reverse/string-reverse.mjs}}
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
{{#include ../../../examples/tutorial/js/string-reverse/run.js}}
```

> [!NOTE]
> In the `jco` example project, you can run `npm run transpiled-js` to build the existing code.

As before, we also need a `package.json` file:

```json
{{#include ../../../examples/tutorial/js/string-reverse/package.json}}
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
