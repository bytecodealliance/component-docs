# Why the Component Model?

If you've tried out WebAssembly, you'll be familiar with the concept of a _module_. Roughly speaking, a module corresponds to a single `.wasm` file, with functions, memory, imports and exports, and so on. These "core" modules can run in the browser, or via a separate runtime such as Wasmtime or WAMR. A module is defined by the [WebAssembly Core Specification](https://webassembly.github.io/spec/core/), and if you compile a program written in Rust, C, Go or whatever for the browser, then a core module is what you'll get.

Core modules are, however, limited to describing themselves in terms of a small number of core WebAssembly types such as integers and floating-point numbers. Just as in native assembly code, richer types, such as strings or records (structs), have to be represented in terms of integers and floating point numbers, for example by the use of pointers and offsets. And just as in native code, those representations are not interchangeable. A string in C might be represented entirely differently from a string in Rust, or a string in JavaScript.

For Wasm modules to interoperate, therefore, there needs to be an agreed-upon way for defining those richer types, and an agreed-upon way of expressing them at module boundaries.

In the component model, these type definitions are written in a language called WIT (Wasm Interface Type), and the way they translate into bits and bytes is called the Canonical ABI (Application Binary Interface).

Such an agreement adds a new dimension to Wasm portability. Not only are components portable across architectures and operating systems, but they are now portable across languages. A Go component can communicate directly and safely with a C or Rust component. It need not even know which language another component was written in - it needs only the component interface, expressed in WIT. Additionally, components can be linked into larger graphs, with one component satisfying another's dependencies, and deployed as units.

Combined with Wasm's strong sandboxing, this opens the door to yet further benefits.  By expressing higher-level semantics than integers and floats, it becomes possible to statically analyse and reason about a component's behaviour - to enforce and guarantee properties just by looking at the surface of the component. The relationships within a graph of components can be analysed, for example to verify that a component containing business logic has no access to a component containing personally identifiable information.

Moreover, components interact _only_ through the Canonical ABI. Specifically, unlike core modules, components may not export Wasm memory. This not only reinforces sandboxing, but enables interoperation between languages that make different assumptions about memory - for example, allowing a component that relies on Wasm GC (garbage collected) memory to collaborate with one that uses conventional linear memory.
