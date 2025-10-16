# Using WIT resources

This section contains language-specific guides on how to use [WIT][docs-wit] resources.

Resources represent functionality that is implemented only on one side of a component boundary,
for example in another component or in the underlying platform/host.

An example of a resource:

```wit
package docs:calc-resource@0.1.0;

interface types {
    enum operation {
        add,
        sub,
        mul,
        div,
    }

    variant execute-error {
        divide-by-zero,
        unexpected(string),
    }

    resource stack-calculator {
        constructor();
        push-operand: func(operand: u32);
        push-operation: func(operation: operation);
        execute: func() -> result<u32, error>;
    }
}

world calculator {
    export types;
}
```

Hosts or Components implementing the stack-based calculator above do share the *resource* (the `stack-calculator` entity)
with components that `import` that functionality, but do *not* share the actual implementation. All calls to `stack-calculator`
resources resolve inside the component that `export`ed the functionality.

## Languages

This guide is implemented for various languages:

| Language                                               |
|--------------------------------------------------------|
| [Rust](./language-support/using-wit-resources/rust.md) |

[docs-wit]: ./design/wit.md
