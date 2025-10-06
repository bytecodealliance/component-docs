# Using WIT Resources (Rust)

[Resources](../design/wit.md#resources) are handles to entities that live outside the component (i.e. in a host, or other component).

## An example stack-based Reverse Polish Notation (RPN) calculator

In this section, our example resource will be a [Reverse Polish Notation (RPN)](https://en.wikipedia.org/wiki/Reverse_Polish_notation) calculator. (Engineers of a certain vintage will remember this from handheld calculators of the 1970s.)

A RPN calculator is a stateful entity: a consumer pushes operands and operations onto a stack
maintained within the calculator, then evaluates the stack to produce a value.

In WIT, the resource looks like the following:

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

## Implementing and exporting a resource in a component

To implement the calculator in Rust:

1. Create a library component as shown in previous sections, with the WIT given above.

2. Define a Rust `struct` to represent the calculator state:

    ```rust
    use std::cell::RefCell;

    struct CalcEngine {
        stack: RefCell<Vec<u32>>,
    }
    ```

    > Why is the stack wrapped in a `RefCell`? As we will see, the generated Rust trait for the calculator engine has _immutable_ references to `self`. But our implementation of that trait will need to mutate the stack. So we need a type that allows for interior mutability, such as `RefCell<T>` or `Arc<RwLock<T>>`.

3. The generated bindings (`bindings.rs`) for an exported resource include a trait named `GuestX`, where `X` is the resource name. For the calculator `engine` resource, the trait is `GuestEngine`. Implement this trait on the `struct` from step 2:

    ```rust
    mod bindings {
        use super::Component;
        wit_bindgen::generate!();
        export!(Component);
    }

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
    // ... bindings & CalcEngine impl code ...

    struct Component;

    impl bindings::Guest for Component {
        type Engine = CalcEngine;
    }

    bindings::export!(Component);
    ```

This completes the implementation of the calculator `engine` resource. Run `cargo build --target=wasm32-wasip2` to create a component `.wasm` file.

## Importing and consuming a resource in a component

To use the calculator engine in another component, that component must import the resource.

1. [Create a runnable component](../../creating-runnable-components/rust.md) as shown in previous sections.

2. Add a `wit/component.wit` to your project, and write a WIT world that imports the RPN calculator types:

    ```wit
    package docs:rpn-cmd;

    world app {
        import docs:rpn/types@0.1.0;
    }
    ```

3. Create a `wkg.toml` file to enable retrieving the relevant WIT files for `docs:rpn` (which contains the `engine` resource):

    ```toml
    [overrides]
    "docs:rpn" = { path = "../path/to/docs-rpn/wit" }
    ```

    After doing this, you can run `wkg wit fetch` to ensure all WIT is available locally.

4. The resource now appears in the generated bindings as a `struct`, with appropriate associated functions. Use these to construct a test app:

    ```rust
    mod bindings {
        use super::Component;
        wit_bindgen::generate!();
        export!(Component);
    }

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

Building the component as is creates a WebAssembly component with an *unsatisfied import* -- namely the `docs:rpn/types` import.

After building the component, it must be [composed with a `.wasm` component that implements the resource.](../composing-and-distributing/composing.md). After composition creates a component with no unsatisfied imports, the composed command component can be run with `wasmtime run`.

Alternatively, a host that can provide the `docs:rpn/types` import (and related resource) can also be used to run the component
in it's "incomplete" state (as the host will "complete" the componnt by providing the expected import).

## Implementing and exporting a resource implementation in a host

If you are hosting a Wasm runtime, you can export a resource from your host for guests to consume.

Hosting a runtime is outside the scope of this book, so we will give only a broad outline here. This is specific
to the Wasmtime runtime; other runtimes may express things differently.

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

[docs-adder]: https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/tutorial/wit/adder/world.wit

[!NOTE]: #
[!WARNING]: #
