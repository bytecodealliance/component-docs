use std::path::PathBuf;

use anyhow::Context;
use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Store};

use crate::state::States;

mod bindings {
    //! Generated code for the
    wasmtime::component::bindgen!({
        path: "add.wit",
        world: "adder",
        async: true
    });
}

/// Perform the add operation for a given WebAssembly component
///
/// This operation asynchronously (as opposed to synchronously
/// without an async runtime like `tokio`).
///
/// # Arguments
///
/// * `path` - Path to the Wasm component bytes
/// * `x` - The left hand side of the addition
/// * `y` - The right hand side of the addition
///
pub async fn add(path: PathBuf, x: u32, y: u32) -> wasmtime::Result<u32> {
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
    let mut linker = Linker::new(&engine);

    // Add wasi exports to linker to support I/O (as in `wasi:io`) interfaces
    // see: https://github.com/WebAssembly/wasi-io
    wasmtime_wasi::add_to_linker_async(&mut linker)?;

    // Instantiate the component as an instance of the `adder` world,
    // with the generated bindings
    let instance = bindings::Adder::instantiate_async(&mut store, &component, &linker)
        .await
        .context("Failed to instantiate the example world")?;

    // Call the add function on instance
    instance
        .docs_adder_add()
        .call_add(&mut store, x, y)
        .await
        .context("calling add function")
}
