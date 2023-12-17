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

world {
    export validator;
    import docs:regex/match@0.1.0;
}

// component 'regex'
package docs:regex@0.1.0;

interface match {
    first-match: func(regex: string, text: string) -> string;
}

world {
    export match;
}
```

If we compose `validator` with `regex`, `validator`'s import of `docs:regex/match@0.1.0` is wired up to `regex`'s export of `match`. The net result is that the composed component exports `docs:validator/validator@0.1.0` and has no imports. The composed component does _not_ export `docs:regex/match@0.1.0` - that has become an internal implementation detail of the composed component.

Component composition tools are in their early stages right now.  Here are some tips to avoid or diagnose errors:

* Composition happens at the level of interfaces. If the initial component directly imports functions, then composition will fail. If composition reports an error such as "component `path/to/component` has a non-instance import named `<name>`" then check that all imports and exports are defined by interfaces.
* Composition is asymmetrical. It is not just "gluing components together" - it takes a primary component which has imports, and satisfies its imports using dependency components. For example, composing an implementation of `validator` with an implementation of `regex` makes sense because `validator` has a dependency that `regex` can satisfy; doing it the other way round doesn't work, because `regex` doesn't have any dependencies, let alone ones that `validator` can satisfy.
* Composition cares about interface versions, and current tools are inconsistent about when they infer or inject versions. For example, if a Rust component exports `test:mypackage`, `cargo component build` will decorate this with the crate version, e.g. `test:mypackage@0.1.0`. If another Rust component _imports_ an interface from `test:mypackage`, that won't match `test:mypackage@0.1.0`. You can use [`wasm-tools component wit`](https://github.com/bytecodealliance/wasm-tools/tree/main/crates/wit-component) to view the imports and exports embedded in the `.wasm` files and check whether they match up.

## Composing components with `wasm-tools`

The [`wasm-tools` suite](https://github.com/bytecodealliance/wasm-tools) includes a `compose` command which can be used to compose components at the command line.

To compose a component with the components it directly depends on, run:

```
wasm-tools compose path/to/component.wasm -d path/to/dep1.wasm -d path/to/dep2.wasm -o composed.wasm
```

Here `component.wasm` is the component that imports interfaces from `dep1.wasm` and `dep2.wasm`, which export them. The composed component, with those dependencies satisfied and tucked away inside it, is saved to `composed.wasm`.

> This syntax doesn't cover transitive dependencies. If, for example, `dep1.wasm` has unsatisfied imports that you want to satisfy from `dep3.wasm`, you'll need to use a [configuration file](https://github.com/bytecodealliance/wasm-tools/blob/main/crates/wasm-compose/CONFIG.md). (Or you can compose `dep1.wasm` with `dep3.wasm` first, then refer to that composed component instead of `dep1.wasm`. This doesn't scale to lots of transitive dependencies though!)

For full information about `wasm-tools compose` including how to configure more advanced scenarios, see [the `wasm-tools compose` documentation](https://github.com/bytecodealliance/wasm-tools/tree/main/crates/wasm-compose).

## Composing components with a visual interface

You can compose components visually using the builder app at https://wasmbuilder.app/.

1. Use the Add Component Button to upload the `.wasm` component files you want to compose. The components appear in the sidebar.

2. Drag the components onto the canvas. You'll see imports listed on the left of each component, and exports on the right.

3. Click the box in the top left to choose the 'primary' component, that is, the one whose exports will be preserved. (The clickable area is quite small - wait for the cursor to change from a hand to a pointer.)

4. To fulfil one of the primary component's imports with a dependency's export, drag from the "I" icon next to the export to the "I" item next to the import. (Again, the clickable area is quite small - wait for the cursor to change from a hand to a cross.)

5. When you have connected all the imports and exports that you want, click the Download Component button to download the composed component as a `.wasm` file.
