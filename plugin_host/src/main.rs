use wasmtime::{Engine, Result, Store, Config};
use wasmtime::component::{ResourceTable, Linker, bindgen, Component};
use wasmtime_wasi::{WasiCtx, WasiView, WasiCtxBuilder};

#[tokio::main]
async fn main() -> Result<()> {
    let mut config = Config::new();
    config.async_support(true);
    config.wasm_component_model(true);
    config.debug_info(true);
    let engine = Engine::new(&config)?;

    bindgen!({world: "new-world", path: "../plugin_api/wit/world.wit", async: true});

    let mut linker = Linker::new(&engine);

    // Add all the WASI extensions to the linker
    wasmtime_wasi::add_to_linker_async(&mut linker)?;

    //NewWorld::add_to_linker(&mut linker, |state: &mut MyState| state)?;

    // ... configure `builder` more to add env vars, args, etc ...
    let mut builder = WasiCtxBuilder::new();
    builder.inherit_stdio();
    let mut store = Store::new(
        &engine,
        MyState {
            ctx: builder.build(),
            table: ResourceTable::new(),
        },
    );
    let component = Component::from_file(&engine, "./plugins/custom_plugin.wasm")?;
    let (bindings, _) = NewWorld::instantiate_async(&mut store, &component, &linker).await?;
    // Here our `greet` function takes one name parameter,
    // but in the Wasmtime embedding API the first argument is always a `Store`.
    let greeting = bindings.call_greeting(&mut store, "Ben").await?;
    println!("{greeting}");
    Ok(())
}

struct MyState {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl WasiView for MyState {
    fn table(&mut self) -> &mut ResourceTable { &mut self.table }
    fn ctx(&mut self) -> &mut WasiCtx { &mut self.ctx }
}

// Imports into the world, like the `name` import for this world, are
// satisfied through traits.
// impl NewWorldImports for MyState {
//     fn name(&mut self) -> String {
//         self.name.clone()
//     }
// }