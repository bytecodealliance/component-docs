# Importing and Reusing components (Dart)

> [!WARNING]
> Compiling Dart to non-web WebAssembly targets is experimental.
> This guide requires Dart version `3.14.0` or later.

## Importing an interface

The simple [adder world file] doesn't specify any imports.
If your component consumes other components, you can edit the relevant `.wit` file to import their interfaces.

For example, suppose you have created and built the adder component as explained in the [earlier tutorials] and want to use
that component in a calculator component. Here is a partial example world for a calculator that imports the add interface:

```wit
{{#include ../../../examples/tutorial/wit/calculator/world.wit}}
```

### Referencing the package to import

To generate code for multiple WIT packages, we need multiple `.wit` files.
Unlike the single `.wit` file the previous tutorials stored in the root directory
of your project, complex `.wit` files are typically stored in a `wit/` directory.

Begin with a new Dart project:

```sh
dart create -t cli dart_calculator
cd dart_calculator
dart pub add wasm_components dev:wasm_tools
```

Next, create a `wit/` directory with this layout (you can copy files from [here][adder world file]):

```
pubspec.yaml
wit/
├── deps/
│   └── docs-adder-0.1.0/
│       └── adder.wit
└── calculator.wit
```

To generate Dart code, select the directory and the root world:

```sh
dart run wasm_tools witgen -i ./wit -w "docs:calculator/calculator"
```

### Calling the import from Dart

Because the `calculator` world imports the `add` interface, the declaration of
`add` in the adder's WIT file is visible as an import when defining the
`Calculate` component (in `bin/dart_calculator.dart`):

```dart
import 'package:dart_calculator/src/components/docs_adder.dart';
import 'package:dart_calculator/src/components/docs_calculator.dart';
import 'package:dart_calculator/src/components/docs_calculator_calculator.dart';

void main() {
  calculatorComponent((imports) => _Calculator(add: imports.adderAdd));
}

final class const _Calculator({required final Add add}) implements Calculate {
  @override
  int evalExpression({
    required CalculateOp op,
    required int x,
    required int y,
  }) {
    return switch (op) {
      .add => add.add(x: x, y: y),
    };
  }
}
```

Again, a link hook needs to define imports and exports:

```dart
// hook/link.dart
import 'dart:convert';
import 'dart:io';

import 'package:hooks/hooks.dart';
import 'package:wasm_tools/hooks.dart';

void main(List<String> args) => link(args, (input, output) async {
  if (input.config.buildWasmComponent) {
    final abi = input.packageRoot.resolve('lib/src/components/docs_calculator_calculator.json');

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

### Fulfilling the import

After building a component with `dart run wasm_tools compile`, the `add` interface
remains unsatisfied (i.e. imported).

The calculator has taken a dependency on the `add` _interface_, but has not linked the `adder` implementation of
that interface - this is not like referencing the `Add` Dart class (Indeed, `calculator` could import the `add` interface even if there was no Dart implementation of the WIT file).

You can see this by running [`wasm-tools component wit`](https://github.com/bytecodealliance/wasm-tools/tree/main/crates/wit-component) to view the calculator's world:

```
$ dart run wasm_tools compile bin/dart_calculator.dart --no-implicit-wasi-imports

$ wasm-tools component wit ./bin/dart_calculator.wasm
package root:component;

world root {
  import docs:adder/add@0.1.0;

  export docs:calculator/calculate@0.1.0;
}
```

As the import is unfulfilled, the `calculate.wasm` component could not run by itself in its current form. The next step is to fulfill the `add` import, creating a runnable component exporting `calculate` without any imports.

The process of fulfilling imports via other components' exports is called "composition". Learn more about how to compose the calculator.wasm
with an adder.wasm into a single, self-contained component in the [component composition guide](../../composing-and-distributing/composing.md).

[adder world file]: ../../importing-and-reusing-components.md
[earlier tutorials]: ../building-a-simple-component/dart.md
