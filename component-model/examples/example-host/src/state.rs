use wasmtime::component::ResourceTable;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};

pub struct States {
    table: ResourceTable,
    ctx: WasiCtx,
}

impl States {
    pub fn new() -> Self {
        let table = ResourceTable::new();
        let ctx = WasiCtxBuilder::new().build();
        Self { table, ctx }
    }
}

impl WasiView for States {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}