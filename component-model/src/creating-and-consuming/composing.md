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

For example, consider the following WIT packages and components

```wit
// component `component-book:validator-impl`
package component-book:validator@0.1.0;

interface validation {
    validate-text: func(text: string) -> string;
}

world validator-world {
    export validator;
    import component-book:regex/match@0.1.0;
}

// component 'component-book:regex-impl'
package component-book:regex@0.1.0;

interface match {
    first-match: func(regex: string, text: string) -> string;
}

world regex-world {
    export match;
}
```

Here we have two WIT packages, `component-book:validator` and `component-book:regex`.  The component `component-book:validator-impl` implements the world `component-book:validator/validator-world` and the component `component-book:regex-impl` implements the world `component-book:regex/regex-world`, each of which could have been written in any guest language that targets the component model.

You can think of the components that people author as having their shape described by a world defined in a WIT package.  Since worlds import and export interfaces, and components implement worlds, when we author a component, we don't specify which implementations we're importing, but just the interfaces from the world we're targeting. When performing a composition, we are specifying which concrete implementation will serve as an *instance* of the imported interface we want to use in our composed output.

If we compose `component-book:validator-impl` with `component-book:regex-impl`, `component-book:validator-impl`'s import of the `component-book:regex/match@0.1.0` interface is wired up to `component-book:regex-impl`'s export of `match`. The net result is that the composed component exports an instance of the `component-book:validator/validation@0.1.0` interface, and has no imports. The composed component does _not_ export `component-book:regex/match@0.1.0` - that has become an internal implementation detail of the composed component.

Component composition tools are in their early stages right now.  Here are some tips to avoid or diagnose errors:

* Compositions will fail unless the imported/exported types correspond!  A component must export an interface that another component imports in order for the composition to succeed.  The name of the interface is not enough... the types defined in it must also match the expected types
* Composition cares about interface versions, and current tools are inconsistent about when they infer or inject versions. For example, if a Rust component exports `test:mypackage`, `cargo component build` will decorate this with the crate version, e.g. `test:mypackage@0.1.0`. If another Rust component _imports_ an interface from `test:mypackage`, that won't match `test:mypackage@0.1.0`. You can use [`wasm-tools component wit`](https://github.com/bytecodealliance/wasm-tools/tree/main/crates/wit-component) to view the imports and exports embedded in the `.wasm` files and check whether they match up.

## Composing components with the wac CLI

You can use the [wac](https://github.com/bytecodealliance/wac) CLI to create your composition.

The example composition described above could be created with the `wac` file below

```
//composition.wac
package component-book:composition;

let regex = new component-book:regex-impl { ... };
let validator = new component-book:validator-impl { "component-book:regex/match": regex.match, ... };

export validator...;
```

For an in depth description about how to use the wac tool, you can check out the [wac usage guide](https://github.com/bytecodealliance/wac/blob/main/LANGUAGE.md)

## Composing components with a visual interface

You can compose components visually using the builder app at https://wasmbuilder.app/.

1. Use the Add Component Button to upload the `.wasm` component files you want to compose. The components appear in the sidebar.

2. Drag the components onto the canvas. You'll see imports listed on the left of each component, and exports on the right.

3. Click the box in the top left to choose the 'primary' component, that is, the one whose exports will be preserved. (The clickable area is quite small - wait for the cursor to change from a hand to a pointer.)

4. To fulfil one of the primary component's imports with a dependency's export, drag from the "I" icon next to the export to the "I" item next to the import. (Again, the clickable area is quite small - wait for the cursor to change from a hand to a cross.)

5. When you have connected all the imports and exports that you want, click the Download Component button to download the composed component as a `.wasm` file.
