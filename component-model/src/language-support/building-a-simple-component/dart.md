# Dart Tooling

WebAssembly components in Dart can be built with the [wasm_tools package](https://github.com/simolus3/wasm.dart/) on pub.dev.

This guide walks through building a Dart component that implements
an `adder` world:

```wit
{{#include ../../../examples/tutorial/wit/adder/world.wit}}
```

Keep in mind that this is a basic intro. For more examples, please see [examples]
from the `wasm_tools` package or [Creating Runnable Components].

If you still have questions, feel free to open an issue on [the repository][wasm_tools]
or reach out on [Zulip][chat].

## 1. Create your Dart project

Begin by creating a fresh Dart project:

```sh
dart create -t cli dart_wasm_adder
cd dart_wasm_adder
```

## 2. Install Dart Component Model tooling

> [!WARNING]
> Compiling Dart to non-web WebAssembly targets is experimental.
> This guide requires Dart version `3.14.0` or later.

All tools required to create WebAssembly components from Dart can be installed via `dart pub`:

```sh
dart pub add wasm_components dev:wasm_tools
```

The `wasm_components` package provides runtime support for the Component Model
(like an allocator or async task management), while `wasm_tools` contains tools
to generate Dart code from `.wit` interface definitions and to compile Dart to
components.

> [!TIP]
> The `wasm_tools` Dart package is not affiliated with the
> [`wasm-tools` Bytecode Alliance project][bca-wasm-tools].
> You can install both without conflict, as the Dart package is always invoked
> through `dart run wasm_tools`.

## 3. Generate bindings for the Wasm component

Since we will be implementing the [`adder` world][docs-adder], we can copy the WIT to our project.
Create a file named `adder.wit` and paste the following code into it:

```wit
{{#include ../../../examples/tutorial/wit/adder/world.wit}}
```

Generate Dart code and required metadata for the compiler with

```sh
dart run wasm_tools witgen -i adder.wit
```

After running the generator, the project structure should look like this:

```
.
├── adder.wit
├── analysis_options.yaml
├── bin
│   └── dart_wasm_adder.dart
├── CHANGELOG.md
├── lib
│   └── src
│       └── components
│           ├── docs_adder_adder.dart
│           ├── docs_adder_adder.json
│           └── docs_adder.dart
├── pubspec.lock
├── pubspec.yaml
└── README.md
```

All generated code is in `lib/src/components`. Specifically,

- `docs_adder.dart` contains interfaces (only `add` in this example).
- `docs_adder_adder.dart` contains bindings for the `adder` world.
- `docs_adder_adder.json` contains metadata used to turn WebAssembly
  modules emitted by `dart2wasm` into components.

The JSON file describes which component imports and exports a Dart program needs.
The toolchain reads it via [link hooks].

All packages defining component imports or exports need a link hook,
including the `dart_wasm_adder` package. Create a `hook/link.dart` file with
these contents:

```dart
import 'dart:convert';
import 'dart:io';

import 'package:hooks/hooks.dart';
import 'package:wasm_tools/hooks.dart';

void main(List<String> args) => link(args, (input, output) async {
  if (input.config.buildWasmComponent) {
    final abi = input.packageRoot.resolve('lib/src/components/docs_adder_adder.json');

    output.dependencies.add(abi);
    output.assets.webAssemblyComponents.add(
      WasmComponentAsset(
        encoded: json.decode(
          File(abi.toFilePath()).readAsStringSync(),
        ) as Map<String, Object?>,
      ),
    );
  }
});
```

## 4. Implement the `add` function

Part of the `docs_adder_adder.dart` file generated with `dart run wasm_tools witgen`
is the `adderComponent` function used to define this component:
It receives imported interfaces as parameters (in this case, there aren't any)
and returns the exported interface.

For this example, replace `bin/dart_wasm_adder.dart` with:

```dart
import 'package:dart_wasm_adder/src/components/docs_adder.dart';
import 'package:dart_wasm_adder/src/components/docs_adder_adder.dart';

import 'package:wasm_components/wasm_components.dart';

void main() {
  // For components, main() should only contain a single call to a generated
  // component function.
  adderComponent((_) {
    // This callback returns exported interfaces (only Add in this case). It
    // also receives imports through its parameter, in this example the adder
    // world has none.
    return const _Add();
  });
}

final class const _Add() implements Add {
  @override
  int add({required int x, required int y}) {
    return x + y;
  }
}
```

## 5. Running the `add` component

With all generated bindings set up, the component can be compiled:

```sh
dart run wasm_tools compile bin/dart_wasm_adder.dart
```

This creates `bin/dart_wasm_adder.wasm`, a component we can invoke from the
CLI with [wasmtime]:

```console
$ wasmtime run --invoke 'add(1, 2)' bin/dart_wasm_adder.wasm
3
```

With this, we have successfully built and run a basic WebAssembly component with Dart 🎉

[wasm_tools]: https://github.com/simolus3/wasm.dart/
[examples]: https://github.com/simolus3/wasm.dart/tree/main/pkg/wasm_tools/example
[Creating Runnable Components]: ../creating-runnable-components/dart.md
[bca-wasm-tools]: https://github.com/bytecodealliance/wasm-tools
[docs-adder]: https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/tutorial/wit/adder/world.wit
[chat]: https://bytecodealliance.zulipchat.com/#narrow/channel/394175-SIG-Guest-Languages/topic/Dart.20subgroup/with/614175797
[link hooks]: https://dart.dev/tools/hooks
[wasmtime]: https://wasmtime.dev/
