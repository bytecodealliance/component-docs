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
// component `bytecode-alliance:validator-impl`
package bytecode-alliance:validator@0.1.0;

interface validation {
    validate-text: func(text: string) -> string;
}

world validator-world {
    export validator;
    import bytecode-alliance:regex/match@0.1.0;
}

// component 'bytecode-alliance:regex-impl'
package bytecode-alliance:regex@0.1.0;

interface match {
    first-match: func(regex: string, text: string) -> string;
}

world regex-world {
    export match;
}
```

Here we have two WIT packages, `bytecode-alliance:validator` and `bytecode-alliance:regex`.  The component `bytecode-alliance:validator-impl` implements the world `bytecode-alliance:validator/validator-world` and the component `bytecode-alliance:regex-impl` implements the world `bytecode-alliance:regex/regex-world`, each of which could have been written in any guest language that targets the component model.

When we compose components together, we are specifying which *instance* of an imported interface we want our components to use.

If we compose `bytecode-alliance:validator-impl` with `bytecode-alliance:regex-impl`, `bytecode-alliance:validator-impl`'s import of the `bytecode-alliance:regex/match@0.1.0` interface is wired up to `bytecode-alliance:regex-impl`'s export of `match`. The net result is that the composed component exports an instance of the `bytecode-alliance:validator/validation@0.1.0` interface, and has no imports. The composed component does _not_ export `bytecode-alliance:regex/match@0.1.0` - that has become an internal implementation detail of the composed component.

Component composition tools are in their early stages right now.  Here are some tips to avoid or diagnose errors:

* Composition happens at the level of interfaces. If the initial component directly imports functions, then composition will fail. If composition reports an error such as "component `path/to/component` has a non-instance import named `<name>`" then check that all imports and exports are defined by interfaces.
* Composition is asymmetrical. It is not just "gluing components together" - it takes a primary component which has imports, and satisfies its imports using dependency components. For example, composing an implementation of `validator` with an implementation of `regex` makes sense because `validator` has a dependency that `regex` can satisfy; doing it the other way round doesn't work, because `regex` doesn't have any dependencies, let alone ones that `validator` can satisfy.
* Composition cares about interface versions, and current tools are inconsistent about when they infer or inject versions. For example, if a Rust component exports `test:mypackage`, `cargo component build` will decorate this with the crate version, e.g. `test:mypackage@0.1.0`. If another Rust component _imports_ an interface from `test:mypackage`, that won't match `test:mypackage@0.1.0`. You can use [`wasm-tools component wit`](https://github.com/bytecodealliance/wasm-tools/tree/main/crates/wit-component) to view the imports and exports embedded in the `.wasm` files and check whether they match up.

## Composing components with `wasm-tools`

The [`wasm-tools` suite](https://github.com/bytecodealliance/wasm-tools) includes a `compose` command which can be used to compose components at the command line.

To compose a component with the components it directly depends on, run:

```sh
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

## Composing components with the wac CLI

You can use the [wac](https://github.com/bytecodealliance/wac) CLI as well to create your composition.

The example composition described above could be created with the `wac` file below

```
//composition.wac
package bytecode-alliance:composition;

let regex = new bytecode-alliance:regex-impl { ... };
let validator = new bytecode-alliance:validator-impl { "bytecode-alliance:regex/match": regex.match, ... };

export validator...;
```

The `new` keyword here is used to create an instance of a component, namely `bytecode-alliance:regex-impl` and `bytecode-alliance:validator-impl` are instantiated (with instance names`regex` and `validator`) that target the `bytecode-alliance:validator/validator-world` and `bytecode-alliance:regex/regex-world` worlds respectively. 

When we instantiate the component `bytecode-alliance:validator-impl`, since its world targets `bytecode-alliance:validotor/validator-world` which imports the `bytecode-alliance:regex/match` interface, we need to specify which instance of that interface we want.  We can use the syntax `regex.match` here to say we want the one from the `regex` instance we got from instantiating `bytecodealliance:regex-impl`.

One of the nice features of composing this way, is that if you had two components that share a dependency on another component, but you don't want them to depend on the same instance, then you could create two separate instances for each of them to depend on.

You may also be wondering what's up with the `...` syntax.  It's common for components to import functionality from their host/runtime, like `wasi:filesystem` or `wasi:http` for example.  The `...` syntax is how we pass that functionality along to all of the components that comprise the composition we're authoring.

The components that you use in your composition can either be referenced from a registry or from a local file system.  There are a few ways to configure where you want your dependencies to live in a local setup, which are described in the [wac repo](https://github.com/bytecodealliance/wac#dependencies).

With all that, we can just run `wac encode composition.wac -o composition.wasm` and it will spit out a component that is runnable.

