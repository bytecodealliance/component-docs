# Async, Streams, and Futures

WASI P3 is built on three new Canonical ABI primitives in the Component Model: `async func`, `stream<T>`, and `future<T>`. Together, they let interfaces express asynchronous operations that compose across component boundaries.

## Why native async?

WASI P2 modeled asynchronous I/O through the `wasi:io` package, which exposed three resources:

- `pollable`: an opaque handle representing readiness
- `input-stream`: a byte source
- `output-stream`: a byte sink

These work fine when a component talks directly to its host. They break down when components are composed. Consider a three-layer chain:

```
A → B → Host
```

Component A makes a call into Component B that requires waiting on the Host. B calls the Host, which returns a `pollable` representing the eventual completion of the work. That `pollable` is a Component Model resource scoped to B's instance — A cannot hold it, poll it, or name it. B would have to actively check the `pollable` and forward the wake-up to A, but there is no Canonical ABI hook for "when this `pollable` becomes ready, run this code in B." In practice the wake-up gets dropped, and B has to actively poll its own `pollable` just to relay readiness back to A.

This is sometimes called the **sandwich problem**: WASI P2 could express async, but could not compose it across component boundaries.

The Component Model solves this by pushing async down into the Canonical ABI. The runtime owns scheduling and wake-up propagation, so async works correctly regardless of how many components sit between a producer and a consumer.

## The three primitives

### `async func`

A function declared as asynchronous in WIT:

```wit
handle: async func(request: request) -> result<response, error-code>;
```

Bindings generators emit language-native async constructs: `async fn` in Rust, `Promise` in JavaScript, coroutines in Python.

### `stream<T>`

A typed, asynchronous data channel. Unlike `input-stream` and `output-stream` in WASI P2, a `stream<T>` is a *value* that can be passed across component boundaries the same way a `u32` can.

```wit
read-via-stream: func() -> tuple<stream<u8>, future<result<_, error-code>>>;
```

### `future<T>`

A single-value async completion. Where WASI P2 used `pollable` resources, WASI P3 uses `future<T>` values.

```wit
write-via-stream: func(data: stream<u8>) -> future<result<_, error-code>>;
```

## Common patterns

### The stream-plus-future pattern

A recurring pattern in WASI P3 pairs a `stream<T>` with a `future` that signals completion or error:

```wit
read-via-stream: func() -> tuple<stream<u8>, future<result<_, error-code>>>;
```

The stream delivers data incrementally. Once the stream closes, the future resolves to indicate whether the operation completed successfully or encountered an error. This pattern appears in stdin, filesystem reads, TCP receives, and directory listings.

### The write-direction flip

In WASI P2, write operations gave you an `output-stream` that you wrote into. In WASI P3, the direction is reversed: you pass in a `stream<u8>` and receive a `future<result>` that resolves when the write completes. This applies to stdout, stderr, filesystem writes, and TCP sends.

```wit
// WASI P2: get a stream, write to it
get-stdout: func() -> output-stream;

// WASI P3: pass data in, get a completion future
write-via-stream: func(data: stream<u8>) -> future<result<_, error-code>>;
```

## Tooling support

`async func`, `stream<T>`, and `future<T>` are native Canonical ABI features in the Component Model, supported by:

- [Wasmtime](https://wasmtime.dev/) 43 and later, via `-Sp3 -W component-model-async=y` when running components.
- [`wit-bindgen`](https://github.com/bytecodealliance/wit-bindgen) with the `async` feature enabled, for Rust guest bindings.
- [jco](https://github.com/bytecodealliance/jco) for JavaScript environments, via the `preview3-shim` package.

For an end-to-end Rust example, see [Creating Runnable Components in Rust](../language-support/creating-runnable-components/rust.md). For the WIT-level details of how to declare these types in your own interfaces, see [WIT Reference](./wit.md).

> For a broader overview of what changed in WASI P3, including per-interface diffs, see [WASI P3](https://wasi.dev/releases/wasi-p3) on WASI.dev.
