# C# Tooling

WebAssembly components in C# can be built with [componentize-dotnet][componentize-dotnet],
a a NuGet package that can be used to create a fully AOT-compiled
component, giving .NET developers a component experience comparable to those in Rust and TinyGo.

[componentize-dotnet]: https://github.com/bytecodealliance/componentize-dotnet

## Building a Component with `componentize-dotnet`

[`componentize-dotnet`][componentize-dotnet] serves as a one-stop shop, wrapping several tools into one:

- [NativeAOT-LLVM](https://github.com/dotnet/runtimelab/tree/feature/NativeAOT-LLVM) (compilation)
- [wit-bindgen](https://github.com/bytecodealliance/wit-bindgen) (WIT imports and exports)
- [wasm-tools](https://github.com/bytecodealliance/wasm-tools) (component conversion)
- [WASI SDK](https://github.com/WebAssembly/wasi-sdk) (SDK used by NativeAOT-LLVM)
- [Wac](https://github.com/bytecodealliance/wac) (used to compose components)

First, install the .NET SDK. For this walkthrough, we’ll use the [.NET 10 SDK preview][dotnet-sdk].
You should also have [wasmtime](https://wasmtime.dev/) installed so you can run the binary that you produce.

[dotnet-sdk]: https://dotnet.microsoft.com/en-us/download/dotnet/10.0
[wasmtime]: https://wasmtime.dev/

## 1. Create a new project

Once you have the .NET SDK installed, create a new project:

```console
dotnet new install BytecodeAlliance.Componentize.DotNet.Templates
dotnet new componentize.wasi.cli -o adder
cd adder
```

## 2. Create or download your WIT world

Next, create or download the WIT world you would like to target.

For this example we will use the [`adder` world][adder-world], with an `add` function (e.g. to `wit/component.wit`):

```wit
package docs:adder@0.1.0;

interface add {
    add: func(x: u32, y: u32) -> u32;
}

world adder {
    export add;
}
```

In the `adder.csproj` project file, add a new `<ItemGroup>`:

```xml
<ItemGroup>
    <Wit Update="wit/component.wit" World="adder" />
</ItemGroup>
```

Since this component will only export a function dotnet considers this a library project.
Let's update the `<OutputType>` to be a library in the `adder.csproj`:

```diff
- <OutputType>Exe</OutputType>
+ <OutputType>Library</OutputType>
```

And remove the automatically generated `Program.cs` file:

```bash
rm Program.cs
```

[adder-world]: https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/tutorial/wit/adder/world.wit

## 3. Write the implementation for the `adder` world

If you try to build the project with `dotnet build`, you'll get an error like the following:

```
➜ dotnet build
Restore complete (8.6s)
You are using a preview version of .NET. See: https://aka.ms/dotnet-support-policy
  adder failed with 1 error(s) (25.6s)
    /path/to/adder/obj/Debug/net10.0/wasi-wasm/wit_bindgen/AdderWorld.wit.exports.docs.adder.v0_1_0.AddInterop.cs(15,19): error CS0103: The name 'AddImpl' does not exist in the current context

Build failed with 1 error(s) in 34.6s
```

This is because we've promised an implementation, but haven't yet written one for the `adder` world.

To fix this, add the following code to your in a file called `Component.cs`:

```csharp
namespace AdderWorld;

public class AdderWorldImpl : IAdderWorld
{
    public static uint Add(uint x, uint y)
    {
        return x + y;
    }
}
```

Then, we can build our component:

```console
dotnet build
```

The component will be available at `bin/Debug/net10.0/wasi-wasm/native/adder.wasm`.

### 5. (optional) the component from the example host

> [!WARNING]
> You must be careful to use a version of the adapter (`wasi_snapshot_preview1.wasm`) that is compatible with the version of
> `wasmtime` that will be used, to ensure that WASI interface versions (and relevant implementation) match.

This repository contains an [example WebAssembly host][example-host] written in Rust that can run components that implement the `adder` world.

> [!NOTE]
> When hosts run components that use WASI interfaces, they must *explicitly* [add WASI to the linker][add-to-linker] to run the built component.

A successful run should show the following output:

```
cargo run --release -- 1 2 adder.component.wasm
   Compiling example-host v0.1.0 (/path/to/component-docs/component-model/examples/example-host)
    Finished `release` profile [optimized] target(s) in 7.85s
     Running `target/debug/example-host 1 2 /tmp/docs/c/adder.component.wasm`
1 + 2 = 3
```

If *not* configured correctly, you may see errors like the following:

```
cargo run --release -- 1 2 adder.component.wasm
   Compiling example-host v0.1.0 (/path/to/component-docs/component-model/examples/example-host)
    Finished `release` profile [optimized] target(s) in 7.85s
     Running `target/release/example-host 1 2 adder.component.wasm`
Error: Failed to instantiate the example world

Caused by:
    0: component imports instance `wasi:io/error@0.2.2`, but a matching implementation was not found in the linker
    1: instance export `error` has the wrong type
    2: resource implementation is missing
```

This kind of error normally indicates that the host in question does not contain satisfy WASI imports.

[host]: https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/example-host
[add-to-linker]: https://docs.wasmtime.dev/api/wasmtime_wasi/fn.add_to_linker_sync.html

## Building a component that exports an interface

The previous example uses a WIT file that exports a function. However, you'll often prefer to export an interface,
either to comply with an existing specification or to capture a set of functions and types that tend to go
together. Let's expand our `example` world to export an interface rather than directly
export the function. We are also adding the `hostapp` world to our WIT file which we will implement
in [the next section](#building-a-component-that-imports-an-interface) to demonstrate how to build a
component that *imports* an interface.

```wit
// adder/world.wit
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
rather than for the `example` world -- this is a consistent pattern. As you export more interfaces
from your world, you implement more classes.

Our `Component.cs` example gets the slight update of:

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

```console
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

Back out of the current project and create a new one:

```console
cd ..
dotnet new componentize.wasi.cli -o host-app
cd host-app
```

Copy the same WIT file as before into your project:

```wit
// adder/world.wit
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
    <Wit Update="adder/add.wit" World="hostapp" />
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

uint left = 1;
uint right = 2;
var result = AddInterop.Add(left, right);
Console.WriteLine($"{left} + {right} = {result}");
```

Once again, compile your component with `dotnet build`:

```console
$ dotnet build
Restore complete (0.4s)
You are using a preview version of .NET. See: https://aka.ms/dotnet-support-policy
  host-app succeeded (1.1s) → bin/Debug/net10.0/wasi-wasm/host-app.dll

Build succeeded in 2.5s
```

At this point, you'll have two Webassembly components:

1. A component that implements the `example` world.
2. A component that implements the `hostapp` world.

Since the `host-app` component depends on the `add` function which is defined in the `example`
world, it needs to be composed the first component. You can compose your `host-app` component with
your `adder` component by running [`wac plug`](https://github.com/bytecodealliance/wac):

```console
wac plug \
    bin/Debug/net10.0/wasi-wasm/native/host-app.wasm \
    --plug ../adder/bin/Debug/net10.0/wasi-wasm/native/adder.wasm \
    -o main.wasm
```

You can also automate the process by adding the following to your `host-app.csproj`:

```xml
<Target Name="ComposeWasmComponent" AfterTargets="Publish">
    <PropertyGroup>
        <EntrypointComponent>bin/$(Configuration)/$(TargetFramework)/wasi-wasm/native/host-app.wasm</EntrypointComponent>
        <DependencyComponent>../adder/bin/$(Configuration)/$(TargetFramework)/wasi-wasm/native/adder.wasm</DependencyComponent>
    </PropertyGroup>
    <MakeDir Directories="dist" />
    <Exec Command="$(WacExe) plug $(EntrypointComponent) --plug $(DependencyComponent) -o dist/main.wasm" />
</Target>
```

Run `dotnet build` again you will have a composed component in `./dist/main.wasm`

Then you can run the composed component:

```console
wasmtime run ./dist/main.wasm
1 + 2 = 3
```

Check out the [componentize-dotnet docs][componentize-dotnet-docs] for more configurations options.

[componentize-dotnet-docs]: https://github.com/bytecodealliance/componentize-dotnet
