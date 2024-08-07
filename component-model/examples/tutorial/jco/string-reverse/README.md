# Simple WebAssembly `reverser` in JavaScript

This folder contains a simple Javascript project can build into a WebAssembly binary.

This is a `node` CLI and browser based example implementation of running a component that exports the `reverse` interface from a JavaScript application. 

It uses [`jco`](https://bytecodealliance.github.io/jco/) to:

- Generate a WebAssembly component (`jco componentize`) that can be executed anywhere WebAssembly components run
- Generate JavaScript bindings (`jco transpile`) that execute the core functionality (in browser or compliant JS runtimes like NodeJS) 

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

A WebAssembly binary will be written to `string-reverse.wasm`.

While somewhat redundant, to use the produced WebAssembly binary (keep in mind the WebAssembly binary could be written in *any* underlying language), we can use `jco` to transpile it to run in Javascript natively:

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
