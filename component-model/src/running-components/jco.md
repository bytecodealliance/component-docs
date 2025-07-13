# jco

[jco](https://github.com/bytecodealliance/jco) is a fully native JavaScript tool for working with components in JavaScript. It supports the [`wasi:cli/command` world](https://github.com/WebAssembly/wasi-cli/blob/main/wit/command.wit). `jco` also provides features for transpiling Wasm components to ES modules, and for building Wasm components from JavaScript and WIT.

To run a component with `jco`, run:

```sh
jco run <path-to-wasm-file> <command-args...>
```

`jco`'s WASI implementation grants the component full access to the underlying system resources. For example, the component can read all environment variables of the `jco` process, or read and write files anywhere in the file system.
