# What is a Component?

If you've tried out WebAssembly, you'll be familiar with the concept of a _module_. Roughly speaking, a module corresponds to a single `.wasm` file, with functions, types, memory, imports and exports, and so on. These modules can run in the browser, or via a separate runtime such as Wasmtime or Wamr. A module is what you get by default when you compile a program written in Rust, C, Go or whatever to WebAssembly.

Modules are, however, limited to describing themselves in terms of core WebAssembly types - integers and floating-point numbers. Just as in native assembly code, richer types, such as strings or records (structs), have to be represented in terms of integers and floating point numbers, for example by the use of pointers and offsets. And just as in native code, those representations are not interchangeable. A string in C might be represented entirely differently from a string in Rust, or a string in JavaScript.

For Wasm modules to interoperate, therefore, there needs to be an agreed-upon way of defining those richer types, and an agreed-upon way of expressing them at module boundaries.

In the component model, these definitions are written in a language called WIT (Wasm Interface Type), and the way they translate into bits and bytes is called the Canonical ABI (Application Binary Interface).

Such an agreement adds a new dimension to Wasm portability. Not only are modules portable across architectures and operating systems, but they are now portable across languages. A Go module can communicate directly and safely with a C or Rust module. It need not even know which language another module was written in - it needs only the module interface, expressed in WIT. Modules can even be linked into larger graphs, with one module satisfying another's dependencies, and deployed as units.

Combined with Wasm's strong sandboxing, this opens the door to yet further benefits.  By expressing higher-level semantics than integers and floats, it becomes possible to statically analyse and reason about a module's behaviour - to enforce and guarantee properties just by looking at the surface of the module. The relationships between a graph of modules can be analysed, for example to verify that a module containing business logic has no access to a module containing personally identifiable information.

This guide doesn't try to formally specify what a component _is_. Physically, components are Wasm modules, such as `.wasm` files, with a specific internal format. Logically, components are containers for modules - or other components - which express their interfaces and dependencies via WIT and the Canonical ABI. Conceptually, components are self-describing units of code.
