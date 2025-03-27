# Distributing and Fetching Components and WIT

Modern applications rely extensively on third-party packages - so extensively that distributing packages is almost an industry in itself. Traditionally, these have been specific to a language. For example, JavaScript developers are used to using packages from NPM, and Rust developers use `crates.io`. Some runtimes support binary distribution and linking, enabling limited cross-language interop; for example, Maven packages can be written in any language that targets the Java runtime. Services like this are variously referred to as "package managers" or "registries."

Publishing and distribution are not defined by the core component model, but form important part of the component ecosystem. For example, if you're writing JavaScript, and want to pull in a highly optimised machine learning algorithm written in C and compiled to Wasm, you can pull it from a registry, ideally just as easily as you would add a NPM package from the NPM registry.

You can get involved with improving the packaging and hosting of Wasm components by joining the [Bytecode Alliance Packaging Special Interest Group (SIG)](https://github.com/bytecodealliance/governance/blob/main/SIGs/sig-packaging/proposal.md).

## Registry Tooling

The [`wasm-pkg-tools` project](https://github.com/bytecodealliance/wasm-pkg-tools) enables fetching and publishing Wasm Components to OCI or Warg registries. It contains a `wkg` CLI tool that eases distributing and fetching components and WIT packages. The following examples walk through using `wkg`.

## Distributing Components Using `wkg`

A component is pushed to an OCI registry using `wkg oci push`. The example below pushes to a GHCR:

```sh
wkg oci push ghcr.io/user/hello:0.1.0 hello.wasm
```

## Fetching Components Using `wkg`

A component is pulled from a OCI registry using `wkg oci pull`. The example below pulls a component from GHCR:

```sh
wkg oci pull ghcr.io/user/hello:0.1.0 -o hello.wasm
```

## Configuring `wkg` to Fetch WASI Packages

The `wkg` tool uses a configuration file to store settings with a default location of `$XDG_CONFIG_HOME/wasm-pkg/config.toml`. It must be configured to know which registry to use for which package namespaces. The following is a convenient configuration to ensure you can fetch WASI packages from the [WebAssembly OCI registry](https://github.com/WebAssembly/WASI/pkgs/container/wasi), where the latest interfaces are published upon WASI releases.

```toml
# $XDG_CONFIG_HOME/wasm-pkg/config.toml
default_registry = "ghcr.io"

[namespace_registries]
wasi = { registry = "wasi",  metadata = { preferredProtocol = "oci", "oci" = {registry = "ghcr.io", namespacePrefix = "webassembly/" } } }

[package_registry_overrides]

[registry]
```

## Distributing WIT Packages using  `wkg`

The `wkg` tool uses a [configuration file](https://github.com/bytecodealliance/wasm-pkg-tools?tab=readme-ov-file#configuration) to store settings with a default location of `$XDG_CONFIG_HOME/wasm-pkg/config.toml`. It must be configured to know which registry to use for which package namespaces. The following configuration, instructs `wkg` to use [ttl.sh](https://ttl.sh/) OCI registry for all packages with the `foo` namespace.

```toml
# $XDG_CONFIG_HOME/wasm-pkg/config.toml
default_registry = "ghcr.io"

[namespace_registries]
foo = { registry = "foo-registry",  metadata = { preferredProtocol = "oci", "oci" = {registry = "ttl.sh", namespacePrefix = "wasm-components/" } } }

[package_registry_overrides]

[registry]
```

Now, `foo` packages can be built and published using `wkg`.

```sh
mkdir wit
cat > wit/foo.wit << EOL
package foo:bar@0.1.0;

interface do-something {
    reduce: func(a: u32, b: u32) -> u32;
}

world example {
    export do-something;
}
EOL

wkg wit build
wkg publish foo:bar@0.1.0.wasm
```

This will publish the component to `ttl.sh/wasm-components/foo/bar:0.1.0`

## Configuring `wkg` to Fetch Custom Packages

After configuring `wkg` to know where to pull `foo` namespaced packages from, the `bar` package can be pulled with `wkg get`:

```sh
wkg get --format wit foo:bar@0.1.0
```

## Fetching WIT Package Dependencies using `wkg`

Sometimes fetching a single package is not sufficient because it depends on other packages. For example, the following world describes a simple Wasm service which requires `wasi:http/proxy``:

```wit
package foo:wasi-http-service;

world target-world {
  include wasi:http/proxy@0.2.3;
}
```

One may be tempted to simply get the `wasi:http` package with `wkg get  --format wit wasi:http@0.2.3 -o wit/deps/http/`. However, `wasi:http` depends on other WASI packages such as `wasi:clocks` and `wasi:io`. To make sure to fetch a package and all its dependencies, use `wkg fetch`, which will read the package containing the world(s) you have defined in the given wit directory (`wit` by default). It will then fetch the
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