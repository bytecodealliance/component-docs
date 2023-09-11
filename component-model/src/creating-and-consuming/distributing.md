# Distributing Components

Modern applications rely extensively on third-party packages - so extensively that distributing packages is almost an industry in itself. Traditionally, these have been specific to a language. For example, JavaScript developers are used to using packages from NPM, and Rust developers use `crates.io`. Some runtimes support binary distribution and linking, enabling limited cross-language interop; for example, Maven packages can be written in any language that targets the Java runtime. Services like this are variously referred to as "package managers" or "registries."

Publishing and distribution are not defined by the core component model, but will form an important part of the component ecosystem. For example, if you're writing JavaScript, and want to pull in a highly optimised machine learning algorithm written in C and compiled to Wasm, you should be able to invoke it from a registry, just as easily as you would add a NPM package from the NPM registry.

Publishing and distribution is a work in progress. The proposed registry protocol is [warg](https://warg.io/), but this is still in development, and there are no public warg registries as yet. You can find more information about the development of the registry protocol [here](https://github.com/bytecodealliance/governance/blob/main/SIGs/SIG-Registries/proposal.md).
