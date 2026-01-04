use anyhow::Result;
use wasmtime::*;

pub fn run_add_module(a: i32, b: i32) -> Result<i32> {
    // Tiny WAT module that exports an 'add' function
    let wat = r#"(module
        (func $add (param i32 i32) (result i32)
            local.get 0
            local.get 1
            i32.add)
        (export "add" (func $add))
    )"#;

    let engine = Engine::default();
    let module = Module::new(&engine, wat)?;
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;
    let add = instance.get_typed_func::<(i32, i32), i32>(&mut store, "add")?;
    let res = add.call(&mut store, (a, b))?;
    Ok(res)
}
