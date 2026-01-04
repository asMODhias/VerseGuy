use anyhow::Result;
use std::env;
use wasmtime::*;
use wasmtime_wasi::WasiCtxBuilder;

fn run_add_module(a: i32, b: i32) -> Result<i32> {
    // WAT for simple add function
    let wat = r#"
    (module
      (func $add (param i32 i32) (result i32)
        local.get 0
        local.get 1
        i32.add)
      (export "add" (func $add)))
    "#;

    let engine = Engine::default();
    let module = Module::new(&engine, wat)?;
    let mut linker = Linker::new(&engine);
    let wasi = WasiCtxBuilder::new().inherit_stdio().build();
    let mut store = Store::new(&engine, wasi);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;
    let instance = linker.instantiate(&mut store, &module)?;
    let add = instance.get_typed_func::<(i32, i32), i32>(&mut store, "add")?;
    let res = add.call(&mut store, (a, b))?;
    Ok(res)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let (a, b) = if args.len() >= 3 {
        (args[1].parse::<i32>()?, args[2].parse::<i32>()?)
    } else {
        (3, 4)
    };
    let res = run_add_module(a, b)?;
    println!("wasm-add -> {} + {} = {}", a, b, res);
    std::process::exit(0);
}
