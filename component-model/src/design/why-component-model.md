# Why the Component Model?

At a high level, the component model builds upon WebAssembly _core modules_
to enhance interoperability, both by enriching the type system
used for checking the safety of interactions between modules,
and by removing potentially error-prone ways for modules to interact.
To understand what the limitations of core modules are,
we start by defining them.

## WebAssembly core modules

A module is defined by the [WebAssembly Core Specification](https://webassembly.github.io/spec/core/).

WebAssembly programs can be written by hand,
but it's more likely that you will use a higher level programming language
such as Rust, C, Go, JavaScript, or Python to build WebAssembly programs.
Many existing toolchains currently produce a
[WebAssembly core module](https://webassembly.github.io/spec/core/syntax/modules.html)—a single
binary `.wasm` file.

A core module usually corresponds to a single binary `.wasm` file.
Here's what the `file` command outputs for a sample `.wasm` file:
```console
$ file adder.wasm
adder.wasm: WebAssembly (wasm) binary module version 0x1 (MVP)
```

A core module is a set of definitions.
Kinds of definitions include:
* _Functions_ define executable units of code
  (sequences of instructions along with declarations
  for argument names and types and return types).
* [_Linear memories_](https://webassembly.github.io/spec/core/syntax/modules.html#syntax-mem)
  define buffers of uninterpreted bytes that can be read from
  and written to by instructions.
* _Imports_ define the names of other modules
   that are required to be available to execute
   the functions in the module,
   along with type signatures for required functions
   in the imported module.
* _Exports_ define the names of functions within
  the module that should be accessible externally.
* And others; see [the Core Specification](https://webassembly.github.io/spec/core/syntax/modules.html)
  for the complete list.

Core modules can be run in the browser,
or via a separate runtime such as [Wasmtime](https://wasmtime.dev/)
or [WAMR](https://github.com/bytecodealliance/wasm-micro-runtime).

### Limitations of core modules

Core modules are limited in the computation they can perform and 
how they expose their functionality to the outside world.
In WebAssembly core modules, functions are restricted, essentially,
to using integer (`i32` or `i64`) or floating-point (`f32` or `f64`) types.
Only these types can be passed as arguments to functions,
and only these types can be returned from functions as results.
Compound types common in higher-level programming languages,
such as strings, lists, arrays, enums (enumerations), or structs (records),
have to be represented in terms of integers and floating-point numbers.

For example, for a function to accept a string, the string argument
might be represented as two separate arguments:
an integer offset into a memory
and an integer representing the length of the string.
Recall that a (linear) memory is an uninitialized region of bytes
declared within a module.

In pseudocode, a type signature for a string-manipulating function
might look like:

```
remove-duplicates: func(offset: i32, length: i32) -> [i32, i32]
```

supposing that `remove-duplicates` is a function
to create a new string consisting of the unique characters
in its argument.
The return type is a list of two 32-bit integers.
The first integer is an offset into one of the linear memories
declared by the module—where the newly allocated string starts—and
the second integer is the length of the string.
After calling the function,
the caller has to reach into the appropriate linear memory
and read the output string, using the returned offset and length.

For this to work, the module defining the `remove-duplicates` function
would also need to include
an export declaration that exports a memory to be used
for the argument and result strings. Pseudocode:

```
export "string_mem" (mem 1)
```

And, the module using the `remove-duplicates` function
would need to import this memory. Pseudocode:

```
import "strings" "string_mem"
```

(This pseudocode is still simplified, since the importer
also needs to declare the size of the memory being
imported.)

Note that there is nothing in the type system to prevent
the returned length from being confused with the returned offset,
since both are integers.
Also, the name of the memory used for the input and output strings
must be established by convention,
and there is also nothing in the type system to stop client code
from indexing into a different memory
(as long as the sum of the offset and length is within bounds).

We would prefer to write a pseudocode type signature like this:

```
remove-duplicates: func(s: string) -> string
```

and dispense with the memory exports and imports altogether.

The complexity doesn't stop there!
Data representations are frequently specific to each programming language.
For example, a string in C is represented entirely differently
from a string in Rust or in JavaScript.
Moreover, to make this approach work, modules must import and export memories,
which can be error-prone, as different languages
make different assumptions about memory layout.

For WebAssembly modules written in different languages to interoperate smoothly,
there needs to be an agreed-upon way to expose these richer types across module boundaries.

## Components

Components solve the two problems that we've seen so far:
the limited type system of core module functions,
and cross-language interoperability.
Conceptually, a component is a WebAssembly binary
(which may or may not contain modules)
that is restricted to interact
only through the modules' imported and exported functions.
Components use a different binary format.
Compared to core modules, components also use a richer
mechanism by default for expressing the types of functions: _interfaces_.

### Interfaces

Interfaces are expressed in a separate language called [WebAssembly Interface Types (WIT)](./wit.md).
[Interfaces](./wit.md#interfaces) contain definitions of _types_
and type signatures for [_functions_](./wit.md#functions).
The bit-level representations of types are specified by
the [Canonical ABI (Application Binary Interface)](./../advanced/canonical-abi.md).

### Interoperability

WebAssembly core modules are already portable across different architectures
and operating systems;
components retain these benefits and, using the Component Model ABI,
add portability across different programming languages.
A component implemented in Go can communicate directly and safely
with a C or Rust component, by relying on the shared conventions of the Component Model ABI.
Writing a component doesn't even require knowledge
of which language its dependent components are implemented in,
only the component interface expressed in WIT.
Additionally, components can be [composed](../composing-and-distributing.md) into larger graphs,
with one component's exports satisfying another's imports.

### Benefits of the component model

Putting all of the pieces together:
the component model is a way of writing WebAssembly modules
that interact with each other only through exports and imports of functions
whose types are expressed using WIT.

Building upon Wasm's strong [sandboxing](https://webassembly.org/docs/security/),
the component model has further benefits.
Richer type signatures express richer semantic properties
than type signatures made up only of integers and floats.
Rich types make it possible to statically analyse
and reason about a component's behaviour.
Simply by examining the surface of the component—the types
of its imports and exports—properties can be
enforced and guaranteed.
The relationships within a graph of components can be analysed:
for example, to verify that a component containing business logic
has no access to a component containing personally identifiable information.

Moreover, a component interacts with a runtime or other components
_only_ by calling its imports and having its exports called.
Specifically, unlike core modules, a component may not export a memory
and thus it cannot indirectly communicate to others
by writing to its memory and having others read from that memory.
This not only reinforces sandboxing, but enables interoperation
between languages that make different assumptions about memory:
for example, allowing a component that relies garbage-collected memory
to interoperate with one that uses conventional linear memory.

## Using components

Now that you have a better idea about how the component model can help you, take a look at [how to build components](../language-support.md) in your favorite language!

## Further reading

For more background on why the component model was created,
take a look at the specification's [goals](https://github.com/WebAssembly/component-model/blob/main/design/high-level/Goals.md),
[use cases](https://github.com/WebAssembly/component-model/blob/main/design/high-level/UseCases.md)
and [design choices](https://github.com/WebAssembly/component-model/blob/main/design/high-level/Choices.md).
