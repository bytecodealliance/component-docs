# Creating Runnable Components (Dart)

> [!WARNING]
> Compiling Dart to non-web WebAssembly targets is experimental.
> This guide requires Dart version `3.14.0-251.0.dev` or later.

## Creating a command component

A _command_ is a component with a specific export that allows it to be executed directly by `wasmtime`
(or other `wasi:cli` hosts). Ignoring the specifics of defining components in Dart, this is the
equivalent of running a plain Dart program with a `main()` function.

### 1. Create a new Dart project

To create a command with Dart, start with a fresh Dart package:

```sh
dart create -t cli dart_wasm_cli
```

To configure this package for WebAssembly components, add tooling dependencies.
Additionally, the `wasi` package provides generated bindings to WASI definitions,
meaning that running `dart run wasm_tools witgen` won't be necessary.

```sh
dart pub add wasi wasm_components dev:wasm_tools
```

### 2. Write the relevant Dart

The following code can be inserted into `bin/dart_wasm_cli.dart`:

```dart
import 'dart:async';
import 'dart:convert';
import 'dart:typed_data';

import 'package:wasi/cli/command.dart';
import 'package:wasi/cli.dart';
import 'package:wasm_components/wasm_components.dart';

void main() {
  commandComponent((imports) => _Run(imports.cliStdout));
}

final class _Run(final Stdout stdout) implements Run {
  @override
  Future<Result<void, void>> run() async {
    final out = StreamController<Uint8List>();
    final stdoutDone = stdout.writeViaStream(data: out.stream);

    out.add(utf8.encode('Hello world!\n'));
    out.close();
    await stdoutDone;

    return const .ok(null);
  }
}
```

### 3. Build the component

To build the component, use `wasm_tools compile`:

```sh
dart run wasm_tools compile bin/dart_wasm_cli.dart
```

The [link hook] of the `wasi` package detects that this Dart program defines a
command component and configures the relevant imports and exports.
This allows the component to target `wasi:cli/command@0.3.0`.

### 4. Run the component with `wasmtime`

To run your command component:

```sh
wasmtime run bin/dart_wasm_cli.wasm
```

[link hook]: https://github.com/simolus3/wasm.dart/blob/ec9f801ebe258fc4c8dc0dceec35af8b9447627f/pkg/wasi/hook/link.dart#L17-L21
