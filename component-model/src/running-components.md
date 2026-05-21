# Running Components

There are two standard WIT worlds that runtimes support.
These worlds are the [`wasi:cli/command` world](https://github.com/WebAssembly/WASI/blob/main/proposals/cli/wit/command.wit)
and the [`wasi:http/proxy` world](https://github.com/WebAssembly/WASI/blob/main/proposals/http/wit/proxy.wit).
All other WIT worlds and interfaces are considered to be custom.
In the following sections, you'll see how to run components that implement either world, as well as how to invoke custom exports.
