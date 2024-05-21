mod bindings;

use bindings::exports::component_book::calculator::calculate::{Guest, Op};

// Bring the imported add function into scope
use bindings::component_book::adder::add::add;

struct Component;

impl Guest for Component {
    fn eval_expression(op: Op, x: u32, y: u32) -> u32 {
        match op {
            Op::Add => add(x, y),
        }
    }
}

bindings::export!(Component with_types_in bindings);
