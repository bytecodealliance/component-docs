# C# Tooling

## Building a Component with `componentize-dotnet`

[componentize-dotnet](https://github.com/bytecodealliance/componentize-dotnet) makes it easy to
compile your code to WebAssembly components using a single tool. This Bytecode Alliance project is a
NuGet package that can be used to create a fully AOT-compiled component, giving .NET developers a
component experience comparable to those in Rust and TinyGo.

componentize-dotnet serves as a one-stop shop for .NET developers, wrapping several tools into one:

- [NativeAOT-LLVM](https://github.com/dotnet/runtimelab/tree/feature/NativeAOT-LLVM) (compilation)
- [wit-bindgen](https://github.com/bytecodealliance/wit-bindgen) (WIT imports and exports)
- [wasm-tools](https://github.com/bytecodealliance/wasm-tools) (component conversion)
- [WASI SDK](https://github.com/WebAssembly/wasi-sdk) (SDK used by NativeAOT-LLVM)
- [Wac](https://github.com/bytecodealliance/wac) (used to compose components)

First, install the .NET SDK. For this walkthrough, we’ll use the [.NET 10 SDK preview](https://dotnet.microsoft.com/en-us/download/dotnet/10.0). 
You should also have [wasmtime](https://wasmtime.dev/) installed so you can run the binary that you produce.

Once you have the .NET SDK installed, create a new project:

```sh
dotnet new install BytecodeAlliance.Componentize.DotNet.Templates
dotnet new componentize.wasi.cli -o adder
cd adder
```

Next, create or download the WIT world you would like to target. For this example we will use an
[`example`
world](https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/example-host/add.wit)
with an `add` function:

```wit
package example:component;

world example {
    export add: func(x: u32, y: u32) -> u32;
}
```

In the .csproj project file, add a new `<ItemGroup>`:

```xml
<ItemGroup>
    <Wit Update="add.wit" World="example" />
</ItemGroup>
```

If you try to build the project with `dotnet build`, you'll get an error like "The name
'ExampleWorldImpl' does not exist in the current context". This is because you've said you'll
provide an implementation, but haven't yet done so. To fix this, add the following code to your
project:

```csharp
namespace ExampleWorld;

public class ExampleWorldImpl : IOperations
{
    public static uint Add(uint x, uint y)
    {
        return x + y;
    }
}
```

If we build it:

```sh
dotnet build
```

The component will be available at `bin/Debug/net10.0/wasi-wasm/native/adder.wasm`.

## Building a component that exports an interface

The previous example uses a WIT file that exports a function. However, you'll often prefer to export an interface, 
either to comply with an existing specification or to capture a set of functions and types that tend to go
together. Let's expand our `example` world to export an interface rather than directly
export the function. We are also adding the `hostapp` world to our WIT file which we will implement
in [the next section](#building-a-component-that-imports-an-interface) to demonstrate how to build a
component that *imports* an interface.

```wit
// add.wit
package example:component;

interface add {
    add: func(x: u32, y: u32) -> u32;
}

world example {
    export add;
}

world hostapp {
    import add;
}
```

If you peek at the bindings, you'll notice that we now implement a class for the `add` interface
rather than for the `example` world. This is a consistent pattern. As you export more interfaces
from your world, you implement more classes. Our add example gets the slight update of:

```csharp
namespace ExampleWorld.wit.exports.example.component;

public class AddImpl : IAdd
{
    public static uint Add(uint x, uint y)
    {
        return x + y;
    }
}
```

Once again, compile an application to a Wasm component using `dotnet build`:

```sh
$ dotnet build
Restore complete (0.4s)
You are using a preview version of .NET. See: https://aka.ms/dotnet-support-policy
  adder succeeded (1.1s) → bin/Debug/net10.0/wasi-wasm/adder.dll

Build succeeded in 2.5s
```

The component will be available at `bin/Debug/net10.0/wasi-wasm/native/adder.wasm`.

## Building a component that imports an interface

So far, we've been dealing with library components. Now we will be creating a command component that
implements the `hostapp` world. This component will import the `add` interface that is exported from
our `adder` component and call the `add` function. We will later compose this command component with
the `adder` library component we just built.

Now we will be taking the `adder` component and executing it from another WebAssembly component.
`dotnet new componentize.wasi.cli` creates a new project that creates an executable.

```sh
dotnet new componentize.wasi.cli -o host-app
cd host-app
```

Copy the same WIT file as before into your project:

```wit
// add.wit
package example:component;

interface add {
    add: func(x: u32, y: u32) -> u32;
}

world example {
    export add;
}

world hostapp {
    import add;
}
```

Add it to your `host-app.csproj` project file as a new `ItemGroup`:

```xml
<ItemGroup>
    <Wit Update="add.wit" World="hostapp" />
</ItemGroup>
```

Notice how the `World` changed from `example` to `hostapp`. The previous examples focused on
implementing the class library for this WIT file - the `export` functions. Now we'll be focusing on
the executable side of the application - the `hostapp` world.

Modify `Program.cs` to look like this:

```csharp
// Pull in all imports of the `hostapp` world, namely the `add` interface.
// example.component refers to the package name defined in the WIT file.
using HostappWorld.wit.imports.example.component;

var left = 1;
var right = 2;
var result = AddInterop.Add(left, right);
Console.WriteLine($"{left} + {right} = {result}");
```

Once again, compile your component with `dotnet build`:

```sh
$ dotnet build
Restore complete (0.4s)
You are using a preview version of .NET. See: https://aka.ms/dotnet-support-policy
  host-app succeeded (1.1s) → bin/Debug/net9.0/wasi-wasm/host-app.dll

Build succeeded in 2.5s
```

At this point, you'll have two Webassembly components:

1. A component that implements the `example` world.
2. A component that implements the `hostapp` world.

Since the `host-app` component depends on the `add` function which is defined in the `example`
world, it needs to be composed the first component. You can compose your `host-app` component with
your `adder` component by running [`wac plug`](https://github.com/bytecodealliance/wac):

```sh
wac plug bin/Debug/net10.0/wasi-wasm/native/host-app.wasm --plug ../adder/bin/Debug/net10.0/wasi-wasm/native/adder.wasm -o main.wasm
```

You can also automate the process by adding the following to your `host-app.csproj`:

```xml
<Target Name="ComposeWasmComponent" AfterTargets="Publish">
    <PropertyGroup>
        <EntrypointComponent>bin/$(Configuration)/$(TargetFramework)/wasi-wasm/native/host-app.wasm</EntrypointComponent>
        <DependencyComponent>../example/bin/$(Configuration)/$(TargetFramework)/wasi-wasm/native/adder.wasm</DependencyComponent>
    </PropertyGroup>
    
    <MakeDir Directories="dist" />
    <Exec Command="$(WacExe) plug $(EntrypointComponent) --plug $(DependencyComponent)" -o dist/main.wasm />
</Target>
```

Run `dotnet build` again you will have a composed component in `./dist/main.wasm`

Then you can run the composed component:

```sh
wasmtime run main.wasm
1 + 2 = 3
```
