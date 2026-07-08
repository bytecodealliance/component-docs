# WIT By Example

This section includes three examples to introduce WIT: a simple "clocks" example, a more elaborate "filesystems" example, and a WASI 0.3 "CLI" example that introduces async functions, streams, and futures.

For a full WIT reference, see [the next section](./wit.md).

## Clocks

The following is a simplified version of the world defined in
the [wasi:clocks](https://github.com/WebAssembly/WASI/tree/main/proposals/clocks) package.

Suppose we want to write a component that provides clock functionality.
This component will represent a "wall clock", which can be reset
(the clock is not monotonic).
(The real `wasi:clocks` package provides two interfaces,
one for a wall clock and one for a monotonic clock.)

### Declaring a world

We declare a world that imports one interface:

```wit
{{#include ../../examples/wit-section-examples/clocks/world.wit}}
```

For exposition, version numbers have been removed.

This file contains a package declaration, which declares that
this world is in the `clocks` package in the `wasi-example` namespace.

The world is declared using the keyword `world`, followed by
the name `imports`.
World declarations must begin with `world`, but the name `imports`
is an arbitrary choice.
What follows is a list of `import` declarations enclosed in curly braces,
each of which consists of the `import` keyword
followed by the name of an interface.
Each declaration is followed by a semicolon.

### Declaring an interface: `wall-clock`

```wit
{{#include ../../examples/wit-section-examples/clocks/wall-clock.wit}}
```

Like a world, an interface is declared with a keyword (`interface`) in this case,
followed by a name, followed by a semicolon-separated list of declarations enclosed
in curly braces.
In this case, declarations are _type declarations_ or _function declarations_.


### Type declarations

_Record types_ are one of the possible types that can be declared in WIT.

```wit
record datetime {
    seconds: u64,
    nanoseconds: u32,
}
```

The `record` keyword is followed by a name, then by a list of
field declarations separated by commas.
Each field declaration is a field name (a string), followed by
a colon, followed by a type name.

A record is analogous to a `struct` in C or Rust,
in that it groups together named fields.
It is also analogous to a JavaScript object, except
that it has no methods or prototype.

In short, the `datetime` type is a record with two fields:
`seconds`, an unsigned 64-bit integer, and `nanoseconds`,
an unsigned 32-bit integer.

### Function declarations

The following declares a function named `now`:

```wit
now: func() -> datetime;
```

The empty parentheses `()` indicate that the function has no arguments.
The return type is the type after the final arrow (`->`),
which is `datetime`.
Putting it together: `now()` is a nullary function that returns a datetime.

### Summing up

The `imports` world contains an interface for wall clocks.
(Real worlds usually contain multiple interfaces.)
The wall clock world defines a record type that represents a time value
in terms of seconds and nanoseconds,
as well as a function to get the current time.


## WIT By Example: Filesystems

That was just a warm-up; let's look at an example that uses
more of WIT's built-in and user-defined types.

The following is a very simplified version of the main interface
defined in the [wasi-filesystem](https://github.com/WebAssembly/WASI/tree/main/proposals/filesystem) package.
Much of the functionality has been removed.
Here, a file descriptor supports just two operations:
* `open-at()`: Open a file.
* `read()`: Read from a file, starting at a particular offset.

```wit
{{#include ../../examples/wit-section-examples/filesystems/types.wit}}
```

Let's look at some WIT features used in this interface.

### Enums

```wit
enum error-code {
    access,
    bad-descriptor,
}
```

This declaration defines an enumeration type named `error-code`
with two alternatives: `access` and `bad-descriptor`.
The contents of the curly brackets is just a list of comma-separated names.
Enum types are similar to enums in C, and are useful for
expressing types that have a known, small set of values.
This declaration expresses the possible error codes
that filesystem operations can return.
In reality, there are many more possible errors,
which would be expressed by adding more alternatives to the enumeration.

### Resources

A resource describes an interface for objects.
This is not the same kind of "interface" as a WIT interface;
a WIT interface can contain many different `resource` declarations.
The declaration of the `descriptor` resource says that
a `descriptor` is an object that implements two methods:
`read` and `open-at`.
Let's look at the method declarations one at a time:

#### Reading from files

```wit
read: func(
    length: filesize,
    offset: filesize,
) -> result<tuple<list<u8>, bool>, error-code>;
```

Method declarations use the same syntax as regular function declarations,
like the ones we already saw in the clocks example.
This declaration says that the `read()` method has two arguments,
`length` and `offset`, both of which have type `filesize`.
The return type of `read` is a `result`.

`result` is another parameterized type, like `option`.
Let's look at the parameters before we look at the entire type:
* `list` is also a parameterized type; in this case,
  it's applied to `u8` (unsigned 8-bit integer),
  so `list<u8>` can be read as "list of bytes".
* `tuple` is like a list with a known size,
  whose elements can have different types.
  `tuple<list<u8>, bool>` represents a 2-tuple (pair)
  of a list of bytes and a boolean.
* `error-code` was defined as an `enum` type.

If `a` and `b` are both types, then `result<a, b>` represents
a type that can be either `a` or `b`.
Often, but not always, `b` is a type that represents an error,
like in this case.
So the type `result<tuple<list<u8>, bool>, error-code>` means
"either a tuple of a list of bytes and a bool; or an error code".

This makes sense for the `read()` function because it takes a
number of bytes to read and an offset within a file to start at;
and the result is either an error, or a list of bytes containing
the data read from the file,
paired with a boolean indicating whether the end of the file was
reached.

#### Opening files

The `open-at()` method is a constructor, which we know because
it returns a `descriptor` when it doesn't fail (remember that
these methods are attached to the resource type `descriptor`):

```wit
open-at: func(
    path: string,
) -> result<descriptor, error-code>;
```

`open-at()` returns a new descriptor, given a path string and flags.

## WASI 0.3 CLI

The two examples above use WIT features that have been part of the language since WASI 0.2.
WASI 0.3 added three new primitives to the Component Model's Canonical ABI:
[`async func`, `stream<T>`, and `future<T>`](./async.md).
This example walks through a simplified version of the
[`wasi:cli`](https://github.com/WebAssembly/WASI/tree/main/proposals/cli) package,
which exercises all three.

### Async functions

A function declared `async` may suspend before returning a result.
The runtime owns the scheduling; the guest sees an ordinary call and the host sees no busy loop:

```wit
package wasi-example:cli;

interface run {
  run: async func() -> result;
}
```

The `result` return type with no parameters means "either success or failure, with no value attached to either."
Bindings generators emit each side in the host language's natural async idiom —
an `async fn` in Rust, a `Promise`-returning function in JavaScript, and so on.

### Streams plus terminal futures

Reading from standard input pairs a `stream<T>` with a `future`:

```wit
interface stdin {
  use types.{error-code};
  read-via-stream: func() -> tuple<stream<u8>, future<result<_, error-code>>>;
}
```

The `stream<u8>` delivers bytes incrementally.
The `future` resolves once the operation has terminated, carrying either success
(the underscore means "no value attached") or an `error-code` (defined as an `enum` in the `types` interface).
The two halves are independent:
the caller can consume the stream eagerly, sample it, or drop it part-way through,
and the future signals the outcome either way.

Unlike resources, `stream<T>` and `future<T>` are *values*.
They can be returned from functions, accepted as parameters,
and passed across component boundaries the same way primitive types are.

### Inverted writes

Writing to standard output reverses the direction.
Instead of the host handing the guest a resource to write into,
the guest supplies its data as a `stream<u8>` parameter
and the host returns a `future` that resolves once the bytes are consumed:

```wit
interface stdout {
  use types.{error-code};
  write-via-stream: func(data: stream<u8>) -> future<result<_, error-code>>;
}
```

This shape — stream parameter, future return — appears throughout WASI 0.3 wherever a guest
writes data: stdout, stderr, filesystem writes, and TCP sends all follow it.

### Aggregating into a world

A world ties imports and exports together.
The `command` world below imports the I/O interfaces and exports `run`:

```wit
world command {
  import stdin;
  import stdout;
  export run;
}
```

A component implementing this world supplies an implementation of `run`,
and from inside that implementation may call `read-via-stream` on stdin and `write-via-stream` on stdout.

For a deeper look at the three primitives, including the composition story that motivated adding them,
see [Async, Streams, and Futures](./async.md).

## Further reading

We've seen how using rich types, WIT can encode a multitude
of ideas about how functions interrelate,
which are not available in the type system of core WebAssembly.

For more WIT examples, see the [tutorial](../tutorial.md) section.
The next section, [WIT Reference](./wit.md), covers WIT syntax
more thoroughly.
