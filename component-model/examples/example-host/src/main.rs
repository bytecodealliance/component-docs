mod add;
use clap::Parser;
use std::path::PathBuf;

/// A CLI for executing WebAssembly components that
/// implement the `example` world.
#[derive(Parser)]
#[clap(name = "add-host", version = env!("CARGO_PKG_VERSION"))]
struct AddApp {
    /// The first operand
    x: i32,
    /// The second operand
    y: i32,
    /// The path to the component.
    #[clap(value_name = "COMPONENT_PATH")]
    component: PathBuf,
}

impl AddApp {
    async fn run(self) -> anyhow::Result<()> {
        let sum = add::add(self.component, self.x, self.y).await?;
        println!("{} + {} = {sum}", self.x, self.y);
        Ok(())
    }
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    AddApp::parse().run().await
}
