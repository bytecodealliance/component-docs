# WIT Worlds

A **WIT world** is a higher-level contract that describes a component's capabilities and needs.

On one hand, a world describes the shape of a component - it says which interfaces the component exposes for other code to call (its exports), and which interfaces the component depends on (its imports). A world only defines the surface of a component, not the internal behaviour. If you're an application or library developer creating a component, you'll specify the world your component targets. This world describes the functionality your component exposes and declares the functionality your component depends on in order to be able to run. Your component may target a custom world definition you have created with a unique set of imports and exports tailored just for your use case, or it may target an existing world definition that someone else has already specified.

On the other hand though, a world defines a _hosting environment_ for components (i.e., an environment in which a component can be instantiated and its functionality can be invoked). An environment supports a world by providing implementations for all of the imports and by optionally invoking one or more of the exports.

For example, WASI (the WebAssembly System Interface) defines a "command line" world which imports interfaces that command line programs typically expect to have available to them such as file I/O, random number generation, clocks and so on. This world has a single export for running the command line tool. Components targeting this world must provide an implementation for this single export, and they may optionally call any of the imports. On the other hand, environments supporting this world must provide implementations for all of the imports and may invoke the single export.

A world is composed of interfaces, but each interface is _directional_ - it indicates whether the interface is available for outside code to call (an "export"), or whether outside code must fulfill the interface for the component to call (an "import"). These interfaces strictly bound the component. A component cannot interact with anything outside itself except by having its exports called, or by it calling its imports. This provides very strong sandboxing; for example, if a component does not have an import for a secret store, then it _cannot_ access that secret store, even if the store is running in the same process.

For a component to run, its imports must be fulfilled, by a host or by other components. Connecting up some or all of a component's imports to other components' matching exports is called _composition_.

## Example Worlds

* A (trivial) "HTTP proxy" world would export a "handle HTTP requests" interface, and import a "send HTTP requests" interface. A host, or another component, would call the exported "handle" interface, passing an HTTP request; the component would forward it on via the imported "send" interface. To be a _useful_ proxy, the component may also need to import interfaces such as I/O and clock time - without those imports the component could not perform, for example, on-disk caching.
* A "regex parser" world would export a "parse regex" function, and would import nothing. This declares not only that the component implementing this world can parse regular expressions, but also that it calls no other APIs. A user of such a parser could know, without looking at the implementation, that is does not access the file system, or send the user's regexes to a network service.

> For a more formal definition of what a WIT world is, take a look at the [WIT world specification](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md#wit-worlds).
