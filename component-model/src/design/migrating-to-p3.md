# Migrating from WASI 0.2 to WASI 0.3

WASI 0.3 reshapes WASI's interfaces around the [native async primitives](./async.md) `async func`, `stream<T>`, and `future<T>`. Most of the changes in `wasi:cli`, `wasi:http`, `wasi:filesystem`, and `wasi:sockets` are consequences of moving to these primitives.

This page covers the mapping between concepts in WASI 0.2 and WASI 0.3. For a WIT-level comparison of every WASI 0.3 interface, see [WASI 0.3](https://wasi.dev/releases/wasi-p3) on WASI.dev.

## Do you need to migrate?

Not immediately. WASI 0.3 runtimes can polyfill 0.2 by mapping 0.2 imports onto native 0.3 primitives at the host boundary, and Wasmtime's `wasmtime serve` already runs both 0.3 and 0.2 components from the same binary, dispatching per component. Migration is the right call when you want:

- Composable async across component boundaries (the [sandwich problem](./async.md#native-async) goes away).
- The newer interface shapes — in particular, `wasi:http`'s collapse of nine resources down to two.
- First-class support in 0.3-targeted toolchains as they continue to land.

## Concept mapping

WASI 0.3 replaces every `wasi:io` resource with a Canonical ABI primitive. The translation is mostly one-to-one:

| WASI 0.2 (`wasi:io`)              | WASI 0.3 (Component Model)                |
| -------------------------------- | ---------------------------------------- |
| `resource pollable`              | `future<T>`                              |
| `resource input-stream`          | `stream<u8>`                             |
| `resource output-stream`         | `stream<u8>` (passed *into* the call)    |
| `poll(list<pollable>)`           | `await` on a future                      |
| `subscribe()` on a resource      | return a `future` from the call          |
| `start-foo` / `finish-foo`       | a single `func` or `async func`          |

## What changed in WIT

### Stream-plus-future for reads

A 0.2 read call returned a single `input-stream` resource and surfaced terminal errors only as you consumed it. 0.3 splits those concerns: the call returns a `stream<u8>` for the data and a `future<result<_, error-code>>` for the outcome, packed into a tuple.

```wit
// WASI 0.2 (filesystem read)
read-via-stream: func(offset: filesize) -> result<input-stream, error-code>;

// WASI 0.3 (filesystem read)
read-via-stream: func(offset: filesize) -> tuple<stream<u8>, future<result<_, error-code>>>;
```

In 0.3 the caller does not have to drain the stream to learn whether the read finished cleanly; the future resolves either way.

### Write-direction flip

0.2 write paths handed a guest some host-owned resource (an `output-stream`) and let the guest push bytes into it. 0.3 inverts that: the guest supplies the data as a `stream<u8>` value, and the host returns a `future` that resolves once it has finished consuming the stream.

```wit
// WASI 0.2: receive an output-stream resource, write into it
get-stdout: func() -> output-stream;

// WASI 0.3: pass a stream value in, receive a completion future
write-via-stream: func(data: stream<u8>) -> future<result<_, error-code>>;
```

### Two-step calls collapsed

0.2 modeled operations that could suspend as a `start-foo` / `finish-foo` pair, with a `pollable` for readiness in between. 0.3 collapses each pair into a single call:

```wit
// WASI 0.2
start-connect: func(network: borrow<network>, remote-address: ip-socket-address) -> result<_, error-code>;
finish-connect: func() -> result<tuple<input-stream, output-stream>, error-code>;

// WASI 0.3
connect: async func(remote-address: ip-socket-address) -> result<_, error-code>;
```

The collapsed call is `async func` when the operation needs to suspend in the host (such as `connect`); operations that historically only used the two-step shape for non-blocking dispatch may collapse to plain `func` instead (`bind`, `listen`).

## Interface highlights

The complete per-interface diff lives on [WASI 0.3](https://wasi.dev/releases/wasi-p3#what-changed-in-each-interface) at WASI.dev. The three changes most likely to drive migration work are:

- **`wasi:io` is gone.** The package has no 0.3.0 release. Every resource it exposed (`pollable`, `input-stream`, `output-stream`) is replaced by a Component Model primitive, per the [concept mapping](#concept-mapping) above.
- **`wasi:http` collapses from nine resources to two.** The incoming/outgoing × request/response/body matrix plus `future-trailers`, `future-incoming-response`, and `response-outparam` all become `request` and `response`, with `stream<u8>` bodies and a `future` for trailers. The handler is now an `async func`:

```wit
// WASI 0.2
handle: func(request: incoming-request, response-out: response-outparam);

// WASI 0.3
handle: async func(request: request) -> result<response, error-code>;
```

The `proxy` world is replaced by `service`, and a new `middleware` world both imports and exports the handler.
- **`wasi:sockets` drops its `network` resource.** Network access is granted at the world level instead of being threaded through every `bind`, `connect`, and DNS lookup. The seven 0.2 socket interfaces consolidate into one `types` interface plus `ip-name-lookup`, and TCP `listen` returns `stream<tcp-socket>` directly instead of requiring a separate `accept` loop.

Smaller per-interface changes — filesystem methods becoming `async func`, the `wasi:clocks` rename pass (`wall-clock` → `system-clock`, `datetime` → `instant`), the `max-len` rename in `wasi:random`, the new shared `wasi:cli/types` interface — are documented in the WASI.dev page linked above.

## Tooling requirements

| Tool          | Minimum                                                         | Notes                                                               |
| ------------- | --------------------------------------------------------------- | ------------------------------------------------------------------- |
| Wasmtime      | 43+ for `wasmtime run`; 44+ for `wasmtime serve`                | Enable with `-Sp3 -W component-model-async=y`.                      |
| `wit-bindgen` | 0.46+                                                           | Use the `async` feature for 0.3 binding generation.                  |
| jco           | latest                                                          | 0.3 host bindings ship in the `preview3-shim` package.               |
| `wkg`         | 0.15+                                                           | Required to fetch `wasi:cli@0.3.0-rc-2026-03-15` and related packages. |
| Rust          | nightly                                                         | Current stable bundles a `wasm-component-ld` too old for 0.3 outputs of `wit-bindgen` 0.58. |

> **Version pinning.** As of WASI 0.3.0's release on 2026-06-11, Wasmtime and `wit-bindgen` still vendor the `0.3.0-rc-2026-03-15` snapshot of the WIT. Components pinning to the published `0.3.0` will fail to instantiate against current Wasmtime; use the RC pin until those tools refresh.

## Further reading

- [Async, Streams, and Futures](./async.md) — the conceptual foundation
- [Creating Runnable Components in Rust](../language-support/creating-runnable-components/rust.md) — worked Rust example with the 0.3 `async fn run()` pattern
- [WASI 0.3](https://wasi.dev/releases/wasi-p3) on WASI.dev — full WIT-level diff per interface
