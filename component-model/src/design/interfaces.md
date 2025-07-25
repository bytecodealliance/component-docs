# Interfaces

Interfaces are based on the idea of [design by contract][wp-contract].
In software design, a _contract_ is a specification
of how a unit of code should behave.

Concretely, an interface is a collection of type definitions
and function declarations.
Conceptually, an _interface_ describes a single-focus, composable contract
through which components can interact with each other
and with hosts.
* _Single-focus_: By convention, an interface describes
  types and functions that are related to each other
  and collectively provide a relatively small unit of
  functionality,
  such as reading from the standard input stream
  in a command-line environment.
* _Composable_: Interfaces can be imported and exported.
  One component's interfaces can be built
  on top of interfaces defined in a different component.
  Interfaces enable typechecking so that interfaces can
  be composed only when it makes sense to do so.

Concretely, an interface is a collection of type definitions
and function declarations
that are used to enable interactions between components and hosts.
For example:

* A "receive HTTP requests" interface might declare
  a single "handle request" function,
  along with definitions of types representing
  incoming requests, outgoing responses,
  HTTP methods and headers, and other data structures.
  This might look like the `incoming-handler` interface
  in [wasi-http][wasi-http-handler]
* A "wall clock" interface might declare two functions,
  one to get the current time
  and one to get the granularity of the timer (whether time
  is measured in seconds, milliseconds, nanoseconds, or another unit).
  It would also define a type to represent an instant in time.
  This might look like the `wall-clock` interface
  in [wasi-clocks][wasi-clocks-wall-clock].

As an example of composing interfaces together,
imagine defining a "timer" interface that declares two functions,
one to start a timer and one to query whether the timeout
has been exceeded.
This interface could be defined by importing the "wall clock"
interface.
The result is an interface that exports the timer functionality,
and imports anything imported by the "wall clock" interface.

Interfaces are defined using [the WIT language](./wit.md).

[wp-contract]: https://en.wikipedia.org/wiki/Design_by_contract
[wasi-http-handler]: https://github.com/WebAssembly/wasi-http/blob/main/wit/handler.wit
[wasi-clocks-wall-clock]: https://github.com/WebAssembly/wasi-clocks/blob/main/wit/wall-clock.wit

> For a more formal definition of an interface, take a look at the [WIT specification](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md).
