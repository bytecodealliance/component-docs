cargo_component_bindings::generate!();

use bindings::exports::docs::calculator::calculate::{Guest, Op};

// Bring the imported add function into scope
use bindings::docs::calculator::add::add;

struct Component;

impl Guest for Component {
    fn eval_expression(op: Op, x: u32, y: u32) -> u32 {
        match op {
            Op::Add => add(x, y),
        }
    }
}
