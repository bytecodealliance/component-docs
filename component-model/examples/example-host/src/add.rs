use anyhow::Context;
use std::path::PathBuf;
use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::preview2::{command, Table, WasiCtx, WasiCtxBuilder, WasiView};

wasmtime::component::bindgen!({
    path: "add.wit",
    world: "example",
    async: true
});

pub async fn add(path: PathBuf, x: i32, y: i32) -> wasmtime::Result<i32> {
    let mut config = Config::default();
    config.wasm_component_model(true);
    config.async_support(true);
    let engine = Engine::new(&config)?;
    let mut linker = Linker::new(&engine);

    // Add the command world (aka WASI CLI) to the linker
    command::add_to_linker(&mut linker).context("Failed to link command world")?;
    let wasi_view = ServerWasiView::new();
    let mut store = Store::new(&engine, wasi_view);

    let component = Component::from_file(&engine, path).context("Component file not found")?;

    let (instance, _) = Example::instantiate_async(&mut store, &component, &linker)
        .await
        .context("Failed to instantiate the example world")?;
    instance
        .call_add(&mut store, x, y)
        .await
        .context("Failed to call add function")
}

struct ServerWasiView {
    table: Table,
    ctx: WasiCtx,
}

impl ServerWasiView {
    fn new() -> Self {
        let table = Table::new();
        let ctx = WasiCtxBuilder::new().inherit_stdio().build();

        Self { table, ctx }
    }
}

impl WasiView for ServerWasiView {
    fn table(&self) -> &Table {
        &self.table
    }

    fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }

    fn ctx(&self) -> &WasiCtx {
        &self.ctx
    }

    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}
