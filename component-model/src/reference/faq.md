# Frequently Asked Questions (FAQ)

This page hosts a series of questions that are frequently asked along with descriptions
of concepts that may be confusing with regards to core WebAssembly, WebAssembly components
(i.e. the Component Model), and the WebAssembly ecosystem as a whole.

## Q: What is the difference between a _module_ and _component_ in WebAssembly?

A WebAssembly module (more precisely referred to as a "WebAssembly core module") is a
binary that conforms to the [WebAssembly Core Specification][wasm-core-spec].

A WebAssembly component:
- Adheres to the component model [binary format][cm-binary-format] (as opposed to a WebAssembly core binary format)
- Uses the [WebAssembly Interface types][wit] specification to encode type information.
- Adheres to the Component Model [Canonical ABI][cabi] for converting between rich types and those present in core WebAssembly.

WebAssembly Components can (and often do) contain core modules, but generally WebAssembly core modules
*cannot* contain Components. WebAssembly components and WebAssembly core modules have a different binary format.

WebAssembly components can be expressed via both a binary and textual format (["WAT", the WebAssembly Text format][wat]).

[wat]: https://webassembly.github.io/spec/core/text/index.html
[cabi]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/CanonicalABI.md
[cm-binary-format]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/Binary.md
[wasi-p1]: https://github.com/WebAssembly/WASI/blob/main/legacy/preview1/witx/wasi_snapshot_preview1.witx
[wasm-core-spec]: https://webassembly.github.io/spec/core/

## Q: How can I tell if a WebAssembly binary is a component or a module?

After converting a WebAssembly binary to it's textual format (e.g. via a tool like [`wasm-tools print`][wasm-tools-examples]),
it is easy to tell a WebAssembly core module and a WebAssembly component apart.

A WebAssembly core module generally consists of a top level `(module)` s-expression:
```wat
(module
  ;; ...
)
```

A WebAssembly component generally consists of a `(component)` s-expression (and may contain
nested `(core:module)`/`(component)` s-expressions):

```wat
(component
  ;; ...
)
```

[WASM-tools-examples]: https://github.com/bytecodealliance/wasm-tools?tab=readme-ov-file#examples

## Q: How do WebAssembly Components and the WebAssembly System Interface (WASI) relate to each other?

While WebAssembly core module *can* represent higher level types using the available primitives, every binary and platform
may do so in an ad-hoc manner. The Component Model presents a consistent way of representing a rich set of types familiar in
most high level languages that is consistent across binaries and platforms.

The set of rich types which can be used by WebAssembly components are called [WebAssembly Interface Types (WIT)][wit].

The WebAssembly System Interface (WASI) is a set of APIs (specified in WIT) developed for eventual standardization by the WASI
Subgroup, which is a subgroup of the WebAssembly Community Group. WASI defines interfaces, functions and types that
a system or platform can expose to a WebAssembly component. At a glance, many parts of WASI are UNIX-like,
in that they match traditional expectations for programs like STDIN, STDOUT, and writing to files.

Some WASI system interfaces work at a much higher level than the command line however, like
[`wasi:http`][wasi-http]. `wasi:http` is included as a standardized platform due to the ubiquity
of the internet and the common use case of WebAssembly components with "the web" as a platform.

With WIT, platform builders can define *any* interface that WebAssembly components
expect to access -- WASI is a standardized set which enables to build on a shared base set of abstractions.

[wit]: https://component-model.bytecodealliance.org/design/wit.html
[wasi-http]: https://github.com/WebAssembly/wasi-http

## Q: I see the terms Preview 1 and Preview 2 frequently. What do those refer to?

Preview 1 refers to the first iteration of the Component Model which was based on WITX and is now deprecated:

https://github.com/WebAssembly/WASI/tree/main/legacy

Preview 2 refers to a newer iteration of the Component Model which uses WebAssembly Interface Types (WIT):

https://github.com/WebAssembly/WASI/tree/main/wasip2

Many programming language toolchains may only support Preview 1 components natively, but this isn't a problem
in practice as Preview 1 components can be *adapted* into Preview 2 components automatically.

While somewhat confusing a WASI Preview 1 "component" is in fact a *WebAssembly core module*. More precisely, a
Preview 1 "component" is a WebAssembly core module with a well-defined set of imports and exports ([legacy specification][wasi-p1]).

## Q: What are component imports?

WebAssembly components are self-describing -- information about required external functionality (which must be provided by the platform or another component) is included in the binary.
For example, a WebAssembly component that may require some use of outside environment variables may *import* a WASI interface like `wasi:cli/environment`.

> [!NOTE]
> The values provided by the `wasi:cli/environment` are not guaranteed
> to be ENV variables on the host machine -- this is a choice left to the
> platform, in the implementation of `wasi:cli/environment` that it exposes.
>
> For example, platforms may choose to elide sensitive environment variables, or provide none at all, in practice.

Imports are easiest illustrated with WIT:

```wit
package example-namespace:example-package;

world example-world {
    import wasi:cli/environment@0.2.4;
}
```

The [`environment` interface in `wasi:cli`][wasi-cli-env] provides various types and functions for interacting with
environment variables.

The component is said to "import" the `wasi:cli/environment` interface, using the available functions and types therein.

[wasi-cli-env]: https://github.com/WebAssembly/wasi-cli/blob/main/wit/environment.wit

## Q: What are component exports?

WebAssembly components are self-describing -- along with imports, WebAssembly components can also describe what functionality
they *export*, which callers of the component (e.g. another component, a WebAssembly host) can reference.

Exports are easiest illustrated with WIT:

```wit
package example-namespace:example-package;

interface example-interface {
    say-hello: func(name: string) -> string;
}

world example-world {
    export example-interface;
}
```

For the component that inhabits the `example-world` defined above, callers can expect the WebAssembly binary to
have a `say-hello` function that is callable via the `example-namespace:example-package/example-interface` interface.

The component is said to "export" the `example-interface` interface, making available the functions and types therein.

## Still have questions?

Please contribute to the Component Book by filing your question (or one that you think should be covered here) as
[an issue on GitHub][gh-issues].

[gh-issues-new]: https://github.com/bytecodealliance/component-docs/issues/new

[!NOTE]: #
