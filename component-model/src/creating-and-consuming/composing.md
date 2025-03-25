# Composing Components

Because the WebAssembly component model packages code in a portable binary format, and provides machine-readable interfaces in [WIT](../design/wit.md) with a standardised ABI (Application Binary Interface), it enables applications and components to work together, no matter what languages they were originally written in. In the same way that, for example, a Rust package (crate) can be compiled together with other Rust code to create a higher-level library or an application, a Wasm component can be linked with other components.

> Component model interoperation is more convenient and expressive than language-specific foreign function interfaces. A typical C FFI involves language-specific types, so it is not possible to link between arbitrary languages without at least some C-language wrapping or conversion. The component model, by contrast, provides a common way of expressing interfaces, and a standard binary representation of those interfaces. So if an import and an export have the same shape, they fit together directly.

## What is composition?

When you compose components, you wire up the imports of one "primary" component to the exports of one or more other "dependency" components, creating a new component. The new component, like the original components, is a `.wasm` file, and its interface is defined as:

* The new component _exports_ the same exports as the primary component
* The new component _does not export_ the exports of the dependencies
* The new component _imports_ all the imports of the dependency components
* The new component _imports_ any imports of the primary component imports that the dependencies didn't satisfy
* If several components import the same interface, the new component imports that interface - it doesn't "remember" that the import was declared in several different places

For example, consider two components with the following worlds:

```wit
// component `validator`
package docs:validator@0.1.0;

interface validator {
    validate-text: func(text: string) -> string;
}

world validator {
    export validator;
    import docs:regex/match@0.1.0;
}
```

```wit
// component 'regex'
package docs:regex@0.1.0;

interface match {
    first-match: func(regex: string, text: string) -> string;
}

world regex {
    export match;
}
```

If we compose `validator` with `regex`, `validator`'s import of `docs:regex/match@0.1.0` is wired up to `regex`'s export of `match`. The net result is that the composed component exports `docs:validator/validator@0.1.0` and has no imports. The composed component does _not_ export `docs:regex/match@0.1.0` - that has become an internal implementation detail of the composed component.

Component composition tools are in their early stages right now.  Here are some tips to avoid or diagnose errors:

* Composition happens at the level of interfaces. If the initial component directly imports functions, then composition will fail. If composition reports an error such as "component `path/to/component` has a non-instance import named `<name>`" then check that all imports and exports are defined by interfaces.
* Composition is asymmetrical. It is not just "gluing components together" - it takes a primary component which has imports, and satisfies its imports using dependency components. For example, composing an implementation of `validator` with an implementation of `regex` makes sense because `validator` has a dependency that `regex` can satisfy; doing it the other way round doesn't work, because `regex` doesn't have any dependencies, let alone ones that `validator` can satisfy.
* Composition cares about interface versions, and current tools are inconsistent about when they infer or inject versions. For example, if a Rust component exports `test:mypackage`, `cargo component build` will decorate this with the crate version, e.g. `test:mypackage@0.1.0`. If another Rust component _imports_ an interface from `test:mypackage`, that won't match `test:mypackage@0.1.0`. You can use [`wasm-tools component wit`](https://github.com/bytecodealliance/wasm-tools/tree/main/crates/wit-component) to view the imports and exports embedded in the `.wasm` files and check whether they match up.

## Composing components with WAC

You can use the [WAC](https://github.com/bytecodealliance/wac) CLI to compose components at the command line.

To perform quick and simple compositions, use the `wac plug` command. `wac plug` satisfies the import of a "socket" component by plugging a "plug" component's export into the socket. For example, a component that implements the [`validator` world above](#what-is-composition) needs to satisfy it's `match` import. It is a socket. While a component that implements the `regex` world, exports the `match` interface, and can be used as a plug. `wac plug` can plug a regex component's export into the validator component's import, creating a resultant composition:

```console
wac plug validator-component.wasm --plug regex-component.wasm -o composed.wasm
```

A component can also be composed with two components it depends on.

```console
wac plug path/to/component.wasm --plug path/to/dep1.wasm --plug path/to/dep2.wasm -o composed.wasm
```

Here `component.wasm` is the component that imports interfaces from `dep1.wasm` and `dep2.wasm`, which export them. The composed component, with those dependencies satisfied and tucked away inside it, is saved to `composed.wasm`.

The `plug` syntax doesn't cover transitive dependencies. If, for example, `dep1.wasm` has unsatisfied imports that you want to satisfy from `dep3.wasm`, you'd need to be deliberate about the order of your composition. You could compose `dep1.wasm` with `dep3.wasm` first, then refer to that composed component instead of `dep1.wasm`. However, this doesn't scale to lots of transitive dependencies, which is why the WAC language was created.

### Advanced composition with the WAC language

`wac plug` is a convenience to achieve a common pattern in component compositions like above. However, composition can be arbitrarily complicated. In cases where `wac plug` is not sufficient, the [WAC language](https://github.com/bytecodealliance/wac/blob/main/LANGUAGE.md) can give us the ability to create arbitrarily complex compositions.

In a WAC file, you use the WAC language to describe a composition. For example, the following is a WAC file that could be used to create that validator component from [earlier](#what-is-composition).

```
//composition.wac
// Provide a package name for the resulting composition
package docs:composition;

// Instantiate the regex-impl component that implements the `regex` world. Bind this instance's exports to the local name `regex`.
let regex = new docs:regex-impl { };

// Instantiate the validator-impl component which implements the `validator` world and imports the match interface from the regex component.
let validator = new docs:validator-impl { match: regex.match, ... };

// Export all remaining exports of the validator instance
export validator...;
```

Then, `wac compose` can be used to compose the components, passing in the paths to the components. Alternatively, you can place the components in a `deps` directory with an expected structure, and in the near future, you will be able to pull in components from registries. See the [`wac` documentation](https://github.com/bytecodealliance/wac) for more details.

```console
wac compose --dep docs:regex-impl=regex-component.wasm --dep docs:validator-impl=validator-component.wasm -o composed.wasm composition.wac
```

For an in depth description about how to use the wac tool, you can check out the [wac language index](https://github.com/bytecodealliance/wac/blob/main/LANGUAGE.md) and [examples](https://github.com/bytecodealliance/wac/tree/main/examples).

## Composing components with a visual interface

You can compose components visually using the builder app at [wasmbuilder.app](https://wasmbuilder.app/).

1. Use the Add Component Button to upload the `.wasm` component files you want to compose. The components appear in the sidebar.

2. Drag the components onto the canvas. You'll see imports listed on the left of each component, and exports on the right.

3. Click the box in the top left to choose the 'primary' component, that is, the one whose exports will be preserved. (The clickable area is quite small - wait for the cursor to change from a hand to a pointer.)

4. To fulfil one of the primary component's imports with a dependency's export, drag from the "I" icon next to the export to the "I" item next to the import. (Again, the clickable area is quite small - wait for the cursor to change from a hand to a cross.)

5. When you have connected all the imports and exports that you want, click the Download Component button to download the composed component as a `.wasm` file.
