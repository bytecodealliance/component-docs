# Components

Physically, a **component** is a specially-formatted WebAssembly file. Internally, the component could include multiple traditional ("core") WebAssembly modules, and sub-components, composed via their imports and exports.

The external interface of a component - its imports and exports - corresponds to a world. The component, however, internally defines how that world is implemented.
