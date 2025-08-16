# C# Tooling

WebAssembly components in C# can be built with [componentize-dotnet][componentize-dotnet],
a NuGet package that can be used to create a fully ahead-of-time-compiled component,
giving .NET developers a component experience comparable to those in Rust and TinyGo.

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
You will also need to install [wac][wac] for composing components.

[dotnet-sdk]: https://dotnet.microsoft.com/en-us/download/dotnet/10.0
[wasmtime]: https://wasmtime.dev/
[wac]: https://github.com/bytecodealliance/wac

## 1. Create a new project

Once you have the .NET SDK installed, create a new project:

```sh
dotnet new install BytecodeAlliance.Componentize.DotNet.Templates
dotnet new componentize.wasi.cli -o adder
cd adder
```

## 2. Create or download your WIT world

Next, create or download the WIT world you would like to target.

For this example we will use the [`adder` world][adder-world], with an `add` function.
Copy and paste the following into a new file called "`wit/component.wit`".

```wit
{{#include ../../examples/tutorial/wit/adder/world.wit}}
```

In the `adder.csproj` project file, add a new `<ItemGroup>`
at the same level as the existing `<ItemGroup>`:

```xml
<ItemGroup>
    <Wit Update="wit/component.wit" World="adder" />
</ItemGroup>
```

Since this component will only export functionality, dotnet considers this a library project.
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

To fix this, add the following code in a file called `Component.cs`:

```csharp
{{#include ../../examples/tutorial/csharp/adder/Component.cs}}
```

Then, we can build our component:

```sh
dotnet build
```

The component will be available at `bin/Debug/net10.0/wasi-wasm/native/adder.wasm`.

### 4. (optional) Run the component from the example host

The following section requires you to have [a Rust toolchain][rust] installed.

{{#include example-host-part1.md}}

A successful run should show the following output
(of course, the paths to your example host and adder component will vary):

{{#include example-host-part2.md}}

[rust]: https://www.rust-lang.org/learn/get-started

## Building a component that exports an interface

The previous example uses a WIT file that exports a function.
However, you'll often prefer to export an interface,
either to comply with an existing specification
or to capture a set of functions and types that tend to go together.
Let's expand our `example` world to export an interface rather than directly export the function.
We are also adding the `hostapp` world to our WIT file,
which we will implement in [the next section](#building-a-component-that-imports-an-interface)
to demonstrate how to build a component that *imports* an interface.

Edit your `wit/component.wit` file to look like this:

```wit
{{#include ../../examples/tutorial/csharp/adder/world-hostapp.wit}}
```

In our `Component.cs` example, we change the `namespace` declaration
to refer to the package name in the `wit/component.wit` file,
and change the `AddImpl` class so that it implements the `add` interface:

```csharp
{{#include ../../examples/tutorial/csharp/adder/Component2.cs}}
```

You also need to edit your `adder.csproj` project file as follows,
to reflect that we are now implementing the `example` world:

```diff
- <Wit Update="wit/component.wit" World="adder" />
+ <Wit Update="wit/component.wit" World="example" />
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

If you peek at the bindings, you'll notice that we now implement a class for the `add` interface
rather than for the `example` world—this is a consistent pattern.
As you export more interfaces from your world, you implement more classes.

## Building a component that imports an interface

So far, we've been dealing with library components.
Now we will be creating a command component that implements the `hostapp` world.
This component will import the `add` interface that is exported from our `adder` component
and call the `add` function.
We will later compose this command component with the `adder` library component we just built.

Now we will be taking the `adder` component and executing it from another WebAssembly component.

`dotnet new componentize.wasi.cli` creates a new project that creates an executable.

Change to the parent directory of your current project and create a new project:

```sh
cd ..
dotnet new componentize.wasi.cli -o host-app
cd host-app
```

Copy the following WIT file into a file called `wit/add.wit` in your project:

```wit
{{#include ../../examples/tutorial/csharp/adder/world-hostapp.wit}}
```

Add it to your `host-app.csproj` project file as a new `ItemGroup` at the top level:

```xml
<ItemGroup>
    <Wit Update="wit/add.wit" World="hostapp" />
</ItemGroup>
```

Notice how the `World` changed from `example` to `hostapp`.
The previous examples focused on implementing the class library
for this WIT file—the `export` functions.
Now we'll be focusing on the executable side of the application—the `hostapp` world.

Modify `Program.cs` to look like this:

```csharp
{{#include ../../examples/tutorial/csharp/adder/Program.cs}}
```

Once again, compile your component with `dotnet build`:

```sh
$ dotnet build
Restore complete (0.4s)
You are using a preview version of .NET. See: https://aka.ms/dotnet-support-policy
  host-app succeeded (1.1s) → bin/Debug/net10.0/wasi-wasm/host-app.dll

Build succeeded in 2.5s
```

At this point, you'll have two WebAssembly components:

1. A component that implements the `example` world.
2. A component that implements the `hostapp` world.

Since the `host-app` component depends on the `add` function which is defined in the `example` world,
it needs to be composed with the first component.
You can compose your `host-app` component with your `adder` component
by running [`wac plug`](https://github.com/bytecodealliance/wac):

```sh
wac plug \
    bin/Debug/net10.0/wasi-wasm/native/host-app.wasm \
    --plug ../adder/bin/Debug/net10.0/wasi-wasm/native/adder.wasm \
    -o main.wasm
```

If you get an error message like "error: the socket component had no matching imports
for the plugs that were provided",
make sure that the package names in both .wit files
(the one for your `adder` component and the one for your `host-app` component) are the same.

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

This requires your original `adder.wasm` component to be in `../adder`
relative to the directory your `host-app` component is in.

If you run `dotnet build` again, you will have a composed component in `./dist/main.wasm`.

Then you can run the composed component:

```sh
wasmtime run ./dist/main.wasm
1 + 2 = 3
```

Check out the [componentize-dotnet docs][componentize-dotnet-docs] for more configuration options.

[componentize-dotnet-docs]: https://github.com/bytecodealliance/componentize-dotnet

[!NOTE]: #
[!WARNING]: #
