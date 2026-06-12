# Canonical ABI

An ABI is an **application binary interface** - an agreement on how to pass data around in a binary format. ABIs are specifically concerned with data layout at the bits-and-bytes level. For example, an ABI might define how integers are represented (big-endian or little-endian?), how strings are represented (pointer to null-terminated character sequence or length-prefixed? UTF-8 or UTF-16 encoded?), and how composite types are represented (the offsets of each field from the start of the structure).

The Component Model defines a **canonical ABI** - an ABI to which all [components](../design/components.md) adhere. This guarantees that components can talk to each other without confusion, even if they are built in different languages. Internally, a C component might represent strings in a quite different way from a Rust component, but the canonical ABI provides a format for them to pass strings across the boundary between them.

> For a more formal definition of what the Canonical ABI is, take a look at the [Canonical ABI explainer](https://github.com/WebAssembly/component-model/blob/main/design/mvp/CanonicalABI.md).

## Native async extensions

WASI 0.3 added [`async func`, `stream<T>`, and `future<T>`](../design/async.md) as Canonical ABI primitives.
Supporting them required extending the ABI itself, since the existing rules assumed that every interface call returned synchronously.
This section sketches what those extensions are;
for the full specification, see the upstream [Concurrency explainer](https://github.com/WebAssembly/component-model/blob/main/design/mvp/Concurrency.md)
and [Canonical ABI explainer](https://github.com/WebAssembly/component-model/blob/main/design/mvp/CanonicalABI.md).

### Async function ABI

A WIT function declared `async` gets a *non-blocking core function signature* in addition to the existing (synchronous) one.
Both signatures stay available,
so a sync caller can invoke an async callee and an async caller can invoke a sync callee
without either side having to adopt the other's calling convention.

In the synchronous lowering (the mapping from a WIT-level signature to actual core Wasm function parameters and results),
parameters and return values are passed as flat core Wasm types when they fit,
or via linear-memory pointers (an *in-pointer* for parameters, an *out-pointer* for the result) otherwise.
The async lowering adds an `i32` status code as the actual function return value;
the logical return value lands at the caller-provided out-pointer once the call completes.
The low bits of the status code distinguish three cases:

- The call has not yet started reading its parameters.
- The call has read its parameters but not yet written its result.
- The call has returned, with both parameters and result memory consumed.

The runtime represents each in-flight async call as a *subtask*.
The caller can wait on a single subtask or on any of a *waitable set*;
the corresponding wait built-ins are listed below.
When the runtime signals completion the caller may resume and consume the result.

### Streams and futures across the boundary

`stream<T>` and `future<T>` are Canonical ABI *values* rather than resources.
At the wire level, each end is represented by an integer index into a per-component handle table (the same general mechanism resources use), with the value-vs-resource distinction showing up in the ownership rules below rather than in the encoding.
Each has two ends: a *readable* end and a *writable* end.
Ownership rules are direct:

- A component that creates a stream or future via the new `stream.new` or `future.new` built-ins receives both ends.
- A component that *receives* a stream or future from another component or the host gets unique ownership of the *readable* end.
- A component that *passes* a stream or future across the boundary transfers ownership of the readable end.

Writable ends are sticky: they stay with the component that created the stream or future and can't be transferred across boundaries.

Core Wasm code reads from streams via the `stream.read` built-in and writes via `stream.write`, passing a linear-memory buffer.
These built-ins are *completion-based*: a call either copies values into or out of the buffer immediately,
or returns a "blocked" sentinel indicating that the operation will continue concurrently.
Futures use the analogous `future.read` and `future.write`.

This is the same shape as OS-level completion-based I/O (`io_uring` on Linux, Overlapped I/O on Windows)
which is why bindings generators can map streams and futures onto a host language's existing concurrency primitives without extra plumbing.

### Other built-ins

Async support adds several other Canonical ABI built-ins:

- `task.return` for the export side of an async function to deliver its result.
- `task.cancel` for cancelling an in-flight subtask.
- *Waitable-set* primitives for waiting on or selecting among multiple in-flight subtasks (the mechanism used to consume async call results).

The full enumeration lives in the [Canonical ABI explainer](https://github.com/WebAssembly/component-model/blob/main/design/mvp/CanonicalABI.md);
the [Concurrency explainer](https://github.com/WebAssembly/component-model/blob/main/design/mvp/Concurrency.md) covers the design rationale and the broader concurrency model.
