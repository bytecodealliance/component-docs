# Migrating from WASI P2 to WASI P3

WASI P3 reshapes WASI's interfaces around the [native async primitives](./async.md) `async func`, `stream<T>`, and `future<T>`. Most of the changes in `wasi:cli`, `wasi:http`, `wasi:filesystem`, and `wasi:sockets` are consequences of moving to these primitives.

This page covers the mapping between concepts in WASI P2 and WASI P3. For a WIT-level comparison of every WASI P3 interface, see [WASI P3](https://wasi.dev/releases/wasi-p3) on WASI.dev.

## Do you need to migrate?

Not immediately. WASI P3 runtimes can polyfill P2 by mapping P2 imports onto native P3 primitives at the host boundary, and Wasmtime's `wasmtime serve` already runs both P3 and P2 components from the same binary, dispatching per component. Migration is the right call when you want:

- Composable async across component boundaries (the [sandwich problem](./async.md#why-native-async) goes away).
- The newer interface shapes — in particular, `wasi:http`'s collapse of eight resources down to two.
- First-class support in P3-targeted toolchains as they continue to land.

## Concept mapping

WASI P3 replaces every `wasi:io` resource with a Canonical ABI primitive. The translation is mostly one-to-one:

| WASI P2 (`wasi:io`)              | WASI P3 (Component Model)                |
| -------------------------------- | ---------------------------------------- |
| `resource pollable`              | `future<T>`                              |
| `resource input-stream`          | `stream<u8>`                             |
| `resource output-stream`         | `stream<u8>` (passed *into* the call)    |
| `poll(list<pollable>)`           | `await` on a future                      |
| `subscribe()` on a resource      | return a `future` from the call          |
| `start-foo` / `finish-foo`       | a single `func` or `async func`          |

## What changed in WIT

### Stream-plus-future for reads

In P2, a read call returned an `input-stream`. In P3, it returns a tuple of a `stream<u8>` plus a `future<result<_, error-code>>` that resolves once the stream closes:

```wit
// WASI P2 (filesystem read)
read-via-stream: func(offset: filesize) -> result<input-stream, error-code>;

// WASI P3 (filesystem read)
read-via-stream: func(offset: filesize) -> tuple<stream<u8>, future<result<_, error-code>>>;
```

The stream delivers data incrementally. The future signals terminal success or failure independently of how much of the stream the caller consumes.

### Write-direction flip

P2 write calls handed you an `output-stream` to write into. P3 reverses the direction: you pass a `stream<u8>` *into* the call and receive a `future<result>` that resolves when the write completes:

```wit
// WASI P2: get a stream, write into it
get-stdout: func() -> output-stream;

// WASI P3: pass data in, get a completion future
write-via-stream: func(data: stream<u8>) -> future<result<_, error-code>>;
```

### Two-step calls collapsed

P2 modeled operations that could suspend as a `start-foo` / `finish-foo` pair, with a `pollable` for readiness in between. P3 collapses each pair into a single call:

```wit
// WASI P2
start-connect: func(network: borrow<network>, remote-address: ip-socket-address) -> result<_, error-code>;
finish-connect: func() -> result<tuple<input-stream, output-stream>, error-code>;

// WASI P3
connect: async func(remote-address: ip-socket-address) -> result<_, error-code>;
```

The collapsed call is `async func` when the operation needs to suspend in the host (such as `connect`); operations that historically only used the two-step shape for non-blocking dispatch may collapse to plain `func` instead (`bind`, `listen`).

## Interface notes

### `wasi:io` (removed)

The package is gone. Everything it expressed is now Canonical ABI primitives.

### `wasi:http`

The most dramatic restructuring. The eight P2 resources (`incoming-request`, `outgoing-request`, `incoming-response`, `outgoing-response`, `incoming-body`, `outgoing-body`, `future-trailers`, and `future-incoming-response`) collapse to two: `request` and `response`. Bodies are `stream<u8>`; trailers are a `future`. The handler is an `async func`:

```wit
// WASI P2
handle: func(request: incoming-request, response-out: response-outparam);

// WASI P3
handle: async func(request: request) -> result<response, error-code>;
```

The `proxy` world is replaced by `service`, with a new `middleware` world that imports and exports the handler.

### `wasi:sockets`

The seven P2 interfaces (`network`, `instance-network`, `tcp`, `tcp-create-socket`, `udp`, `udp-create-socket`, `ip-name-lookup`) collapse to a unified `types` interface containing both `tcp-socket` and `udp-socket` resources, plus `ip-name-lookup`. The `network` resource is removed entirely; network access is now granted at the world level. `start-*` / `finish-*` pairs collapse as described above. TCP `listen` returns `stream<tcp-socket>` directly instead of requiring a separate `accept` call.

### `wasi:filesystem`

Most `descriptor` methods are `async func`. Streaming reads and writes use the stream-plus-future pattern; `read-directory` returns `stream<directory-entry>`. The `error-code` enum gains a catch-all `other(option<string>)` variant.

### `wasi:cli`

stdin, stdout, and stderr use `stream<u8>` with the stream-plus-future pattern. `run` becomes `async func`. A shared `wasi:cli/types` interface carries an `error-code` variant. The `exit-with-code` function stabilizes alongside `exit`.

### `wasi:clocks`

`wall-clock` is renamed to `system-clock`, and `datetime` is renamed to `instant`. The `instant` record uses `s64` seconds (instead of `u64`), supporting pre-epoch timestamps. `subscribe-instant` and `subscribe-duration` are replaced by `wait-until` and `wait-for` `async func`s.

### `wasi:random`

The `len` parameter is renamed to `max-len` on `get-random-bytes` and `get-insecure-random-bytes`. Implementations may now return fewer bytes than requested, so callers should loop.

## Tooling requirements

| Tool          | Minimum                                                         | Notes                                                               |
| ------------- | --------------------------------------------------------------- | ------------------------------------------------------------------- |
| Wasmtime      | 43+ for `wasmtime run`; 44+ for `wasmtime serve`                | Enable with `-Sp3 -W component-model-async=y`.                      |
| `wit-bindgen` | 0.46+                                                           | Use the `async` feature for P3 binding generation.                  |
| jco           | latest                                                          | P3 host bindings ship in the `preview3-shim` package.               |
| `wkg`         | 0.15+                                                           | Required to fetch `wasi:cli@0.3.0-rc-2026-03-15` and related packages. |
| Rust          | nightly                                                         | Current stable bundles a `wasm-component-ld` too old for P3 outputs of `wit-bindgen` 0.58. |

> **Version pinning.** As of WASI 0.3.0's release on 2026-06-11, Wasmtime and `wit-bindgen` still vendor the `0.3.0-rc-2026-03-15` snapshot of the WIT. Components pinning to the published `0.3.0` will fail to instantiate against current Wasmtime; use the RC pin until those tools refresh.

## Further reading

- [Async, Streams, and Futures](./async.md) — the conceptual foundation
- [Creating Runnable Components in Rust](../language-support/creating-runnable-components/rust.md) — worked Rust example with the P3 `async fn run()` pattern
- [WASI P3](https://wasi.dev/releases/wasi-p3) on WASI.dev — full WIT-level diff per interface
