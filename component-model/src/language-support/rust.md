# Components in Rust

Rust has first-class support for the component model via [the `cargo component` tool](https://github.com/bytecodealliance/cargo-component). It is a `cargo` subcommand for
creating WebAssembly components using Rust as the component's implementation language.

## Installing `cargo component`

To install `cargo component`, run:

```sh
cargo install cargo-component
```

> You can find more details about `cargo component` in its [crates.io page](https://crates.io/crates/cargo-component).

## Building a Component with `cargo component`

Create a Rust library that implements the `add` function in the [`example`
world](https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/example-host/add.wit). First scaffold a project:

```sh
$ cargo component new adder --lib && cd adder
```

Note that `cargo component` generates the necessary bindings as a module called `bindings`. 

Next, update `wit/world.wit` to match `add.wit` and modify the component package reference to change the
package name to `example:component`. The `component` section of `Cargo.toml` should look like the following:

```toml
[package.metadata.component]
package = "example:component"
```

`cargo component` will generate bindings for the world specified in a package's `Cargo.toml`. In particular, it will create a `Guest` trait that a component should implement. Since our `example` world has no interfaces, the trait lives directly under the bindings module. Implement the `Guest` trait in `add/src/lib.rs` such that it satisfies the `example` world, adding an `add` function:

```rs
struct Component;

impl Guest for Component {
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
}
```

Now, use `cargo component` to build the component, being sure to optimize with a release build.

```sh
$ cargo component build --release
```

You can use `wasm-tools component wit` to output the WIT package of the component:

```sh
$ wasm-tools component wit target/wasm32-wasip1/release/add.wasm
package root:component;

world root {
  export add: func(x: s32, y: s32) -> s32;
}
```

### Running a Component from Rust Applications

To verify that our component works, lets run it from a Rust application that knows how to run a
component targeting the [`example` world](https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/example-host/add.wit).

The application uses [`wasmtime`](https://github.com/bytecodealliance/wasmtime) crates to generate
Rust bindings, bring in WASI worlds, and execute the component.

```sh
$ cd examples/example-host
$ cargo run --release -- 1 2 ../add/target/wasm32-wasip1/release/add.wasm
1 + 2 = 3
```

## Exporting an interface with `cargo component`

The [sample `add.wit` file](https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/example-host/add.wit) exports a function. However, you'll often prefer to export an interface, either to comply with an existing specification or to capture a set of functions and types that tend to go together. For example, to implement the following world:

```wit
package docs:adder@0.1.0;

interface add {
    add: func(a: u32, b: u32) -> u32;
}

world adder {
    export add;
}
```

you would write the following Rust code:

```rust
mod bindings;
// Separating out the interface puts it in a sub-module
use bindings::exports::docs::adder::add::Guest;

struct Component;

impl Guest for Component {
    fn add(a: u32, b: u32) -> u32 {
        a + b
    }
}
```

## Importing an interface with `cargo component`

The world file (`wit/world.wit`) generated for you by `cargo component new --lib` doesn't specify any imports.

> `cargo component build`, by default, uses the Rust `wasm32-wasi` target, and therefore automatically imports any required WASI interfaces - no action is needed from you to import these. This section is about importing custom WIT interfaces from library components.

If your component consumes other components, you can edit the `world.wit` file to import their interfaces.

For example, suppose you have created and built an adder component as explained in the [exporting an interface section](#exporting-an-interface-with-cargo-component) and want to use that component in a calculator component. Here is a partial example world for a calculator that imports the add interface:

```wit
// in the 'calculator' project

// wit/world.wit
package docs:calculator;

interface calculate {
    eval-expression: func(expr: string) -> u32;
}

world calculator {
    export calculate;
    import docs:adder/add@0.1.0;
}
```

### Referencing the package to import

Because the `docs:adder` package is in a different project, we must first tell `cargo component` how to find it. To do this, add the following to the `Cargo.toml` file:

```toml
[package.metadata.component.target.dependencies]
"docs:adder" = { path = "../adder/wit" }  # directory containing the WIT package
```

Note that the path is to the adder project's WIT _directory_, not to the `world.wit` file. A WIT package may be spread across multiple files in the same directory; `cargo component` will look at all the files.

### Calling the import from Rust

Now the declaration of `add` in the adder's WIT file is visible to the `calculator` project. To invoke the imported `add` interface from the `calculate` implementation:

```rust
// src/lib.rs
mod bindings;

use bindings::exports::docs::calculator::calculate::Guest;

// Bring the imported add function into scope
use bindings::docs::calculator::add::add;

struct Component;

impl Guest for Component {
    fn eval_expression(expr: String) -> u32 {
        // Cleverly parse `expr` into values and operations, and evaluate
        // them meticulously.
        add(123, 456)
    }
}
```

### Fulfilling the import

When you build this using `cargo component build`, the `add` interface remains imported. The calculator has taken a dependency on the `add` _interface_, but has not linked the `adder` implementation of that interface - this is not like referencing the `adder` crate. (Indeed, `calculator` could import the `add` interface even if there was no Rust project implementing the WIT file.) You can see this by running [`wasm-tools component wit`](https://github.com/bytecodealliance/wasm-tools/tree/main/crates/wit-component) to view the calculator's world:

```
# Do a release build to prune unused imports (e.g. WASI)
$ cargo component build --release

$ wasm-tools component wit ./target/wasm32-wasip1/release/calculator.wasm
package root:component;

world root {
  import docs:adder/add@0.1.0;

  export docs:calculator/calculate@0.1.0;
}
```

As the import is unfulfilled, the `calculator.wasm` component could not run by itself in its current form. To fulfill the `add` import, so that only `calculate` is exported, you would need to [compose the `calculator.wasm` with some `exports-add.wasm` into a single, self-contained component](../creating-and-consuming/composing.md).

## Creating a command component with `cargo component`

A _command_ is a component with a specific export that allows it to be executed directly by `wasmtime` (or other `wasi:cli` hosts). In Rust terms, it's the equivalent of an application (`bin`) package with a `main` function, instead of a library crate (`lib`) package.

To create a command with cargo component, run:

```sh
cargo component new <name>
```

Unlike library components, this does _not_ have the `--lib` flag. You will see that the created project is different too:

- It doesn't contain a `.wit` file. `cargo component build` will automatically export the `wasi:cli/run` interface for Rust `bin` packages, and hook it up to `main`.
- Because there's no `.wit` file, `Cargo.toml` doesn't contain a `package.metadata.component.target` section.
- The Rust file is called `main.rs` instead of `lib.rs`, and contains a `main` function instead of an interface implementation.

You can write Rust in this project, just as you normally would, including importing your own or third-party crates.

> All the crates that make up your project are linked together at build time, and compiled to a _single_ Wasm component. In this case, all the linking is happening at the Rust level: no WITs or component composition is involved. Only if you import Wasm interfaces do WIT and composition come into play.

To run your command component:

```sh
cargo component build
wasmtime run ./target/wasm32-wasip1/debug/<name>.wasm
```

> **WARNING:** If your program prints to standard out or error, you may not see the printed output! Some versions of `wasmtime` have a bug where they don't flush output streams before exiting. To work around this, add a `std::thread::sleep()` with a 10 millisecond delay before exiting `main`.

### Importing an interface into a command component

As mentioned above, `cargo component build` doesn't generate a WIT file for a command component. If you want to import a Wasm interface, though, you'll need to create a WIT file and a world, plus reference the packages containing your imports:

1. Add a `wit/world.wit` to your project, and write a WIT world that imports the interface(s) you want to use. For example:

    ```wit
    package docs:app;

    world app {
        import docs:calculator/calculate@0.1.0;
    }
    ```

    > `cargo component` sometimes fails to find packages if versions are not set explicitly. For example, if the calculator WIT declares `package docs:calculator` rather than `docs:calculator@0.1.0`, then you may get an error even though `cargo component build` automatically versions the binary export.

2. Edit `Cargo.toml` to tell `cargo component` about the new WIT file:

    ```toml
    [package.metadata.component.target]
    path = "wit"
    ```

    (This entry is created automatically for library components but not for command components.)

3. Edit `Cargo.toml` to tell `cargo component` where to find external package WITs:

    ```toml
    [package.metadata.component.target.dependencies]
    "docs:calculator" = { path = "../calculator/wit" }
    "docs:adder" = { path = "../adder/wit" }
    ```

    > If the external package refers to other packages, you need to provide the paths to them as well.

4. Use the imported interface in your Rust code:

    ```rust
    use bindings::docs::calculator::calculate::eval_expression;

    fn main() {
        let result = eval_expression("1 + 1");
        println!("1 + 1 = {result}");
    }
    ```

5. [Compose the command component with the `.wasm` components that implement the imports.](../creating-and-consuming/composing.md)

6. Run the composed component:

    ```sh
    $ wasmtime run ./my-composed-command.wasm
    1 + 1 = 579  # might need to go back and do some work on the calculator implementation
    ```

## Using user-defined types

[User-defined types](../design/wit.md#user-defined-types) map to Rust types as follows.

| WIT type   | Rust binding |
|------------|--------------|
| `record`   | `struct` with public fields corresponding to the record fields |
| `variant`  | `enum` with cases corresponding to the variant cases |
| `enum`     | `enum` with cases corresponding to the enum cases, with no data attached |
| `resource` | [See below](#using-resources) |
| `flags`    | Opaque type supporting bit flag operations, with constants for flag values |

For example, consider the following WIT:

```wit
interface types {
    enum operation {
        add,
        sub,
        mul,
        div
    }

    record expression {
        left: u32,
        operation: operation,
        right: u32
    }

    eval: func(expr: expression) -> u32;
}
```

When exported from a component, this could be implemented as:

```rust
impl Guest for Implementation {
    fn eval(expr: Expression) -> u32 {
        // Record fields become public fields on a struct
        let (l, r) = (expr.left, expr.right);
        match expr.operation {
            // Enum becomes an enum with only unit cases
            Operation::Add => l + r,
            Operation::Sub => l - r,
            Operation::Mul => l * r,
            Operation::Div => l / r,
        }
    }
}
```

## Using resources

[Resources](../design/wit.md#resources) are handles to entities that live outside the component, for example in a host, or in a different component.

### Example

In this section, our example resource will be a [Reverse Polish Notation (RPN)](https://en.wikipedia.org/wiki/Reverse_Polish_notation) calculator. (Engineers of a certain vintage will remember this from handheld calculators of the 1970s.) A RPN calculator is a stateful entity: a consumer pushes operands and operations onto a stack maintained within the calculator, then evaluates the stack to produce a value. The resource in WIT looks like this:

```wit
package docs:rpn@0.1.0;

interface types {
    enum operation {
        add,
        sub,
        mul,
        div
    }

    resource engine {
        constructor();
        push-operand: func(operand: u32);
        push-operation: func(operation: operation);
        execute: func() -> u32;
    }
}

world calculator {
    export types;
}
```

### Implementing and exporting a resource in a component

To implement the calculator using `cargo component`:

1. Create a library component as shown in previous sections, with the WIT given above.

2. Define a Rust `struct` to represent the calculator state:

    ```rust
    use std::cell::RefCell;

    struct CalcEngine {
        stack: RefCell<Vec<u32>>,
    }
    ```

    > Why is the stack wrapped in a `RefCell`? As we will see, the generated Rust trait for the calculator engine has _immutable_ references to `self`. But our implementation of that trait will need to mutate the stack. So we need a type that allows for interior mutability, such as `RefCell<T>` or `Arc<RwLock<T>>`.

3. The generated bindings (`bindings.rs`) for an exported resource include a trait named `GuestX`, where `X` is the resource name. (You may need to run `cargo component build` to regenerate the bindings after updating the WIT.) For the calculator `engine` resource, the trait is `GuestEngine`. Implement this trait on the `struct` from step 2:

    ```rust
    use bindings::exports::docs::rpn::types::{GuestEngine, Operation};

    impl GuestEngine for CalcEngine {
        fn new() -> Self {
            CalcEngine {
                stack: RefCell::new(vec![])
            }
        }

        fn push_operand(&self, operand: u32) {
            self.stack.borrow_mut().push(operand);
        }

        fn push_operation(&self, operation: Operation) {
            let mut stack = self.stack.borrow_mut();
            let right = stack.pop().unwrap(); // TODO: error handling!
            let left = stack.pop().unwrap();
            let result = match operation {
                Operation::Add => left + right,
                Operation::Sub => left - right,
                Operation::Mul => left * right,
                Operation::Div => left / right,
            };
            stack.push(result);
        }

        fn execute(&self) -> u32 {
            self.stack.borrow_mut().pop().unwrap() // TODO: error handling!
        }
    }
    ```

4. We now have a working calculator type which implements the `engine` contract, but we must still connect that type to the `engine` resource type. This is done by implementing the generated `Guest` trait. For this WIT, the `Guest` trait contains nothing except an associated type. You can use an empty `struct` to implement the `Guest` trait on. Set the associated type for the resource - in our case, `Engine` - to the type which implements the resource trait - in our case, the `CalcEngine` `struct` which implements `GuestEngine`. Then use the `export!` macro to export the mapping:

    ```rust
    struct Implementation;
    impl Guest for Implementation {
        type Engine = CalcEngine;
    }

    bindings::export!(Implementation with_types_in bindings);
    ```

This completes the implementation of the calculator `engine` resource. Run `cargo component build` to create a component `.wasm` file.

### Importing and consuming a resource in a component

To use the calculator engine in another component, that component must import the resource.

1. Create a command component as shown in previous sections.

2. Add a `wit/world.wit` to your project, and write a WIT world that imports the RPN calculator types:

    ```wit
    package docs:rpn-cmd;

    world app {
        import docs:rpn/types@0.1.0;
    }
    ```

3. Edit `Cargo.toml` to tell `cargo component` about the new WIT file and the external RPN package file:

    ```toml
    [package.metadata.component]
    package = "docs:rpn-cmd"

    [package.metadata.component.target]
    path = "wit"

    [package.metadata.component.target.dependencies]
    "docs:rpn" = { path = "../wit" } # or wherever your resource WIT is
    ```

4. The resource now appears in the generated bindings as a `struct`, with appropriate associated functions. Use these to construct a test app:

    ```rust
    #[allow(warnings)]
    mod bindings;
    use bindings::docs::rpn::types::{Engine, Operation};

    fn main() {
        let calc = Engine::new();
        calc.push_operand(1);
        calc.push_operand(2);
        calc.push_operation(Operation::Add);
        let sum = calc.execute();
        println!("{sum}");
    }
    ```

You can now build the command component and [compose it with the `.wasm` component that implements the resource.](../creating-and-consuming/composing.md). You can then run the composed command with `wasmtime run`.

### Implementing and exporting a resource implementation in a host

If you are hosting a Wasm runtime, you can export a resource from your host for guests to consume. Hosting a runtime is outside the scope of this book, so we will give only a broad outline here. This is specific to the Wasmtime runtime; other runtimes may express things differently.

1. Use `wasmtime::component::bindgen!` to specify the WIT you are a host for:

    ```rust
    wasmtime::component::bindgen!({
        path: "../wit"
    });
    ```

2. Tell `bindgen!` how you will represent the resource in the host via the `with` field. This can be any Rust type. For example, the RPN engine could be represented by a `CalcEngine` struct:

    ```rust
    wasmtime::component::bindgen!({
        path: "../wit",
        with: {
            "docs:rpn/types/engine": CalcEngine,
        }
    });
    ```

    > If you don't specify the host representation for a resource, it defaults to an empty enum. This is rarely useful as resources are usually stateful.

3. If the representation type isn't a built-in type, define it:

    ```rust
    struct CalcEngine { /* ... */ }
    ```

4. As a host, you will already be implementing a `Host` trait. You will now need to implement a `HostX` trait (where `X` is the resource name) _on the same type_ as the `Host` trait:

    ```rust
    impl docs::rpn::types::HostEngine for MyHost {
        fn new(&mut self) -> wasmtime::component::Resource<docs::rpn::types::Engine> { /* ... */ }
        fn push_operand(&mut self, self_: wasmtime::component::Resource<docs::rpn::types::Engine>) { /* ... */ }
        // etc.
    }
    ```

    > **Important:** You implement this on the 'overall' host type, *not* on the resource representation! Therefore, the `self` reference in these functions is to the 'overall' host type. For instance methods of the resource, the instance is identified by a second parameter (`self_`), of type `wasmtime::component::Resource`.

5. Add a `wasmtime::component::ResourceTable` to the host:

    ```rust
    struct MyHost {
        calcs: wasmtime::component::ResourceTable,
    }
    ```

6. In your resource method implementations, use this table to store and access instances of the resource representation:

    ```rust
    impl docs::rpn::types::HostEngine for MyHost {
        fn new(&mut self) -> wasmtime::component::Resource<docs::rpn::types::Engine> {
            self.calcs.push(CalcEngine::new()).unwrap() // TODO: error handling
        }
        fn push_operand(&mut self, self_: wasmtime::component::Resource<docs::rpn::types::Engine>) {
            let calc_engine = self.calcs.get(&self_).unwrap();
            // calc_engine is a CalcEngine - call its functions
        }
        // etc.
    }
    ```
