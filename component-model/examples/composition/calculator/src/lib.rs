#[allow(warnings)]
mod bindings;

use bindings::exports::component::calculator::calculate::{Guest, Op};
use bindings::docs::adder::add::add;
use bindings::docs::subtractor::sub::sub;
struct Component;

impl Guest for Component {
    fn eval_expression(op: Op, x: u32, y: u32) -> u32{
        match op {
            Op::Add => add(x, y),
            Op::Sub => sub(x, y),
        }
    }
}

bindings::export!(Component with_types_in bindings);
