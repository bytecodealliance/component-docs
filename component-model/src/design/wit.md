# An Overview of WIT

We've explained how worlds and interfaces
define how components relate to each other.
To define a new component, you will need to define worlds and interfaces
by writing code in the Wasm Interface Type (WIT) language.
WIT also serves as documentation for existing components
that you may wish to use.

The WIT (Wasm Interface Type) language is used to define Component Model [interfaces](#interfaces) and [worlds](#worlds).
WIT isn't a general-purpose programming language and doesn't define behaviour;
it defines only _contracts_ between components.
This topic provides an overview of key elements of the WIT language.
The official WIT specification and history can be found in the [`WebAssembly/component-model` repository](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md).

- [An Overview of WIT](#an-overview-of-wit)
  - [Structure of a WIT file](#structure-of-a-wit-file)
  - [Comments](#comments)
  - [Documentation](#documentation)
  - [Identifiers](#identifiers)
  - [Built-in types](#built-in-types)
    - [Primitive types](#primitive-types)
    - [Lists](#lists)
    - [Options](#options)
    - [Results](#results)
    - [Tuples](#tuples)
  - [User-defined types](#user-defined-types)
    - [Records](#records)
    - [Variants](#variants)
    - [Enums](#enums)
    - [Resources](#resources)
    - [Flags](#flags)
    - [Type aliases](#type-aliases)
  - [Functions](#functions)
  - [Interfaces](#interfaces)
    - [Using definitions from elsewhere](#using-definitions-from-elsewhere)
  - [Worlds](#worlds)
    - [Interfaces from other packages](#interfaces-from-other-packages)
    - [Inline interfaces](#inline-interfaces)
    - [Including other worlds](#including-other-worlds)
  - [Packages](#packages)

## Structure of a WIT file

A WIT file contains one or more **interfaces** or **worlds**. An interface or world can define **types** and/or **functions**.

> Types and functions can't be defined outside of interfaces or worlds.

A file may optionally start with a **package** declaration.

## Comments

WIT comment syntax is similar to the one used by the C++ family of languages:

* Everything from `//` to end of line is a comment.
* Any text enclosed in `/*` ... `*/` is a comment.
  * Unlike the C++ family, block comments _can_ be nested, e.g. `/* blah /* rabbit */ rhubarb */`.

## Documentation

WIT defines special comment formats for documentation:

* Everything from `///` to end of line is documentation for the following item.
* Any text enclosed in `/**` ... `*/` is documentation for the following item.

For example:

```wit
/// Prints "hello".
print-hello: func();

/**
Prints "hello".
*/
print-hello: func();
```

## Identifiers

_Identifiers_ are names for variables, functions, types, interfaces, and worlds.
WIT identifiers have a slightly different set of rules
from what you might be familiar with in languages like C, Rust, and Java.
These rules apply to all names, except for packages.
Package identifiers are a little more complex and will be covered in the [Packages section](#packages).

* Identifiers are restricted to ASCII `kebab-case`: sequences of words, separated by single hyphens.
  * Double hyphens (`--`) are not allowed.
  * Hyphens aren't allowed at the beginning or end of the sequence, only between words.
* An identifier may be preceded by a single `%` sign.
  * This is _required_ if the identifier would otherwise be a WIT keyword.
    For example, `interface` is **not** a legal identifier, but `%interface` is legal.
* Each word in the sequence must begin with an ASCII letter, and may contain only ASCII letters and digits.
  * A word cannot begin with a digit.
  * A word cannot contain a non-ASCII Unicode character.
  * A word cannot contain punctuation, underscores, etc.
* Each word must be either all `lowercase` or all `UPPERCASE`.
  * Different words in the identifier may have different cases. For example, `WIT-demo` is allowed.
* An identifier cannot be a WIT keyword such as `interface` (unless preceded by a `%` sign).

## Built-in types

The types in this section are defined by the WIT language itself.

### Primitive types

WIT defines the following primitive types:

| Identifier                 | Description |
|----------------------------|-------------|
| `bool`                     | Boolean value `true` or `false`. |
| `s8`, `s16`, `s32`, `s64`  | Signed integers of the appropriate width. For example, `s32` is a signed 32-bit integer. |
| `u8`, `u16`, `u32`, `u64`  | Unsigned integers of the appropriate width. For example, `u32` is an unsigned 32-bit integer. |
| `f32`, `f64`               | Floating-point numbers of the appropriate width. For example, `f64` is a 64-bit (double precision) floating-point number. See the note on `NaN`s below. |
| `char`                     | Unicode character. (Specifically, a [Unicode scalar value](https://unicode.org/glossary/#unicode_scalar_value).) |
| `string`                   | A Unicode string: that is, a finite sequence of characters. |

> The `f32` and `f64` types support the usual set of IEEE 754 single and double-precision values,
> except that they logically only have a single `nan` value.
> The exact bit-level representation of an IEEE 754 `NaN`
> is not guaranteed to be preserved when values pass through WIT interfaces
> as the singular WIT `nan` value.

### Lists

`list<T>` for any type `T` denotes an ordered sequence of values of type `T`.
`T` can be any type, built-in or user-defined:

```wit
list<u8>       // byte buffer
list<customer> // a list of customers
```

This is similar to Rust `Vec`, or Java `List`.

### Options

`option<T>` for any type `T` may contain a value of type `T`, or may contain no value.
`T` can be any type, built-in or user-defined.
For example, a lookup function might return an option in order to allow
for the possibility that the lookup key wasn't found:

```wit
option<customer>
```

This is similar to Rust `Option`, C++ `std::optional`, or Haskell `Maybe`.

> This is a special case of a [variant](#variants) type.
> WIT defines it so that there is a common way of expressing it,
> so that you don't need to create a variant type for every value type,
> and to enable it to be mapped idiomatically into languages with option types.

### Results

`result<T, E>` for any types `T` and `E`
may contain a value of type `T` _or_ a value of type `E`
(but not both).
This is typically used for "value or error" situations:
for example, a HTTP request function might return a result,
with the success case (the `T` type) representing a HTTP response,
and the error case (the `E` type) representing the various kinds of error that might occur:

```wit
result<http-response, http-error>
```

This is similar to Rust `Result`, or Haskell `Either`.

> This is a special case of a [variant](#variants) type.
> WIT defines the `result` type so that there is a common way of expressing this behavior,
> so that developers don't need to create variant types for every combination of value and error types,
> and to enable it to be mapped idiomatically into languages with result or "either" types.

Sometimes there is no data associated with one or both of the cases.
For example, a `print` function could return an error code if it fails,
but has nothing to return if it succeeds.
In this case, you can omit the corresponding type as follows:

```wit
result<u32>     // no data associated with the error case
result<_, u32>  // no data associated with the success case
result          // no data associated with either case
```

The underscore `_` stands in "no data" and is generally represented as the unit type in a target language (e.g. `()` in rust, `null` in Javsacript).

### Tuples

A `tuple` type is an ordered _fixed-length_ sequence of values of specified types.
It is similar to a [_record_](#records), except that the fields are identified by indices
instead of by names.

```wit
tuple<u64, string>      // An integer and a string
tuple<u64, string, u64> // An integer, then a string, then an integer
```

This is similar to tuples in Rust or OCaml.

## User-defined types

You can define your own types within an `interface` or `world`.
WIT offers several ways of defining new types.

### Records

A `record` type declares a set of named fields, each of the form `name: type`,
separated by commas.
A record instance contains a value for every field.
Field types can be built-in or user-defined.
The syntax is as follows:

```wit
record customer {
    id: u64,
    name: string,
    picture: option<list<u8>>,
    account-manager: employee,
}
```

Records are similar to C or Rust `struct`s.

> User-defined records can't be generic (that is, parameterised by type).
> Only built-in types can be generic.

### Variants

A `variant` type represents data whose structure varies.
The declaration defines a list of cases;
each case has a name and, optionally,
a type of data associated with that case.
An instance of a variant type matches exactly one case.
Cases are separated by commas.
The syntax is as follows:

```wit
variant allowed-destinations {
    none,
    any,
    restricted(list<address>),
}
```

This can be read as "an allowed destination is either none, any,
or restricted to a particular list of addresses".

Variants are similar to Rust `enum`s or OCaml discriminated unions.
The closest C equivalent is a tagged union, but variants in WIT
both take care of the "tag" (the case)
and enforce the correct data shape for each tag.

> User-defined variants can't be generic (that is, parameterised by type).
> Only built-in types can be generic.

### Enums

An `enum` type is a variant type where none of the cases have associated data:

```wit
enum color {
    hot-pink,
    lime-green,
    navy-blue,
}
```

This can provide a simpler representation in languages without discriminated unions.
For example, a WIT `enum` can translate directly to a C/C++ `enum`.

### Resources

A resource is a handle to some entity that exists outside of the component.
Resources describe entities that can't or shouldn't be copied; entities that should
be passed by reference rather than by value.
Components can pass resources to each other via a handle.
They can pass ownership of resources, or pass non-owned references to resources.

> If you're not familiar with the concepts of borrowing and ownership
> for pointers, see [the Rust documentation](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html).

Unlike other WIT types, which are simply plain data,
resources only expose behavior through methods.
Resources can be thought of as _objects that implement_ an interface.
("Interface" here is used in the object-oriented programming sense,
not in the sense of a WIT interface.)

For example, we could model a blob (binary large object) as a resource.
The following WIT defines the `blob` resource type,
which contains a constructor, two methods, and a static function:

```wit
resource blob {
    constructor(init: list<u8>);
    write: func(bytes: list<u8>);
    read: func(n: u32) -> list<u8>;
    merge: static func(lhs: blob, rhs: blob) -> blob;
}
```

As shown in the `blob` example, a resource can contain:

- _methods_: functions that implicitly take a `self` (AKA `this`) parameter that is a handle.
  (Some programming languages use the `this` keyword instead of `self`.)
  `read` and `write` are methods.
- _static functions_: functions which do not have an implicit `self` parameter
  but are meant to be nested in the scope of the resource type,
  similarly to static functions in C++ or Java.
  `merge` is a static function.
- at most one _constructor_: a function that is syntactic sugar for
  a function returning a handle of the containing resource type.
  The constructor is declared with `constructor`.

A method can be rewritten to be a function with a borrowed `self` parameter,
and a constructor can be rewritten to a function that returns a value
owned by the caller.
For example, the `blob` resource [above](#resources) could be approximated as:

```wit
resource blob;
blob-constructor: func(bytes: list<u8>) -> blob;
blob-write: func(self: borrow<blob>, bytes: list<u8>);
blob-read: func(self: borrow<blob>, n: u32) -> list<u8>;
blob-merge: static func(lhs: blob, rhs: blob) -> blob;
```

When a `resource` type name is wrapped with `borrow<...>`,
it stands for a "borrowed" resource.
A borrowed resource represents a temporary loan of a resource
from the caller to the callee for the duration of the call.
In contrast, when the owner of an owned resource drops that resource,
the resource is destroyed.
(Dropping the resource means either explicitly dropping it
if the underlying programming language supports that,
or returning without transferring ownership to another function.)

> More precisely, these are borrowed or owned `handles` of the resource.
> Learn more about `handles` in the [upstream component model specification](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md#handles).

### Flags

A `flags` type is a set of named booleans.

```wit
flags allowed-methods {
    get,
    post,
    put,
    delete,
}
```

> A `flags` type is logically equivalent to a record type where each field is of type `bool`,
> but it is represented more efficiently (as a bitfield) at the binary level.

### Type aliases

You can define a new type alias using `type ... = ...`.
Type aliases are useful for giving shorter or more meaningful names to types:

```wit
type buffer = list<u8>;
type http-result = result<http-response, http-error>;
```

## Functions

A function is defined by a name and a function type.
As with record fields, the name is separated from the type by a colon:

```wit
do-nothing: func();
```

The function type is the keyword `func`,
followed by a parenthesised, comma-separated list of parameters (names and types).
If the function returns a value, this is expressed as an arrow symbol (`->`) followed by the return type:

```wit
// This function does not return a value
print: func(message: string);

// These functions return values
add: func(a: u64, b: u64) -> u64;
lookup: func(store: kv-store, key: string) -> option<string>;
```

To express a function that returns multiple values,
you can use any compound type ([tuples](#tuple), [record](#record), etc).

```wit
get-customers-paged: func(cont: continuation-token) -> tuple<list<customer>, continuation-token>;
```

A function can be declared inside an [interface](#interfaces),
or can be declared as an import or export in a [world](#worlds).

## Interfaces

An interface is a named set of types and functions,
enclosed in braces and introduced with the `interface` keyword:

```wit
interface canvas {
    type canvas-id = u64;

    record point {
        x: u32,
        y: u32,
    }

    draw-line: func(canvas: canvas-id, from: point, to: point);
}
```

Notice that types and functions in an interface are _not_ comma-separated.

### Using definitions from elsewhere

An interface can reuse types declared in another interface via a `use` directive.
The `use` directive must give the interface where the types are declared,
then a dot, then a braced list of the types to be reused.
The interface can then refer to the types named in the `use`.

```wit
interface types {
    type dimension = u32;
    record point {
        x: dimension,
        y: dimension,
    }
}

interface canvas {
    use types.{dimension, point};
    type canvas-id = u64;
    draw-line: func(canvas: canvas-id, from: point, to: point, thickness: dimension);
}
```

The `canvas` interface uses the types `dimension` and `point` declared in the `types` interface.

> Even if you are only using one type, it must still be enclosed in braces.
> For example, `use types.{dimension}` is legal but `use types.dimension` is not.

This works across files as long as the files are in the same package (effectively, in the same directory).
For information about using definitions from other packages, see [the specification](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md#interfaces-worlds-and-use).

## Worlds

Roughly, a world describes the contract of a component.
A world describes a set of imports and exports,
enclosed in braces and introduced with the `world` keyword.
Imports and exports may be interfaces or specific functions.
Exports describe the interfaces or functions provided by a component.
Imports describe the interfaces or functions that a component depends on.

```wit
interface printer {
    print: func(text: string);
}

interface error-reporter {
    report-error: func(error-message: string);
}

world multi-function-device {
    // The component implements the `printer` interface
    export printer;

    // The component implements the `scan` function
    export scan: func() -> list<u8>;

    // The component needs to be supplied with an `error-reporter`
    import error-reporter;
}
```

This code defines a world called `multi-function device`,
with two exports, a `printer` interface and a `scan` function.
The exported `printer` interface is defined in the same file.
The imported `error-reporter` interface is also defined in the same file.
From looking at the `error-reporter` interface,
you can see that When a world imports an interface,
the full interface with types and function declarations
needs to be provided,
not just the name of the interface.

### Interfaces from other packages

To import and export interfaces defined in other packages,
you can use `package/name` syntax:

```wit
world http-proxy {
    export wasi:http/incoming-handler;
    import wasi:http/outgoing-handler;
}
```

> As this example shows, import and export apply at the interface level, not the package level.
> You can import one interface defined in a package,
> while exporting another interface defined in the same package.
> A package groups definitions together; it doesn't describe a coherent set of behaviours.

WIT does not define how packages are resolved;
different tools may resolve them in different ways.

### Inline interfaces

Interfaces can be declared inline in a world:

```wit
world toy {
    export example: interface {
        do-nothing: func();
    }
}
```

### Including other worlds

You can `include` another world.
This causes your world to export all that world's exports,
and import all that world's imports.

```wit
world glow-in-the-dark-multi-function-device {
    // The component provides all the same exports, and depends on
    // all the same imports, as a `multi-function-device`...
    include multi-function-device;

    // ...but also exports a function to make it glow in the dark
    export glow: func(brightness: u8);
}
```

As with `use` directives, you can `include` worlds from other packages.

## Packages

A package is a set of interfaces and worlds,
potentially defined across multiple files.
To declare a package, use the `package` directive to specify the package ID.
A package ID must include a namespace and name, separated by a colon,
and may optionally include a [semver](https://semver.org/)-compliant version number:

```wit
package documentation:example;
package documentation:example@1.0.1;
```

All files must have the `.wit` extension and must be in the same directory.
If a package spans multiple files,
only one file needs to contain a package declaration,
but if multiple files contain package declarations,
the package IDs must all match each other.
 For example, the following `documentation:http` package is spread across four files:

```wit
// types.wit
interface types {
    record request { /* ... */ }
    record response { /* ... */ }
}

// incoming.wit
interface incoming-handler {
    use types.{request, response};
    // ...
}

// outgoing.wit
interface outgoing-handler {
    use types.{request, response};
    // ...
}

// http.wit
package documentation:http@1.0.0;

world proxy {
    export incoming-handler;
    import outgoing-handler;
}
```

This package defines request and response types in `types.wit`,
an incoming handler interface in `incoming wit`,
an outgoing handler interface in `outgoing.wit`,
and declares the package and defines a world that uses these interfaces
in `http.wit`.

> For a more formal definition of the WIT language, take a look at the [WIT specification](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md).
