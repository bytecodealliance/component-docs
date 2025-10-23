mod bindings {
    //! This module contains generated code for implementing
    //! the `adder` world in `wit/world.wit`.
    //!
    //! The `path` option is actually not required,
    //! as by default `wit_bindgen::generate` will look
    //! for a top-level `wit` directory and use the files
    //! (and interfaces/worlds) there-in.

    // The line below will be expanded as Rust code containing
    wit_bindgen::generate!({
        path: "wit/adder/world.wit",
    });

    // In the lines below we use the generated `export!()` macro re-use and
    use super::AdderComponent;
    export!(AdderComponent);
}

/// Struct off of which the implementation will hang
///
/// The name of this struct is not significant.
struct AdderComponent;

impl bindings::exports::docs::adder::add::Guest for AdderComponent {
    fn add(x: u32, y: u32) -> u32 {
        x + y
    }
}
