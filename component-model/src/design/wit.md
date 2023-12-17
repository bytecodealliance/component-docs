# An Overview of WIT

The WIT (Wasm Interface Type) language is used to define Component Model interfaces and worlds. WIT isn't a general-purpose coding language and doesn't define behaviour; it defines only _contracts_ between components. This topic provides an overview of key elements of the WIT language.

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

WIT identifiers have a slightly different set of rules from what you might be familiar with from, say, C, Rust, or Java.  These rules apply to all names - types, functions, interfaces, and worlds. (Package identifiers are a little more complex and will be covered in the [Packages section](#packages).)

* Identifiers are restricted to ASCII `kebab-case` - sequences of words, separated by single hyphens.
  * Double hyphens (`--`) are not allowed.
  * Hyphens aren't allowed at the beginning or end of the sequence, only between words.
* An identifier may be preceded by a single `%` sign.
  * This is _required_ if the identifier would otherwise be a WIT keyword. For example, `interface` is **not** a legal identifier, but `%interface` is legal.
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
| `bool`                     | Boolean value - true or false. |
| `s8`, `s16`, `s32`, `s64`  | Signed integers of the appropriate width. For example, `s32` is a 32-bit integer. |
| `u8`, `u16`, `u32`, `u64`  | Unsigned integers of the appropriate width. For example, `u32` is a 32-bit integer. |
| `float32`, `float64`       | Floating-point numbers of the appropriate width. For example, `float64` is a 64-bit (double precision) floating-point number. |
| `char`                     | Unicode character. (Specifically, a [Unicode scalar value](https://unicode.org/glossary/#unicode_scalar_value).) |
| `string`                   | A Unicode string - that is, a finite sequence of characters. |

### Lists

`list<T>` for any type T denotes an ordered sequence of values of type T.  T can be any type, built-in or user-defined:

```wit
list<u8>        // byte buffer
list<customer>  // a list of customers
```

This is similar to Rust `Vec`, or Java `List`.

### Options

`option<T>` for any type T may contain a value of type T, or may contain no value.  T can be any type, built-in or user-defined.  For example, a lookup function might return an option, allowing for the possibility that the lookup key wasn't found:

```wit
option<customer>
```

This is similar to Rust `Option`, C++ `std::optional`, or Haskell `Maybe`.

> This is a special case of a [variant](#variants) type.  WIT defines it so that there is a common way of expressing it, so that you don't need to create a variant type for every value type, and to enable it to be mapped idiomatically into languages with option types.

### Results

`result<T, E>` for any types T and E may contain a value of type T _or_ a value of type E (but not both). This is typically used for "value or error" situations; for example, a HTTP request function might return a result, with the success case (the T type) representing a HTTP response, and the error case (the E type) representing the various kinds of error that might occur:

```wit
result<http-response, http-error>
```

This is similar to Rust `Result`, or Haskell `Either`.

> This is a special case of a [variant](#variants) type.  WIT defines it so that there is a common way of expressing it, so that you don't need to create a variant type for every combination of value and error types, and to enable it to be mapped idiomatically into languages with result or "either" types.

Sometimes there is no data associated with one or both of the cases. For example, a `print` function could return an error code if it fails, but has nothing to return if it succeeds. In this case, you can omit the corresponding type as follows:

```wit
result<u32>     // no data associated with the error case
result<_, u32>  // no data associated with the success case
result          // no data associated with either case
```

### Tuples

A tuple type is an ordered _fixed length_ sequence of values of specified types. It is similar to a [_record_](#records), except that the fields are identified by their order instead of by names.

```wit
tuple<u64, string>  // An integer and a string
tuple<u64, string, u64>  // An integer, then a string, then an integer
```

This is similar to tuples in Rust or OCaml.

## User-defined types

You can define your own types within an `interface` or `world`. WIT offers several ways of defining new types.

### Records

A record type declares a set of named fields, each of the form `name: type`, separated by commas. A record instance contains a value for every field. Field types can be built-in or user-defined. The syntax is as follows:

```wit
record customer {
    id: u64,
    name: string,
    picture: option<list<u8>>,
    account-manager: employee,
}
```

Records are similar to C or Rust `struct`s.

> User-defined records can't be generic (that is, parameterised by type). Only built-in types can be generic.

### Variants

A variant type declares one or more cases. Each case has a name and, optionally, a type of data associated with that case. A variant instance contains exactly one case. Cases are separated by commas. The syntax is as follows:

```wit
variant allowed-destinations {
    none,
    any,
    restricted(list<address>),
}
```

Variants are similar to Rust `enum`s or OCaml discriminated unions. The closest C equivalent is a tagged union, but WIT both takes care of the "tag" (the case) and enforces the correct data shape for each tag.

> User-defined variants can't be generic (that is, parameterised by type). Only built-in types can be generic.

### Enums

An enum type is a variant type where none of the cases have associated data:

```wit
enum color {
    hot-pink,
    lime-green,
    navy-blue,
}
```

This can provide a simpler representation in languages without discriminated unions. For example, a WIT enum can translate directly to a C++ `enum`.

### Flags

A flags type is a set of named booleans.  In an instance of the type, each flag will be either true or false.

```wit
flags allowed-methods {
    get,
    post,
    put,
    delete,
}
```

> A flags type is logically equivalent to a record type where each field is of type `bool`, but it is represented more efficiently (as a bitfield) at the binary level.

### Type aliases

You can define a new named type using `type ... = ...`. This can be useful for giving shorter or more meaningful names to types:

```wit
type buffer = list<u8>;
type http-result = result<http-response, http-error>;
```

## Functions

A function is defined by a name and a function type. Like in record fields, the name is separated from the type by a colon:

```wit
do-nothing: func();
```

The function type is the word `func`, followed by a parenthesised, comma-separated list of parameters (names and types). If the function returns a value, this is expressed as an arrow symbol (`->`) followed by the return type:

```wit
// This function does not return a value
print: func(message: string);

// These functions return values
add: func(a: u64, b: u64) -> u64;
lookup: func(store: kv-store, key: string) -> option<string>;
```

A function can have multiple return values. In this case the return values must be named, similar to the parameter list. All return values must be populated (in the same way as tuple or record fields).

```wit
get-customers-paged: func(cont: continuation-token) -> (customers: list<customer>, cont: continuation-token);
```

A function can be declared as part of an [interface](#interfaces), or can be declared as an import or export in a [world](#worlds).

## Interfaces

An interface is a named set of types and functions, enclosed in braces and introduced with the `interface` keyword:

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

Notice that items in an interface are _not_ comma-separated.

### Using definitions from elsewhere

An interface can reuse types declared in another interface via a `use` directive. The `use` directive must give the interface where the types are declared, then a dot, then a braced list of the types to be reused. The interface can then refer to the types named in the `use`.

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

> Even if you are only using one type, it must still be enclosed in braces. For example, `use types.{dimension}` is legal but `use types.dimension` is not.

This works across files as long as the files are in the same package (effectively, in the same directory). For information about using definitions from other packages, see [the specification](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md#interfaces-worlds-and-use).

## Worlds

A world describes a set of imports and exports, enclosed in braces and introduced with the `world` keyword. Roughly, a world describes the contract of a component. Exports are provided by the component, and define what consumers of the component may call; imports are things the component may call. The imports and exports may be interfaces or individual functions.

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

### Interfaces from other packages

You can import and export interfaces defined in other packages. This can be done using `package/name` syntax:

```wit
world http-proxy {
    export wasi:http/incoming-handler;
    import wasi:http/outgoing-handler;
}
```

> As this example shows, import and export apply at the interface level, not the package level. You can import one interface defined in a package, while exporting another interface defined in the same package. Packages group definitions; they don't represent behaviour.

WIT does not define how packages are resolved - different tools may resolve them in different ways.

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

You can `include` another world. This causes your world to export all that world's exports, and import all that world's imports.

```wit
world glow-in-the-dark-multi-function-device {
    // The component provides all the same exports, and depends on all the same imports, as a `multi-function-device`...
    include multi-function-device;
    // ...but also exports a function to make it glow in the dark
    export glow: func(brightness: u8);
}
```

As with `use` directives, you can `include` worlds from other packages.

## Packages

A package is a set of interfaces and worlds, potentially defined across multiple files. To declare a package, use the `package` directive to specify the package ID. This must include a namespace and name, separated by a colon, and may optionally include a semver-compliant version:

```wit
package documentation:example;
package documentation:example@1.0.1;
```

If a package spans multiple files, only one file needs to contain a package declaration (but if multiple files contain declarations then they must all be the same). All files must have the `.wit` extension and must be in the same directory. For example, the following `documentation:http` package is spread across four files:

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

> ⓘ For a more formal definition of the WIT language, take a look at the [WIT specification](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md).
