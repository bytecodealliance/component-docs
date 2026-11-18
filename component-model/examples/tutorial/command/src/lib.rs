mod bindings {
    //! This module contains generated code for implementing
    //! the `app` world in `wit/world.wit`.

    use super::Component;

    wit_bindgen::generate!({
        world: "app",
    });

    export!(Component);
}

use std::fmt;

use clap::Parser;

use bindings::docs::calculator::calculate::{self, Op};

fn parse_operator(op: &str) -> anyhow::Result<Op> {
    match op {
        "add" => Ok(Op::Add),
        _ => anyhow::bail!("Unknown operation: {}", op),
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
        }
    }
}

/// A CLI for executing mathematical expressions
/// using WebAssembly
#[derive(Parser)]
#[clap(name = "calculator", version = env!("CARGO_PKG_VERSION"))]
struct Command {
    /// The first operand
    x: u32,
    /// The second operand
    y: u32,
    /// Expression operator
    #[clap(value_parser = parse_operator)]
    op: Op,
}

impl Command {
    fn run(self) {
        let res = calculate::eval_expression(self.op, self.x, self.y);
        println!("{} {} {} = {res}", self.x, self.op, self.y);
    }
}

/// Struct off of which the implementation will hang
///
/// The name of this struct is not significant.
struct Component;

/// Implementation of the `wasi:cli/run` export, which is the
/// equivalent of a `main` function for a command component.
impl bindings::exports::wasi::cli::run::Guest for Component {
    fn run() -> Result<(), ()> {
        match Command::try_parse() {
            Ok(command) => {
                command.run();
                Ok(())
            }
            Err(err) => {
                // Printing can only fail if stdout/stderr are closed
                let _ = err.print();
                // clap reports `--help` and `--version` as "errors",
                // but they should not cause a failing exit code
                if err.use_stderr() { Err(()) } else { Ok(()) }
            }
        }
    }
}
