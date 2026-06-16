# Frequently Asked Questions (FAQ)

This page hosts a series of questions that are frequently asked
along with descriptions of concepts that may be confusing with regards to core WebAssembly,
WebAssembly components (i.e. the Component Model), and the WebAssembly ecosystem as a whole.

## Q: What is the difference between a _module_ and _component_ in WebAssembly?

A WebAssembly module (more precisely referred to as a "WebAssembly core module")
is a binary that conforms to the [WebAssembly Core Specification][wasm-core-spec].

A WebAssembly component:
- Adheres to the component model [binary format][cm-binary-format] (as opposed to the WebAssembly core binary format).
- Uses the [WebAssembly Interface types][wit] specification to encode type information.
- Adheres to the Component Model [Canonical ABI][cabi] for converting between rich types
  and those present in core WebAssembly.

WebAssembly Components can (and often do) contain core modules,
but generally WebAssembly core modules *cannot* contain components.
WebAssembly components and WebAssembly core modules have a different binary format.

WebAssembly components can be expressed via both a binary and textual format (["WAT", the WebAssembly Text format][wat]).

[wat]: https://webassembly.github.io/spec/core/text/index.html
[cabi]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/CanonicalABI.md
[cm-binary-format]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/Binary.md
[wasi-p1]: https://github.com/WebAssembly/WASI/blob/main/legacy/preview1/witx/wasi_snapshot_preview1.witx
[wasm-core-spec]: https://webassembly.github.io/spec/core/

## Q: How can I tell if a WebAssembly binary is a component or a module?

After converting a WebAssembly binary to its textual format
(e.g. via a tool like [`wasm-tools print`][wasm-tools-examples]),
it is easy to tell a WebAssembly core module and a WebAssembly component apart.

A WebAssembly core module generally consists of a top level `(module)` [s-expression][s-expression]:
```wat
(module
  ;; ...
)
```

A WebAssembly component generally consists of a `(component)` s-expression
(and may contain nested `(core:module)`/`(component)` s-expressions):

```wat
(component
  ;; ...
)
```

[WASM-tools-examples]: https://github.com/bytecodealliance/wasm-tools?tab=readme-ov-file#examples
[s-expression]: https://en.wikipedia.org/wiki/S-expression

## Q: How do WebAssembly Components and the WebAssembly System Interface (WASI) relate to each other?

While WebAssembly core modules *can* represent higher-level types using the available primitives,
every binary and platform may do so in an *ad hoc* manner.
The Component Model presents a representation for a rich set of
types—familiar from most high-level languages—that is consistent across binaries and platforms.
The set of rich types that can be used by WebAssembly components is called [WebAssembly Interface Types (WIT)][wit].

The WebAssembly System Interface (WASI) is a set of APIs (specified in WIT)
developed for eventual standardization by the WASI Subgroup, which is a subgroup of the WebAssembly Community Group.
WASI defines interfaces, functions and types that a system or platform can expose to a WebAssembly component.
At a glance, many parts of WASI are UNIX-like, in that they match traditional expectations for programs
like `stdin`, `stdout`, and writing to files.

Some WASI system interfaces work at a much higher level than the command line, however,
like [`wasi:http`][wasi-http].
`wasi:http` is included as a standardized platform due to the ubiquity of the Internet
and the common use case of WebAssembly components with the Web as a platform.

With WIT, platform builders can define *any* interface that WebAssembly components
expect to access—WASI enables building interfaces on top of a shared standard set of abstractions.

[wit]: https://component-model.bytecodealliance.org/design/wit.html
[wasi-http]: https://github.com/WebAssembly/WASI/tree/main/proposals/http

## Q: I see the terms Preview 1, Preview 2, and Preview 3 frequently. What do those refer to?

Preview 1 refers to [the first iteration of the Component Model](https://github.com/WebAssembly/WASI/tree/wasi-0.1)
which was based on WITX and is now deprecated.
Preview 2 refers to [a newer iteration of the Component Model](https://github.com/WebAssembly/WASI/blob/main/docs/Preview2.md)
which uses WebAssembly Interface Types (WIT).
Preview 3 (WASI 0.3, released June 11, 2026) adds native async to the Component Model:
`async func`, `stream<T>`, and `future<T>` are now Canonical ABI primitives, and the `wasi:io` package is removed.
See [Async, Streams, and Futures](../design/async.md) for the conceptual underpinnings
and [Migrating from WASI 0.2 to WASI 0.3](../design/migrating-to-p3.md) for what changes for an existing component.

Many programming language toolchains may only support Preview 1 components natively,
but this isn't a problem in practice as Preview 1 components can be *adapted* into Preview 2 components automatically.

While somewhat confusing, a WASI Preview 1 "component" is in fact a *WebAssembly core module*.
More precisely, a Preview 1 "component" is a WebAssembly core module with a well-defined set of imports and exports ([legacy specification][wasi-p1]).

## Q: What is WASI 0.3?

WASI 0.3 (WASI 0.3) is the latest milestone release of WASI, published on June 11, 2026.
It rebases WASI's interfaces onto three new Canonical ABI primitives that the Component Model added for this release:
[`async func`, `stream<T>`, and `future<T>`](../design/async.md).
The `wasi:io` package is removed; its functionality is now provided by the Component Model directly.

The practical effect is smaller, more composable WASI interfaces.
The `wasi:http` handler is an `async func` returning a response;
stdin, stdout, and stderr use `stream<u8>`;
and async operations propagate readiness across component boundaries without each component running its own event loop.

For a per-interface overview of what changed, see [WASI 0.3](https://wasi.dev/releases/wasi-p3) on WASI.dev.

## Q: Do I need to migrate from WASI 0.2 to WASI 0.3?

Not immediately.
WASI 0.3 runtimes can polyfill 0.2 by mapping 0.2 imports onto native 0.3 primitives at the host boundary,
and Wasmtime's `wasmtime serve` already runs both 0.3 and 0.2 components from the same binary, dispatching per component.

Migrating is the right call when you want composable async across component boundaries
or the newer interface shapes — in particular, `wasi:http`'s collapse of nine resources down to two.

See [Migrating from WASI 0.2 to WASI 0.3](../design/migrating-to-p3.md) for the concept mapping
and per-interface highlights.

## Q: What happened to `wasi:io`?

WASI 0.3 removes the `wasi:io` package entirely.
The resources it provided (`pollable`, `input-stream`, and `output-stream`)
are replaced by the Component Model's native Canonical ABI primitives
`future<T>`, `stream<u8>`, and `stream<u8>` (used as a function parameter), respectively.

The motivation was the so-called sandwich problem:
when async readiness was expressed as a resource scoped to a single component,
that readiness signal could not propagate cleanly across component boundaries.
With native async in the Canonical ABI, the runtime owns scheduling and wake-up propagation.

See [Async, Streams, and Futures](../design/async.md) for the underlying concepts.

## Q: What are component imports?

WebAssembly components are self-describing: information about required external functionality
(which must be provided by the platform or another component) is included in the binary.
For example, a WebAssembly component that may require outside environment variables may *import*
a WASI interface like `wasi:cli/environment`.

> [!NOTE]
> The values provided by the `wasi:cli/environment` interface are not guaranteed
> to be environment variables on the host machine—this is a choice left to the platform,
> in the implementation of `wasi:cli/environment` that it exposes.
>
> For example, platforms may choose to elide sensitive environment variables, or provide none at all, in practice.

Imports are most easily illustrated with WIT:

```wit
{{#include ../../examples/faq/example.wit}}
```

The [`environment` interface in `wasi:cli`][wasi-cli-env] provides various types and functions
for interacting with environment variables.

The component is said to "import" the `wasi:cli/environment` interface,
using the available functions and types therein.

[wasi-cli-env]: https://github.com/WebAssembly/WASI/blob/main/proposals/cli/wit/environment.wit

## Q: What are component exports?

WebAssembly components are self-describing: along with imports, WebAssembly components
can also describe what functionality they *export*, which callers of the component
(e.g. another component or a WebAssembly host) can reference.

Exports are easiest illustrated with WIT:

```wit
{{#include ../../examples/faq/example-hello.wit}}
```

For a component that implements the `example-world` defined above,
callers can expect the WebAssembly binary to have a `say-hello` function that is callable
via the `example-namespace:example-package/example-interface` interface.

The component is said to "export" the `example-interface` interface, making available the functions and types therein.

## Still have questions?

Please contribute to the Component Book by filing your question (or one that you think should be covered here)
as [an issue on GitHub][gh-issues-new].

[gh-issues-new]: https://github.com/bytecodealliance/component-docs/issues/new

[!NOTE]: #
