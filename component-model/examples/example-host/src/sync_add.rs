use crate::state::States;
use anyhow::Context;
use std::path::PathBuf;
use wasmtime::component::{bindgen, Component, Linker};
use wasmtime::{Engine, Store};

bindgen!({
    path: "add.wit",
    world: "example",
    async: false
});

pub fn add(path: PathBuf, x: i32, y: i32) -> wasmtime::Result<i32> {
    // Construct engine
    let engine = Engine::default();
    // Construct component
    let component = Component::from_file(&engine, path).context("Component file not found")?;
    // Construct store for storing running states of the component
    let wasi_view = States::new();
    let mut store = Store::new(&engine, wasi_view);
    // Construct linker for linking interfaces.
    // For this simple adder component, no need to link additional interfaces.
    let linker = Linker::new(&engine);
    let instance = Example::instantiate(&mut store, &component, &linker)
        .context("Failed to instantiate the example world")?;
    instance
        .call_add(&mut store, x, y)
        .context("Failed to call add function")
}