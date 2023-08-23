cargo_component_bindings::generate!();
use bindings::exports::docs::calculator::add::Add;

struct Component;

impl Add for Component {
    fn add(a: u32, b: u32) -> u32 {
        a + b
    }
}
