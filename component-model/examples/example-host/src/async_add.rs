use anyhow::Context;
use std::path::PathBuf;
use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};
use crate::state::States;

bindgen!({
    path: "add.wit",
    world: "example",
    async: true
});

pub async fn add(path: PathBuf, x: i32, y: i32) -> wasmtime::Result<i32> {
    // Construct engine
    let mut config = Config::default();
    config.async_support(true);
    let engine = Engine::new(&config)?;
    // Construct component
    let component = Component::from_file(&engine, path).context("Component file not found")?;
    // Construct store for storing running states of the component
    let wasi_view = States::new();
    let mut store = Store::new(&engine, wasi_view);
    // Construct linker for linking interfaces.
    // For this simple adder component, no need to manually link additional interfaces.
    let mut linker = Linker::new(&engine);
    // Add wasi exports to linker to support io interfaces
    wasmtime_wasi::add_to_linker_async(&mut linker)?;
    let instance = Example::instantiate_async(&mut store, &component, &linker)
        .await
        .context("Failed to instantiate the example world")?;
    instance
        .call_add(&mut store, x, y)
        .await
        .context("Failed to call add function")
}