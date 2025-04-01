# Distributing and Fetching Components and WIT

Modern applications rely extensively on third-party packages - so extensively that distributing packages is almost an industry in itself. Traditionally, these have been specific to a language. For example, JavaScript developers are used to using packages from NPM, and Rust developers use `crates.io`. Some runtimes support binary distribution and linking, enabling limited cross-language interop; for example, Maven packages can be written in any language that targets the Java runtime. Services like this are variously referred to as "package managers" or "registries."

Publishing and distribution are not defined by the core component model, but form important part of the component ecosystem. For example, if you're writing JavaScript, and want to pull in a highly optimised machine learning algorithm written in C and compiled to Wasm, you can pull it from a registry, ideally just as easily as you would add a NPM package from the NPM registry.

You can get involved with improving the packaging and hosting of Wasm components by joining the [Bytecode Alliance Packaging Special Interest Group (SIG)](https://github.com/bytecodealliance/governance/blob/main/SIGs/sig-packaging/proposal.md).

## The `wkg` Registry Tool

The [`wasm-pkg-tools` project](https://github.com/bytecodealliance/wasm-pkg-tools) enables fetching and publishing Wasm components to OCI registries. It contains a `wkg` CLI tool that eases distributing and fetching components and WIT packages. `wkg` contains several subcommand:

- `wkg oci` - is a CLI wrapper around the [oci-wasm](https://github.com/bytecodealliance/rust-oci-wasm) crate which enables pushing/pulling Wasm artifacts to/from any OCI registry
- `wkg publish` - pushes *library* components or WIT packages
- `wkg get` - pulls *library* components or WIT packages
- `wkg wit` - commands for interacting with WIT files and dependencies
- `wkg config` - interact with the `wkg` configuration

The following sections detail a subset of actions that can be performed with `wkg`.

## `wkg` Configuration Files

The `wkg` tool uses a configuration file to understand where to publish and fetch specific packages to and from. It provides the ability to configure:

- a default registry for all packages at the top level of the file
- a default registry for all packages of a specific namespace under `[namespace_registries]`. This section can be used to configure the registry of all `wasi` namespaced packages, such as `wasi:clocks` and `wasi:http`.
- an override for package of a specific namespace under `[package_registry_overrides]`. This section can be used to fetch/publish a specific package of a namespace from/to a different location than all other packages of that namespace. For example, maybe you want to fetch `wasi:http` from a different registry.
- credentials for a registry under `[registry."<registry-name>".oci]`
- and more! See the [`wkg` docs for more configuration options](https://github.com/bytecodealliance/wasm-pkg-tools?tab=readme-ov-file#configuration).

For example, to fetch WASI packages, such as `wasi:clocks` and `wasi:http`, a line can be added under the `namespace_registries` section for the `wasi` namespace. Specifically, the example below configures `wkg` to fetch WASI packages from the [WebAssembly OCI GitHub Container Registry](https://github.com/orgs/WebAssembly/packages), where the latest interfaces are published upon WASI releases. To edit your `wkg` config file, simply run `wkg config --edit`.

> Remember, all package names consist of the a namespace field followed by the package field. The package name `wasi:clocks` has a namespace of `wasi` and package field of `clocks`. In this way, the following configuration ensures that `wkg` will know to route fetches and publishes of any `wasi:x` to the configured location.

```toml
# $XDG_CONFIG_HOME/wasm-pkg/config.toml
default_registry = "ghcr.io"

[namespace_registries]
# Instruct wkg to use the OCI protocol to fetch packages with the `wasi` namespace from ghcr.io/webassembly
wasi = { registry = "wasi",  metadata = { preferredProtocol = "oci", "oci" = {registry = "ghcr.io", namespacePrefix = "webassembly/" } } }
```

As a more generic example, The following configuration, instructs `wkg` to use [ttl.sh](https://ttl.sh/) OCI registry for all packages with the `docs` namespace.

```toml
# $XDG_CONFIG_HOME/wasm-pkg/config.toml
default_registry = "ghcr.io"

[namespace_registries]
# Instruct wkg to use the OCI protocol to fetch packages with the `foo` namespace from ttl.sh/wasm-components
docs = { registry = "docs-registry",  metadata = { preferredProtocol = "oci", "oci" = {registry = "ttl.sh", namespacePrefix = "wasm-components/" } } }
```

> Note: the registry name can be referenced in the `package_registry_overrides` section of the `wkg` config to provide overrides for specific packages of a namespace.


## Distributing WIT and Library Components using  `wkg publish`

Once you've [configured `wkg`](#wkg-configuration-files) to know where to publish packages to, you can use the `wkg publish` command to publish library *components* or *interfaces* to be consumed by others.

Imagine you have defined the following `adder` world in WIT:

```wit
package docs:adder@0.1.0;

interface add {
    add: func(a: u32, b: u32) -> u32;
}

world adder {
    export add;
}
```

You can publish this *WIT* using `wkg` by wrapping it up as a Wasm component. Yes, you heard that right! We are packaging WIT as Wasm.

```sh
# Package the contents of add WIT directory as Wasm
wkg wit build --wit-dir tutorial/wit/adder
# Publish the produced component
wkg publish docs:adder@0.1.0.wasm
```

If you had configured `wkg` as described in the [`wkg` configuration section](#wkg-configuration-files), this would publish the component to `ttl.sh/wasm-components/docs/adder:0.1.0`. This WIT can then be fetched using `wkg get`, specifying the format `wit`:

```sh
wkg get --format wit docs:adder@0.1.0
```

Instead of publishing the WIT interface, you could publish the built component by running:

```sh
wkg publish adder.wasm --package docs:adder@0.1.0
```

This component can then be fetched by running:

```sh
wkg get docs:adder@0.1.0
```

## More Generic Operations with `wkg oci`

The `wkg oci` subcommand is a CLI wrapper around the [oci-wasm](https://github.com/bytecodealliance/rust-oci-wasm) crate which enables pushing/pulling Wasm artifacts to/from any OCI registry. Unlike `wkg publish` and `wkg get` it is not limited to library components, as providing the WIT package is not required.

A component is pushed to an OCI registry using `wkg oci pull`. The example below pulls a component from a GitHub Container Registry.

```sh
wkg oci push ghcr.io/user/component:0.1.0 component.wasm
```

To pull a component, run:

```sh
wkg oci pull ghcr.io/user/component:0.1.0 -o component.wasm
```

## Fetching WIT Package Dependencies using `wkg`

Sometimes fetching a single package is not sufficient because it depends on other packages. For example, the following world describes a simple Wasm service which requires `wasi:http/proxy`:

```wit
package foo:wasi-http-service;

world target-world {
  include wasi:http/proxy@0.2.3;
}
```

You may be tempted to simply get the `wasi:http` package with `wkg get  --format wit wasi:http@0.2.3 -o wit/deps/http/`. However, `wasi:http` depends on other WASI packages such as `wasi:clocks` and `wasi:io`. To make sure to fetch a package and all its dependencies, use `wkg wit fetch`, which will read the package containing the world(s) you have defined in the given wit directory (`wit` by default). It will then fetch the
dependencies and write them to the `deps` directory along with a lock file.

After placing the above file in `./wit`, run the following to fetch the dependencies:

```sh
wkg wit fetch
```

The `./wit` directory will be populated as follows:
```sh
wit
├── deps
│   ├── wasi-cli-0.2.3
│   │   └── package.wit
│   ├── wasi-clocks-0.2.3
│   │   └── package.wit
│   ├── wasi-http-0.2.3
│   │   └── package.wit
│   ├── wasi-io-0.2.3
│   │   └── package.wit
│   └── wasi-random-0.2.3
│       └── package.wit
└── world.wit
```

Now, you can use the language toolchain of your choice to generate bindings and create your component.