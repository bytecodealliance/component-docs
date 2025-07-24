# Components

Conceptually, a component is a self-describing WebAssembly binary
that interacts only through interfaces
instead of shared memory.
Let's break down what each of these terms means:

* _Self-describing_: Like a WebAssembly core module,
  a component includes import and export declarations
  that declare both the names and types of
  imported and exported functions.
  Compared to core modules, components use a richer type system
  to describe these types, so it's easier to understand
  what functionality a module provides
  and what functionality it relies on.
* _Interacts_: When a component interacts with other components,
  that means either that it calls a function defined in a different component,
  or that another component calls a function defined in it.
  Interfaces specify what kinds of function calls are valid.
* _Shared memory_: In the ["Why the Component Model?"](./why-component-model.md) section,
  we showed how WebAssembly core modules can only exchange compound data
  through shared memory.
  Components use memory in the same way that core modules do,
  except that in components, memories are never exported or imported;
  they are not shared.

Logically, a component is a structure
that may contain core modules and/or other components.
The component encodes the interfaces of these contained
modules and sub-components using [WebAssembly Interface Types (WIT)](./wit.md).

The on-disk representation of a component
is a specially-formatted WebAssembly file.
Internally, this file could include representations
of one or many traditional ("core") WebAssembly modules
and sub-components,
composed together via their imports and exports.
Two modules or components can be composed if the
imports of one are satisfied by the exports of another.
Composition can be repeated arbitarily, composing a
single component out of many interlocking modules and components.
[Interfaces](./interfaces.md) enable checking that
a particular composition makes sense.

Each component is described by a [world](./worlds.md),
which potentially collects together multiple interfaces
to describe all the imports and exports of the component.
The world only describes the types of imported and exported functions;
the component internally defines the code to implement the world.

> For a more formal definition of a component,
> take a look at the [Component Model specification](https://github.com/WebAssembly/component-model).
