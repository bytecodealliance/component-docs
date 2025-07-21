# WIT By Example

This section includes two examples to introduce WIT:
a simpler "clocks" example and a more complicated "filesystems" example.
For a full WIT reference, see [the next section](./wit.md).

## Clocks

The following is a simplified version of the world defined in
the [wasi-clocks](https://github.com/WebAssembly/wasi-clocks) package.

Suppose we want to write a component that provides clock functionality.
Our component will provide two sets of functionality,
each corresponding to its own interface:
* A monotonic clock, whose time is guaranteed to always increase in value.
* A wall clock, which can be reset.

### Declaring a world

We declare a world that imports two interfaces:

```wit
{{#include ../../examples/wit-section-examples/clocks/world.wit}}
```

For exposition, version numbers have been removed.

This file contains a package declaration, which declares that
this world is in the `clocks` package in the `wasi` namespace.

The world is declared using the keyword `world`, followed by
the name `imports`.
World declarations must begin with `world`, but the name `imports`
is an arbitrary choice.
What follows is a list of `import` declarations enclosed in curly braces,
each of which consists of the `import` keyword
followed by the name of an interface.
Each declaration is followed by a semicolon.

### Declaring an interface: `monotonic-clock`

A separate file, `monotonic-clock.wit`, defines the `monotonic-clock`
interface:

```wit
{{#include ../../examples/wit-section-examples/clocks/monotonic-clock.wit}}
```

Anything beginning with `///` is a comment.

Like a world, an interface is declared with a keyword (`interface`) in this case,
followed by a name, followed by a semicolon-separated list of declarations enclosed
in curly braces.
In this case, declarations are _type declarations_ or _function declarations_.

### Type declarations

The following is an example of a _type alias_:

```wit
type instant = u64;
```

Type declarations can be more complex, but this example only uses type aliases.
This declaration declares a type named `instant` that is a synonym for the type `u64`
(unsigned 64-bit integers).
In other words, this interface represents an instant in time as
an unsigned 64-bit integer.
The declaration for `duration` is similar.

### Function declarations

The following declares a function named `now`:

```wit
now: func() -> instant;
```

The empty parentheses `()` indicate that the function has no arguments.
The return type is the type after the final arrow (`->`),
which is `instant`.
Putting it together: `now()` is a nullary function that returns an instant.

Similarly, `resolution()` is a nullary function that returns a duration.

### Declaring an interface: `wall-clock`

```wit
{{#include ../../examples/wit-section-examples/clocks/wall-clock.wit}}
```

This interface looks similar to `monotonic-clock`, declaring a type `datetime`
and two functions `now` and `resolution`, but it contains a more complicated
type declaration.

### More type declarations

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

### Summing up

The `imports` world contains two interfaces, one for monotonic clocks
and one for wall clocks.
The monotonic clock world defines types to represent instants and durations,
along with functions to get the current time and the resolution of the clock.
The wall clock world defines a record type that represents a time value
differently,
as well as functions to get the current time and the resolution of the clock
that return this record type.


## WIT By Example: Filesystems

That was just a warm-up; let's look at an example that uses
more of WIT's built-in and user-defined types.

The following is a simplified version of the main interface
defined in the [wasi-filesystem](https://github.com/WebAssembly/wasi-filesystem) package.
Much of the functionality has been removed in order to show
the minimal amount of functionality to exhibit all of the
types that are available in WIT.
Many of the types have been simplified by removing
some of the alternatives from enums
and some of the fields from records.
We've even removed the ability to write to files.
Here, a file descriptor supports four operations:
* `open-at()`: Open a file.
* `read()`: Read from a file.
* `stat()`: Get file metadata.
* `set-times-at()`: Set the access time on a file.

```wit
{{#include ../../examples/wit-section-examples/filesystems/types.wit}}
```

Let's look at the contents of this interface piece by piece.

### Enums

```wit
enum descriptor-type {
    directory,
    regular-file,
}
```

This declaration defines an enumeration type named `descriptor-type`
with two alternatives: `directory` and `regular-file`.
The contents of the curly brackets is just a list of comma-separated names.
Enum types are similar to enums in C, and are useful for
expressing types that have a known, small set of values.
This declaration expresses that a descriptor type
can be either a directory or a regular file.
This corresponds to a simplified model of a filesystem
where files can either be directories or non-directories,
with no other file types.

### Options

```wit
record descriptor-stat {
    %type: descriptor-type,
    size: filesize,
    data-access-timestamp: option<datetime>,
}
```

This declaration expresses the metadata for a file descriptor.
In our simplified example, that metadata correspond to
the type of the file, the size of the file, and the timestamp
at which the file was last accessed.

We already encountered record types, but this record type
has a new type in one of its fields, `option`.
The `option` type is parameterized, like templates in C++ or generics in Java:
you can read `option<datetime>` as "option of datetime".
It represents a value that may or may not be present,
and is used here to model the fact that some filesystems
don't track access timestamps.

Note that the `datetime` type is defined by the `wasi:clocks/wall-clock` interface,
as indicated by the `use` declaration at the beginning of the `types` interface.

### Flags

```wit
flags open-flags {
    create,
    directory,
}
```

A `flags` type is similar to a `record` type, but with
the restriction that all the fields have type `bool` (boolean).
Flags are represented efficiently, using bitfields, at runtime.
This type declaration says that there are two flags used
for opening a file,
`create` and `directory`.
This means that when opening a file, we can either create it if it doesn't exist,
or error out.
Also, we can open it as a directory, or as a regular file.
We can have all four possible combinations of values
of these two boolean flags.

### Variants

```wit
variant new-timestamp {
    no-change,
    now,
    timestamp(datetime),
}
```

As we'll see, this type is used by the
`set-times-at` function that mutates the timestamp of a file.
A variant type is similar to an `enum` in Rust
or an algebraic datatype in Haskell or ML.
The closest C equivalent is a tagged union.

This declaration is saying that a `new-timestamp` can either be
the tag `no-change` with no argument,
the tag `now` with no argument,
or a `datetime` tagged with the tag `timestamp`.
The implementations of functions that use this type can
use pattern-matching to simultaneously check the tag
and extract the contents (if applicable).

### Putting it all together: resources

A resource describes an interface for objects.
This is not the same kind of "interface" as a WIT interface;
a WIT interface can contain many different `resource` declarations.
The declaration of the `descriptor` resource says that
a `descriptor` is an object that implements four methods:
`read`, `stat`, `set-times-at`, and `open-at`.
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

#### Getting metadata about files.

```wit
stat: func() -> result<descriptor-stat, error-code>;
```

The next method, `stat()`, gets metadata about a file.
It has no arguments (other than the implicit object argument
that all non-constructor methods have).
Like `read()`, it can fail, so it returns a `result` type.
The result can be either the `descriptor-stat` record we
already saw, or an error.

#### Setting timestamps on files

```wit
set-times-at: func(
    path: string,
    data-access-timestamp: new-timestamp,
) -> result<_, error-code>;
```

The next method, `set-times-at()`, sets the timestamp on a file.
Like the previous two methods, it can fail, so it returns a `result` type.
The result, if not an error, is `_`; this type represents the absence of a value,
like `void` in C/C++.

The function has two arguments: a file path, which is a string,
and a timestamp to set on the file at this path,
which is expressed using the `new-timestamp` variant type
that we already saw.
Looking at the definition of `new-timestamp`, we can see that
there are three different behaviors this method can implement:
either not changing the timestamp; setting it to the current time;
or setting it to a given time other than the current time.

#### Opening files

The `open-at()` method is a constructor, which we know because
it returns a `descriptor` when it doesn't fail (remember that
these methods are attached to the resource type `descriptor`):

```wit
open-at: func(
    path: string,
    open-flags: open-flags,
) -> result<descriptor, error-code>;
```

`open-at()` returns a new descriptor, given a path string and flags.

## Further reading

We've seen how using records, variants, enums, options, lists, tuples,
resources, and other rich types, WIT can encode a multitude of
ideas about how functions interrelate,
which are not available in the type system of core WebAssembly.

For more WIT examples, see the [tutorial](../tutorial.md) section.
The next section, [WIT Reference](./wit.md), covers WIT syntax
more thoroughly.
