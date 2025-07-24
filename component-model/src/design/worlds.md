# WIT Worlds

A **WIT world** (or just "world") is a contract with a broader scope
than a single interface.
A world describes the functionality a component provides,
and the functionality it requires in order to work.

A world can be used to describe a component,
and a hosting environment for other components, 
depending on which imports and exports are specified.
This is because components can be composed: a component
can provide functionality that other components can depend on.

On the one hand, a world describes how a component relates to other components:
it describes the functionality the component exposes
and declares the functionality it depends on in order to be able to run.
Functionality is exposed by defining interfaces to export,
and dependencies are declared by importing interfaces.
A world only defines the surface of a component, not its internal behaviour.

On the other hand, a world defines a hosting environment for components:
that is, an environment in which a component can be instantiated
and its functionality can be invoked.
* In WebAssembly, _instantiation_ means turning a static description of a module
  into a dynamic structure in memory.
  It's analogous to [loading](https://en.wikipedia.org/wiki/Loader_(computing))
  an executable file.

A hosting environment supports a world by providing implementations
for all of the imports
and by optionally invoking one or more of the exports.
If you're an application or library developer creating a component,
you'll specify the world your component targets.
Your component may target a custom world definition you have created
with a unique set of imports and exports tailored just for your use case,
or it may target an existing world definition that someone else has already specified.
In either case, the world specifies all the external functionality your component needs.
Targeting a world is analogous to relying on a particular version of a standard library,
except that components give you the ability to precisely specify
exactly what functions your code depends on.

For example, WASI (the WebAssembly System Interface) defines a "command line" world
that imports interfaces that command-line programs typically expect to have available to them:
for example, file input/output, random number generation, and clocks.
This world has a single export for running the command-line tool.
Components targeting this world must provide an implementation for this single export,
and they may optionally call any of the imports.
For example, a component that prints out a summary of the sizes of files
in a particular directory (like the Unix `du` command)
could target the "command line" world, and would depend on
the file input/output interfaces imported by the world.
A hosting environment that supports this world
must provide implementations for all of the imports
and may invoke the single export.
Running your example disk usage component
would mean invoking it in a hosting environment
that supports the "command line" world.

A world is a collection of interfaces, where each interface is _directional_.
Each interface is explicitly labeled as either an export or an import.
Exported interfaces are available for outside code to call,
whereas imported interfaces must be fulfilled by outside code.
These interfaces define a strict boundary for a component.
The only ways a component can interact with anything outside itself
are by having its exports called,
or by calling its imports.
This boundary provides very strong sandboxing:
for example, if a component does not have an import for a secret store,
then it _cannot_ access that secret store,
even if the store is running in the same process.

For a component to run, its imports must be fulfilled, by a host or by other components.
Connecting up some or all of a component's imports to other components' matching exports is called _composition_.

A world is defined in a WIT file; a single WIT files can contain multiple worlds.

## Example Worlds

* A (trivial) "HTTP proxy" world would export a "handle HTTP requests" interface
and import a "send HTTP requests" interface.
A host, or another component, would call the exported "handle" interface, passing an HTTP request;
the component would forward it on via the imported "send" interface.
To be a _useful_ proxy, the component may also need to import interfaces such as I/O and clock time:
without those imports the component could not perform on-disk caching or other needed features.
* A "regex parser" world would export a "parse regex" function, and would import nothing.
This declares not only that the component implementing this world can parse regular expressions,
but also that it calls no other APIs.
A user of such a parser could know, without looking at the implementation,
that it does not access the file system or send the user's regexes to a network service.

> For a more formal definition of what a WIT world is, take a look at the [WIT world specification](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md#wit-worlds).
