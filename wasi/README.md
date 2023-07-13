# WASI proposed table of contents

This is presented as plain Markdown for easy discussion and iteration. The final form would likely be a [`mdbook`](https://rust-lang.github.io/mdBook/index.html) SUMMARY document, but that doesn't allow for inline comments and explanations.

The intent of this book would be to target Preview 2 only, initially focusing on `wasi-http` to establish a sense for how API documentation could be best structured. There may be scope for a Preview 1 book but that's not what this one is for.

- Introduction to WASI
  - How to use this book - *who is this for, how to use this book, how to contribute, background info/prerequisite knowledge*
  - Roadmap
    - Current status
    - Future directions
  - Implementations - *runtimes and stdlibs that support P2 - maybe not in first drop?*
- Tutorial
  - C
  - Rust
- Overview and Design
  - Goals and Principles
    - The Component Model
  - Concepts - *some of these may be by reference to the component model, but we need readers to be able to find things by looking in __this__ book, rather than knowing they need to look in the other book*
    - Hosts and Guests
    - Interfaces
    - Capabilities
    - Commands and Reactors
- Language Support - *depending on what is available when*
  - C
  - C# and .NET
  - Go
  - Rust
- Reference
  - HTTP
    - Usage guide
    - Common types and functions - *may want to divide a different way, e.g. types at top level and associated functions as part of each type (cf. how the `fields` type and functions are presented in the WIT) - lots of experimentation to do here...*
      - Types
      - Functions
    - Incoming HTTP handler
      - Types
      - Functions
    - Outgoing HTTP
      - Types
      - Functions
    - HTTP proxy
      - Types
      - Functions
- Examples
- Additional References

