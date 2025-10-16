# Composing Components

The WebAssembly component model enables applications and components to work together,
no matter what languages they were originally written in.
The component model accomplishes this by packaging code in a portable binary format
and providing  machine-readable interfaces in [WIT](../design/wit.md)
with a standardised Application Binary Interface (ABI).
In the same way that, for example, a Rust package (crate) can be compiled together with other Rust code
to create a higher-level library or an application, a WebAssembly component can be composed with other components.

> Component model interoperation is more convenient and expressive than language-specific foreign function interfaces (FFIs).
> A typical C FFI involves language-specific types, so it is not possible to link between arbitrary languages
> without at least some C-language wrapping or conversion.
> The component model, by contrast, provides a common way of expressing interfaces,
> and a standard binary representation of those interfaces.
> So if an import and an export have the same shape, they fit together directly.

## What is composition?

When you compose components, you wire up the imports of one _primary_ component
to the exports of one or more other _dependency_ components, creating a new component.
The new component, like the original components, is a `.wasm` file, and its interface is defined as follows:

* The new component _exports_ the same exports as the primary component.
* The new component _does not export_ the exports of the dependency components.
* The new component _imports_ all the imports of the dependency components.
* The new component _imports_ any imports of the primary component
  that the dependencies didn't satisfy.
* If several components import the same interface,
  the new component imports that interface—it doesn't "remember"
  that the import was declared in several different places.

For example, consider two components with the following worlds:

```wit
{{#include ../../examples/composing-section-examples/validator.wit}}
```

```wit
{{#include ../../examples/composing-section-examples/regex.wit}}
```

In this example, `validator` is the primary component
and `regex` is a dependency component.
If we compose `validator` with `regex`, `validator`'s import of `docs:regex/match@0.1.0`
is wired up to `regex`'s export of `match`.
The net result is that the composed component exports `docs:validator/validator@0.1.0` and has no imports.
The composed component does _not_ export `docs:regex/match@0.1.0`: that has become an internal implementation detail of the composed component.

Component composition tools are in their early stages right now.
Here are some tips to avoid or diagnose errors:

* Composition happens at the level of interfaces.
  If the initial component directly imports functions, then composition will fail.
  If composition reports an error such as
  "component `path/to/component` has a non-instance import named `<name>`",
  then check that all imports and exports are defined by interfaces.
* Composition is asymmetrical. It is not just "gluing components together"—it takes a primary component
  that has imports, and satisfies its imports using dependency components.
  For example, composing an implementation of `validator` with an implementation of `regex` makes sense
  because `validator` has a dependency that `regex` can satisfy; doing it the other way around doesn't work,
  because `regex` doesn't have any dependencies, let alone ones that `validator` can satisfy.
* Composition cares about interface versions, and current tools are inconsistent about
  when they infer or inject versions.
  If another Rust component _imports_ an interface from `test:mypackage`, that won't match `test:mypackage@0.1.0`.
  You can use [`wasm-tools component wit`](https://github.com/bytecodealliance/wasm-tools/tree/main/crates/wit-component)
  to view the imports and exports embedded in the `.wasm` files and check whether they match up.

## Composing components with WAC

You can use the [WebAssembly Compositions (WAC)](https://github.com/bytecodealliance/wac) CLI
to compose components at the command line.

To perform quick and simple compositions, use the `wac plug` command.
`wac plug` satisfies the import of a "socket" component by plugging a "plug" component's export into the socket.
The socket component is the primary component, while the plug components are dependency components.
For example, a component that implements the [`validator` world above](#what-is-composition)
needs to satisfy its `match` import. It is a socket.
On the other hand, a component that implements the `regex` world exports the `match` interface,
and can be used as a plug.
`wac plug` can plug a regex component's export into the validator component's import,
creating a resultant composition:

```sh
wac plug validator-component.wasm --plug regex-component.wasm -o composed.wasm
```

A component can also be composed with more than component that it depends on.

```sh
wac plug path/to/component.wasm --plug path/to/dep1.wasm --plug path/to/dep2.wasm -o composed.wasm
```

Here `component.wasm` is the component that imports interfaces from `dep1.wasm` and `dep2.wasm`,
which export interfaces.
The composed component, with those dependencies satisfied and tucked away inside it, is saved to `composed.wasm`.

The `plug` syntax doesn't cover transitive dependencies.
If, for example, `dep1.wasm` has unsatisfied imports that you want to satisfy from `dep3.wasm`,
you'd need to be deliberate about the order of your composition.
You could compose `dep1.wasm` with `dep3.wasm` first, then refer to that composed component instead of `dep1.wasm`.
However, this doesn't scale to lots of transitive dependencies, which is why the WAC language was created.

### Advanced composition with the WAC language

`wac plug` is a convenience to achieve a common pattern in component compositions like the ones above.
However, composition can be arbitrarily complicated.
In cases where `wac plug` is not sufficient, the [WAC language](https://github.com/bytecodealliance/wac/blob/main/LANGUAGE.md)
gives us the ability to create arbitrarily complex compositions.

In a WAC file, you use the WAC language to describe a composition.
For example, the following is a WAC file that could be used to create the validator component from [earlier](#what-is-composition).

```
{{#include ../../examples/composing-section-examples/composition.wac}}
```

Then, `wac compose` can be used to compose the components, using the `--dep` flag to specify
the relationships between component names and `.wasm` files:

```sh
wac compose --dep docs:regex-impl=regex-component.wasm \
            --dep docs:validator-impl=validator-component.wasm \
            -o composed.wasm \
            composition.wac
```

Alternatively, you can place the components in a `deps` directory with an expected structure,
and in the near future, you will be able to pull in components from registries.
See the [`wac` documentation](https://github.com/bytecodealliance/wac) for more details.

For an in-depth description about how to use the `wac` tool,
you can check out the [WAC language index](https://github.com/bytecodealliance/wac/blob/main/LANGUAGE.md)
and [examples](https://github.com/bytecodealliance/wac/tree/main/examples).

## Composing components with a visual interface

You can compose components visually using the builder app at [wasmbuilder.app](https://wasmbuilder.app/).

1. Use the "Add Component" button to upload the `.wasm` component files you want to compose.
   The components appear in the sidebar.

2. Drag the components onto the canvas.
   You'll see imports listed on the left of each component, and exports on the right.

3. Click the box in the top left to choose the 'primary' component, that is,
   the one whose exports will be preserved.
   (The clickable area is quite small—wait for the cursor to change from a hand to a pointer.)

4. To fulfil one of the primary component's imports with a dependency's export,
   drag from the "I" icon next to the export to the "I" item next to the import.
   (Again, the clickable area is quite small—wait for the cursor to change from a hand to a cross.)

5. When you have connected all the imports and exports that you want,
   click the Download Component button to download the composed component as a `.wasm` file.

[!NOTE]: #
