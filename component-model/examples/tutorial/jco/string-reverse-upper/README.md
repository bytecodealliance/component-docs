# `reverse()` + `toUpper()` in JavaScript, with WebAssembly composition

This folder contains a Javascript project can build into a WebAssembly component binary.

This component *uses* functionality provided by another binary to export *new* functionality, with the following interface:

```wit
package example:string-reverse-upper@0.1.0

@since(version = 0.1.0)
interface reversed-upper {
    reverse-and-uppercase: func(s: string) -> string;
}

world revup {
    //
    // NOTE, the import below translates to:
    // <namespace>:<package>/<interface>@<package version>
    //
    import example:reverse-string/reverse@0.1.0;

    export reversed-upper;
}
```

This is a `node` CLI and browser based example implementation of running a component that exports the `reversed-upper` interface from a JavaScript application. 

It uses [`jco`](https://bytecodealliance.github.io/jco/) to:

- Generate a WebAssembly component (`jco componentize`) that can be executed anywhere WebAssembly components run
- Generate JavaScript bindings (`jco transpile`) that execute the core functionality (in browser or compliant JS runtimes like NodeJS) 
- Build a component that *composes with another component*

For another example of using `jco` with components in multiple environments, see the [`jco` example](https://github.com/bytecodealliance/jco/blob/main/docs/src/example.md).

# Quickstart

First, install required dependencies:

```console
npm install
```

Then, build the component with `jco`:

```console
npm run build
```

A WebAssembly binary will be written to `string-reverse-upper.wasm`.

While somewhat redundant, to use the produced WebAssembly binary (keep in mind the WebAssembly binary could be written in *any* underlying language), we can use `jco` to transpile it to run in Javascript natively:

Since this component *uses* another component, we must *compose* the two components together, given that we have code which satisfies the `import`.

> [!WARN]
> The command below will attempt to build the WebAssembly component in `tutorial/jco/string-reverse`

```console
npm run compose
```

You should see output like the following:

```
```

After running component composition, there will be a component with all it's imports satisfied, called `string-reverse-upper.composed.wasm`. 

We can transpile that *composed* component to a JS module:

```console
npm run transpile
```

Transpilation produces files in `dist/transpiled` that enable the WebAssembly component (`string-reverse.wasm`) to run in any compliant JS runtime:

```
dist
└── transpiled
    ├── reverser.core.wasm
    ├── reverser.d.ts
    └── reverser.js
```

With this transpiled code available, we can now run native NodeJS code that will *use* the WebAssembly module:

```
npm run transpiled-js
```
