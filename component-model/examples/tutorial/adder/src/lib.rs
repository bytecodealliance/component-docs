#[allow(warnings)]
mod bindings;

// Separating out the interface puts it in a sub-module
use bindings::exports::docs::adder::add::Guest;

struct Component;

impl Guest for Component {
    fn add(x: u32, y: u32) -> u32 {
        x + y
    }
}

bindings::export!(Component with_types_in bindings);
