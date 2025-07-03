# Frequently Asked Questions (FAQ)

This page hosts a series of questions that are frequently asked or that might be confusing about
WebAssembly (core), components, and the WebAssembly ecosystem as a whole.

## Q: What is the difference between a _component_ and _module_ in WebAssembly?

A WebAssembly module (more precisely referred to as a "WebAssembly core module") is a
binary that conforms to the [WebAssembly Core Specification][wasm-core-spec].

A WebAssembly component is a WebAssembly binary that:
- Adheres to the component model [binary format][cm-binary-format] (as opposed to a WebAssembly core binary format)
- Uses the [WebAssembly Interface types][wit] specification to encode type information.
- Adheres to the Component Model [Canonical ABI][cabi] for converting between rich types and those present in core WebAssembly.
WebAssembly Components can (and often do) contain core modules, but generally WebAssembly core modules
*cannot not* contain Components. One easy way to differentiate is by reading the WAT for a component:
## Q: How can I tell if a WebAssembly binary is a component or a module?
A WebAssembly core module generally consists of a `(module)` s-expression:
```wat
(module
  ;; ...
)
```

A WebAssembly component generally consists of a `(component)` s-expression (and may contain
nested `(module)`/`(component)` s-expressions):

```wat
(component
  ;; ...
)
```

One part that might cause confusion here is that a WASI Preview 1 "component" is in fact a
*core module*. More precisely, a Preview 1 "component" is a WebAssembly core module with a well-defined
set of imports and exports ([legacy specification][wasi-p1]).

[cabi]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/CanonicalABI.md
[wasi-p1]: https://github.com/WebAssembly/WASI/blob/main/legacy/preview1/witx/wasi_snapshot_preview1.witx
[wasm-core-spec]: https://webassembly.github.io/spec/core/

## Q: How do WebAssembly Components and the WebAssembly System Interface (WASI) relate to each other?

WebAssembly core modules use functions provided by the host system (the "outside world") but they can only perform
computations on a fixed set of primitive types (`i32`, `i64`, `f32`, `f64`), `v128`). The component model enables core components to interact with the "outside world"
 via a rich set of types ([WebAssembly Interface Types][wit]).

While WebAssembly core module *can* represent higher level types using the available primitives, every binary and platform may do so in an ad-hoc manner. The Component Model presents a consistent way of representing a rich set of types familiar in most high level languages that is consistent across binaries and platforms.

The WebAssembly System Interface (WASI) is a set of APIs for WASI being developed for eventual standardization by the WASI Subgroup, which is a subgroup of the WebAssembly Community Group.

WASI defines interfaces, functions and types that
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

WebAssembly components represent a new kind of binary that can *describe* its expected usage natively. This means that
WebAssembly components have functionality that they *export*, which users of the component (e.g. another component, or
a WebAssembly host) can use.

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

For the component that inhabits the `example-world` defined above, the outside world can expect the WebAssembly binary to
have a `say-hello` function that is callable via the `example-namespace:example-package/example-interface` interface.

The component is said to "export" the `example-interface` interface, making available the functions and types therein.

## Still have questions?

Please contribute to the Component Book by filing your question (or one that you think should be covered here) as
[an issue on GitHub][gh-issues].

[gh-issues-new]: https://github.com/bytecodealliance/component-docs/issues/new
