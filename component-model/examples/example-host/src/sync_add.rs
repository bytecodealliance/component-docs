use std::path::PathBuf;

use anyhow::Context;
use wasmtime::component::{Component, Linker};
use wasmtime::{Engine, Store};

use crate::state::States;

mod bindings {
    wasmtime::component::bindgen!({
        path: "../tutorial/wit/adder/world.wit",
        world: "adder",
        async: false
    });
}

pub fn add(path: PathBuf, x: u32, y: u32) -> wasmtime::Result<u32> {
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
    let instance = bindings::Adder::instantiate(&mut store, &component, &linker)
        .context("Failed to instantiate the example world")?;
    instance
        .docs_adder_add()
        .call_add(&mut store, x, y)
        .context("Failed to call add function")
}
