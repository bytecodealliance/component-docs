# Using HTTP in Dart Components

> [!WARNING]
> Compiling Dart to non-web WebAssembly targets is experimental.
> This guide requires Dart version `3.14.0-251.0.dev` or later.

### 1. Create a new Dart project

Again, we'll start with a fresh Dart package:

```sh
dart create -t cli dart_wasm_service
```

To configure this package for WebAssembly components, add tooling dependencies.
Additionally, the `wasi` package provides generated bindings to WASI definitions,
meaning that running `dart run wasm_tools witgen` won't be necessary.

```sh
dart pub add wasi wasm_components dev:wasm_tools
```

### 2. Writing the HTTP handler

The following code can be inserted into `bin/dart_wasm_service.dart`:

```dart
import 'dart:async';
import 'dart:convert';

import 'package:wasi/src/components/wasi_http_service.dart';
import 'package:wasi/src/components/wasi_http.dart';
import 'package:wasm_components/wasm_components.dart';

void main() {
  serviceComponent((imports) => _RequestHandler(imports));
}

final class _RequestHandler(final ServiceImports _imports) implements Handler {
  var _requestId = 0;

  @override
  Future<Result<Owned<TypesResponse>, TypesErrorCode>> handle({
    required Owned<TypesRequest> request,
  }) async {
    final headers = _imports.httpTypes.constructorFields();

    final responseText =
        '''
<!doctype html>
<html>
<head>
  <title>dart2wasm http server</title>
</head>
<body>
<h1>This website is running Dart!</h1>

<p>
This is request number ${_requestId++} served by this server.
</p>
</body>
</html>
''';

    final (response, _) = _imports.httpTypes.staticResponseNew(
      headers: headers,
      contents: .some(.value(utf8.encode(responseText))),
      trailers: Future.syncValue(.ok(.none)),
    );

    request.drop();
    return .ok(response);
  }
}
```

### 3. Build the component

To build the component, use `wasm_tools compile`:

```sh
dart run wasm_tools compile bin/dart_wasm_service.dart
```

### 4. Serve the component with `wasmtime`

To run your HTTP service component:

```sh
wasmtime serve bin/dart_wasm_service.wasm -O pooling-max-tables-per-module=4
```

> [!NOTE]
>
> The `pooling-max-tables-per-module` option is required since `wasmtime serve` restricts
> this by default, preventing Dart modules from running.
> `serve` is the only `wasmtime` subcommand with this restriction.
