#[allow(warnings)]
mod bindings;

use bindings::exports::docs::adder::add::Guest;

struct Component;

impl Guest for Component {
    fn add(x: u32, y: u32) -> u32 {
        a + b
    }
}

bindings::export!(Component with_types_in bindings);
