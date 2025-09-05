# Distributing and Fetching Components and WIT

Modern applications rely extensively on third-party packages—so extensively
that distributing packages is almost an industry in itself.
Traditionally, package distribution services have been specific to a single programming language.
For example, JavaScript developers are used to using packages from NPM,
and Rust developers use `crates.io`.
Some runtimes support binary distribution and linking, enabling limited cross-language interoperability:
for example, Maven packages can be written in any language that targets the Java runtime.
Services like this are variously referred to as "package managers" or "registries."

Publishing and distribution are not defined by the core component model,
but they form an important part of the component ecosystem.
For example, if you're writing JavaScript, and want to pull in a highly optimised machine learning algorithm
written in C and compiled to Wasm, you can pull it from a registry,
ideally just as easily as you would add an NPM package from the NPM registry.

You can get involved with improving the packaging and hosting of WebAssembly components
by joining the [Bytecode Alliance Packaging Special Interest Group (SIG)](https://github.com/bytecodealliance/governance/blob/main/SIGs/sig-packaging/proposal.md).

## The `wkg` Registry Tool

The [`wasm-pkg-tools` project](https://github.com/bytecodealliance/wasm-pkg-tools)
enables fetching and publishing WebAssembly components to
[Open Container Initiative](https://opencontainers.org/) (OCI) registries.

`wasm-pkg-tools` contains a `wkg` CLI tool that eases distributing and fetching components and WIT packages.
The usual way of using `wkg` is to address packages by their names: for example, `example:adder@1.0.0`.
When using `wkg` this way, you don't need to know about the physical location of the package,
as the `wkg` configuration handles that for you.
If you need to, though, you can also use `wkg` to work with OCI artifacts directly,
addressing them by OCI references when using the `wkg oci` subcommand.

`wkg` contains several subcommand:

- `wkg oci`: pushes/pulls WebAssembly artifacts to/from any OCI registry
- `wkg publish`: publishes components or WIT packages by package name
- `wkg get`: pulls components or WIT packages by package name
- `wkg wit`: interacts with WIT files and dependencies
- `wkg config`: interacts with the `wkg` configuration

The following sections detail a subset of actions that can be performed with `wkg`.

## `wkg` Configuration Files

When you use most `wkg` commands (`wkg oci` being the exception),
you don't interact with physical locations, only with package names.
The `wkg` configuration file is used to map package names to physical locations.
It provides the ability to configure:

- The default registry for packages in a given namespace: for example,
  the location for `wasi` packages such as `wasi:clocks` or `wasi:http`.
- Registry overrides for specific packages that are not stored in the same place as the rest of their namespaces.
  For example, an override would be used if `wasi:key-value` were stored in a different registry
  from other `wasi` packages.
- The default registry for all packages not listed in one of the previous sections.

The configuration file also includes credentials for private registries,
or for pushing to registries where you have permission, and other configuration options.
See the [`wkg` docs for more configuration options](https://github.com/bytecodealliance/wasm-pkg-tools?tab=readme-ov-file#configuration).

For example, to fetch WASI packages, such as `wasi:clocks` and `wasi:http`,
you can add a line under the `namespace_registries` section for the `wasi` namespace.
Specifically, the example below configures `wkg` to fetch WASI packages from the [WebAssembly OCI GitHub Container Registry](https://github.com/orgs/WebAssembly/packages),
where the latest interfaces are published upon WASI releases.
To edit your `wkg` config file, run `wkg config --edit`.

> Remember, all package names consist of a namespace field followed by a package field.
> For example, the package name `wasi:clocks` has a namespace field of `wasi` and package field of `clocks`.
> In this way, the following configuration ensures that `wkg` will know to route fetches and publishes
> of any `wasi:<package field>` to the configured location.

```toml
# $XDG_CONFIG_HOME/wasm-pkg/config.toml
default_registry = "ghcr.io"

[namespace_registries]
# Tell wkg that packages with the `wasi` namespace are in an OCI registry
# under ghcr.io/webassembly
wasi = { registry = "wasi",  metadata = { preferredProtocol = "oci", "oci" = {registry = "ghcr.io", namespacePrefix = "webassembly/" } } }
```

As a more generic example, the following configuration instructs `wkg` to use
the [ttl.sh](https://ttl.sh/) OCI registry for all packages with the `docs` namespace.

```toml
# $XDG_CONFIG_HOME/wasm-pkg/config.toml
default_registry = "ghcr.io"

[namespace_registries]
# Instruct wkg to use the OCI protocol to fetch packages with the `docs` namespace from ttl.sh/wasm-components
docs = { registry = "docs",  metadata = { preferredProtocol = "oci", "oci" = {registry = "ttl.sh", namespacePrefix = "wasm-components/" } } }
```

> Note: the registry name can be referenced in the `package_registry_overrides` section of the `wkg` config
> to provide overrides for specific packages of a namespace.

## Distributing WIT and Components by Package Name with `wkg publish`

Once you've [configured `wkg`](#wkg-configuration-files) to specify where to publish packages to,
you can use the `wkg publish` command to publish *components* or *interfaces* to be consumed by others.

Imagine you have defined the following `adder` world in WIT:

```wit
{{#include ../../examples/tutorial/wit/adder/world.wit}}
```

You can publish this *WIT* using `wkg` by wrapping it up as a Wasm component.
Yes, you heard that right! We are packaging WIT as Wasm.
If you've saved this world file in a directory called `tutorial/wit/adder`,
you can execute:

```sh
# Package the contents of add WIT directory as Wasm
wkg wit build --wit-dir tutorial/wit/adder
# Publish the produced component
wkg publish docs:adder@0.1.0.wasm
```

If you had configured `wkg` as described in the [`wkg` configuration section](#wkg-configuration-files),
this would publish the component to `ttl.sh/wasm-components/docs/adder:0.1.0`.
This WIT can then be fetched using `wkg get`, specifying the format `wit`:

```sh
wkg get --format wit docs:adder@0.1.0 --output adder.wit
```

Instead of publishing the WIT interface, you could publish the built component by running:

```sh
wkg publish adder.wasm --package docs:adder@0.1.0
```

You could then fetch the component by running:

```sh
wkg get docs:adder@0.1.0 --output adder.wasm
```

## More Generic Operations with `wkg oci`

The `wkg oci` subcommand enables pushing and pulling Wasm artifacts to or from any OCI registry.
Unlike with `wkg publish` and `wkg get`, providing the WIT package is not required.

To push a component to an OCI registry, use `wkg oci pull`.
The example below pushes a component to a GitHub Container Registry.

```sh
wkg oci push ghcr.io/user/component:0.1.0 component.wasm
```

To pull a component, run:

```sh
wkg oci pull ghcr.io/user/component:0.1.0 -o component.wasm
```

## Fetching WIT Package Dependencies using `wkg`

Sometimes fetching a single package is not sufficient because it depends on other packages.
For example, the following world describes a simple Wasm service that requires `wasi:http/proxy`:

```wit
{{#include ../../examples/composing-section-examples/http-service.wit}}
```

You may be tempted to simply get the `wasi:http` package with
`wkg get  --format wit wasi:http@0.2.3 -o wit/deps/http/`.
However, `wasi:http` depends on other WASI packages such as `wasi:clocks` and `wasi:io`.
To make sure to fetch a package and all its dependencies, use `wkg wit fetch`,
which will read the package containing the world(s) you have defined in the given WIT directory (`wit` by default).
It will then fetch the dependencies and write them to the `deps` subdirectory along with a lock file.
(The lock file specifies the exact version of each dependency that will be used in your project.)

After saving the above file as `./wit/world.wit`, run the following command to fetch the dependencies:

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
