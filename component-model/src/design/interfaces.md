# Interfaces

An **interface** describes a single-focus, composable contract, through which components can interact with each other and with hosts. Interfaces describe the types and functions used to carry out that interaction. For example:

* A "receive HTTP requests" interface might have only a single "handle request" function, but contain types representing incoming requests, outgoing responses, HTTP methods and headers, and so on.
* A "wall clock" interface might have two functions, one to get the current time and one to get the granularity of the timer. It would also include a type to represent an instant in time.

Interfaces are defined using [the WIT language](./wit.md).

> ⓘ For a more formal definition of what an interface is, take a look at the [WIT specification](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md).
