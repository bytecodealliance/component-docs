# Dart Tooling

> [!WARNING]
> Compiling Dart to non-web WebAssembly targets is experimental.
> This guide requires Dart version `3.14.0-251.0.dev` or later.

WebAssembly components in Dart can be built with the [wasm_tools package](https://github.com/simolus3/wasm.dart/) on pub.dev.

This guide walks through building a Dart component that implements
the `adder` world defined in the [`adder/world.wit` package][docs-adder].
The component will implement the `adder` world, which contains an `add` interface with an `add` function.

Keep in mind that this is a basic intro. For more examples, please see the [dart-samples][examples]
from the `wasm_tools` package or TODO: running a component.

If you still have questions, feel free to open an issue on [the repository][wasm_tools]
or reach out on [Zulip][chat].

## 1. Create your Dart project

Begin by creating a fresh Dart project:

```sh
dart create -t cli dart_wasm_adder
cd dart_wasm_adder
```

## 2. Install the tools

All tools requires to create WebAssembly components from Dart can be installed via pub:

```sh
dart pub add wasm_components dev:wasm_tools
```

The `wasm_components` package provides runtime support for component models
(like an allocator or async task management), while `wasm_tools` contains `witgen`
and tools to compile Dart to components.

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

This generates:

- `lib/src/components/docs_adder.dart` containing interfaces (only `add` in this example).
- `lib/src/components/docs_adder_adder.dart` containing bindings for the `adder` world.
- `lib/src/components/docs_adder_adder.json` containing metadata used to turn WebAssembly
  modules emitted by `dart2wasm` into components.

The JSON file describe which component imports and exports a Dart program needs.
The toolchain reads it via [link hooks]. All packages defining component imports
or exports need one, including the `dart_wasm_adder` package. Create a
`hook/link.dart` file with these contents:

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

The generated `adderComponent` function is used to define this component:
It receives imported interfaces as parameters (in this case, there aren't any)
and returns the exported interface.

For this example, replace `bin/dart_wasm_adder.dart` with:

```dart
import 'package:dart_wasm_adder/src/components/docs_adder.dart';
import 'package:dart_wasm_adder/src/components/docs_adder_adder.dart';

import 'package:wasm_components/wasm_components.dart';

void main() {
  adderComponent((_) => const _Add());
}

final class const _Add() implements Add {
  @override
  int add({required int x, required int y}) {
    return x + y;
  }
}
```

## 5. Testing the `add` component

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
[docs-adder]: https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/tutorial/wit/adder/world.wit
[chat]: https://bytecodealliance.zulipchat.com/#narrow/channel/394175-SIG-Guest-Languages/topic/Dart.20subgroup/with/614175797
[link hooks]: https://dart.dev/tools/hooks
[wasmtime]: https://wasmtime.dev/
