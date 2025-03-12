use clap::Parser;
use std::path::PathBuf;

mod async_add;
mod state;
mod sync_add;

/// A CLI for executing WebAssembly components that
/// implement the `example` world.
#[derive(Parser)]
#[clap(name = "add-host", version = env!("CARGO_PKG_VERSION"))]
struct AddApp {
    /// The first operand
    x: u32,
    /// The second operand
    y: u32,
    /// The path to the component.
    #[clap(value_name = "COMPONENT_PATH")]
    component: PathBuf,
}

impl AddApp {
    async fn run(self) -> anyhow::Result<()> {
        let sum1 = async_add::add(self.component.clone(), self.x, self.y).await?;
        let sum2 = sync_add::add(self.component, self.x, self.y)?;
        assert_eq!(sum1, sum2);
        println!("{} + {} = {sum1}", self.x, self.y);
        Ok(())
    }
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    AddApp::parse().run().await
}
