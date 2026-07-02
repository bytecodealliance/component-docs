# Native Async with WASI 0.3

WASI 0.3 adds new Canonical ABI primitives to the Component Model that enable async functionality. Components that target WASI 0.3 can use the new features in their WIT files:
* `async func` 
* `stream<T>`
* `future<T>`

These new types let interfaces express asynchronous operations that compose across component boundaries.

For migration mechanics (e.g., how a WASI 0.2 component maps onto these primitives) see [Migrating from WASI 0.2 to WASI 0.3](./migrating-to-p3.md). 

For the a closer look at WASI 0.3 release, including a full per-interface diff, see [WASI 0.3](https://wasi.dev/releases/wasi-p3) on WASI.dev. 

This page focuses on the Component Model concepts themselves.

## The async problem that WASI 0.3 solves

The Component Model's Canonical ABI defines how typed values cross component boundaries. Until WASI 0.3, that vocabulary had no notion of suspension or asynchronous completion; every interface call returned synchronously, and asynchronous I/O was modeled with resources (`pollable` for readiness, `input-stream` and `output-stream` for byte channels) scoped to whichever component obtained them.

That arrangement holds up for two-party interactions, but it falters once components are composed in a chain. If a component awaits work that another component delegates further, the readiness signal has to travel back up the chain. When readiness is expressed as a resource scoped to a single component, the intermediate component is stuck running an event loop purely to forward the wake-up to its caller; the runtime cannot help, because the resource doesn't live in a place the runtime can reach across. This is sometimes called the **sandwich problem**: an async vocabulary that describes a single hop just fine but cannot propagate readiness past one.

Native async primitives help close this expressivity gap. With updated Component ABI mechanics that enable `async func`, `stream<T>`, and `future<T>` available at the WIT level, scheduling and wake-up propagation become the runtime's job rather than any individual component's. 

Components can pass futures and streams along without keeping their own event loops running to relay readiness, as was necessary with WASI 0.2.

## Async functions, Streams, and Futures

### Async Functions (`async func`)

A WIT function declared `async` tells the runtime that the call may suspend before producing its result. The Canonical ABI handles the suspension and resumption; the guest doesn't see a `pollable`, and the host doesn't see a polling loop.

```wit
handle: async func(request: request) -> result<response, error-code>;
```

Code generated from the WIT picks up each language's natural async idiom: `async fn` in Rust, a `Promise`-returning function in JavaScript, a coroutine in Python.

### `stream<T>`

A typed, asynchronous channel for a sequence of `T` values. Crucially, `stream<T>` is a Canonical ABI *value*, not a resource: it can be returned from a call, accepted as a parameter, and handed from one component to another without giving up ownership of the underlying buffer. The same value can also be passed straight through a middle component without that component having to relay any wake-ups.

```wit
read-via-stream: func() -> tuple<stream<u8>, future<result<_, error-code>>>;
```

### `future<T>`

A typed handle for a single value that will become available later. Like `stream<T>`, `future<T>` is a value rather than a resource, so it crosses component boundaries the same way a primitive does. A function returning `future<T>` does not block; the caller awaits the result when it needs it.

```wit
write-via-stream: func(data: stream<u8>) -> future<result<_, error-code>>;
```

## How the primitives work in WASI 0.3

### Stream plus terminal future

Reads return both a data channel and a completion handle, packed into a tuple:

```wit
read-via-stream: func() -> tuple<stream<u8>, future<result<_, error-code>>>;
```

The two halves are independent. The caller can consume the stream eagerly, sample it, or drop it part-way through; either way the future resolves once the operation has terminated, carrying the success-or-failure outcome. The same shape appears in stdin, filesystem reads, TCP receives, and directory listings.

### Stream parameter, future return

Writes use the symmetric shape: the guest supplies the data as a `stream<u8>` parameter, and the host returns a `future` that resolves once it has consumed the stream. Stdout, stderr, filesystem writes, and TCP sends all follow this shape:

```wit
write-via-stream: func(data: stream<u8>) -> future<result<_, error-code>>;
```

## Where to go next

For an end-to-end Rust example that uses these primitives in practice, see [Creating Runnable Components in Rust](../language-support/creating-runnable-components/rust.md). For runtime support and CLI flags, see [Wasmtime](../running-components/wasmtime.md). For the WIT syntax in detail, see [WIT Reference](./wit.md).
