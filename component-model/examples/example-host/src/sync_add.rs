use std::path::PathBuf;

use anyhow::Context;
use wasmtime::component::{Component, Linker};
use wasmtime::{Engine, Store};
use wasmtime_wasi;

use crate::state::States;

mod bindings {
    wasmtime::component::bindgen!({
        path: "../tutorial/wit/adder/world.wit",
        world: "adder",
        async: false
    });
}

/// Perform a add operation for a given WebAssembly component
///
/// This operation happens synchronously (as opposed to asynchronously
/// powered by an async runtime like `tokio` or `async-std`).
///
/// # Arguments
///
/// * `path` - Path to the Wasm component bytes
/// * `x` - The left hand side of the addition
/// * `y` - The right hand side of the addition
///
pub fn add(path: PathBuf, x: u32, y: u32) -> wasmtime::Result<u32> {
    // Construct engine
    let engine = Engine::default();

    // Construct component
    let component = Component::from_file(&engine, path).context("Component file not found")?;

    // Construct store for storing running states of the component
    let wasi_view = States::new();
    let mut store = Store::new(&engine, wasi_view);

    // Construct linker for linking interfaces.
    let mut linker = Linker::new(&engine);

    // Add wasi exports to linker to support I/O (as in `wasi:io`) interfaces
    // see: https://github.com/WebAssembly/wasi-io
    wasmtime_wasi::p2::add_to_linker_sync(&mut linker).expect("Could not add wasi to linker");

    // Instantiate the component as an instance of the `adder` world,
    // with the generated bindings
    let instance = bindings::Adder::instantiate(&mut store, &component, &linker)
        .context("Failed to instantiate the example world")?;

    // Call the add function on instance
    instance
        .docs_adder_add()
        .call_add(&mut store, x, y)
        .context("calling add function")
}
