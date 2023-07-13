# Component model proposed table of contents

This is presented as plain Markdown for easy discussion and iteration. The final form would likely be a [`mdbook`](https://rust-lang.github.io/mdBook/index.html) SUMMARY document, but that doesn't allow for inline comments and explanations.

The intent of this book would be to target Preview 2 only.  Discussion of future ambitions would be clearly ring-fenced to a roadmap section.

- **Introduction to the Component Model**
  - **How to use this book** - *who is this for, how to use this book, how to contribute, background info/prerequisite knowledge*
  - **Roadmap**
    - **Current Status**
    - **Future Directions**
  - **Implementations** - *where can I run components (runtimes, languages, etc)*
- **Tutorial** - *an example to jump into using components. Involves downloading wasm-tools. Maybe this [wasm compose](https://github.com/bytecodealliance/wasm-tools/tree/main/crates/wasm-compose/example) example or Ryan Levick's walkthrough in his [components book](https://github.com/rylev/component-book)*
- **What is a Component?** - *Might include 'differences between modules and components' as a specific section or page*
  - **Use Cases**
  - **Features of Components**
  - **How Components are Physically Represented** - *Some folks feel this is more than users need to know; if so could drop or move to advanced section*
- **Design** 
  - **Goals and Principles** - *Focuses on covering all the "whys"*
  - **WIT Overview** - *explained through an annotated WIT interface*
- **Concepts** - *concepts build upon each other. example oriented rather than just a reference*
  - **Interfaces**
  - **Worlds**
  - **Packages**
  - **Capabilities**
  - **Resources**
- **Creating and Consuming Components** - *how to author a component, combine components, call components, validate a component, distribute a component*
- **Language Support**
  - **C**
  - **Go**
  - **JavaScript**
  - **Python**
  - **Rust**
- **WIT Reference** - *More readable version of [the WIT Definition Language doc](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md)*
- **Advanced**
  - **Binary Interface** - *how strings are laid out in memory, ownership, low-level nuts and bolts*
    - **How Types are Laid Out in Memory**
    - **Function Arguments and Results**
    - **Memory Management**
- **Examples**
- **Additional References**
