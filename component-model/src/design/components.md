# Components

* Logically, components are containers for modules - or other components - which express their [interfaces](./interfaces.md) and dependencies via [WIT](./wit.md) and the [Canonical ABI](./canonical-abi.md).
* Conceptually, components are self-describing units of code that interact only through interfaces instead of shared memory.
* Physically, a **component** is a specially-formatted WebAssembly file. Internally, the component could include multiple traditional ("core") WebAssembly modules, and sub-components, composed via their imports and exports.

The external interface of a component - its imports and exports - corresponds to a [world](./worlds.md). The component, however, internally defines how that world is implemented.

> ⓘ For a more formal definition of what a component is, take a look at the [Component Model specification](https://github.com/WebAssembly/component-model).
