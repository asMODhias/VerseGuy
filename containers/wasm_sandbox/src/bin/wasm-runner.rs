use anyhow::Result;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let (a, b) = if args.len() >= 3 {
        (args[1].parse::<i32>()?, args[2].parse::<i32>()?)
    } else {
        (3, 4)
    };
    // Reuse library implementation which uses current wasmtime API
    let res = verseguy_wasm_sandbox::run_add_module(a, b)?;
    println!("wasm-add -> {} + {} = {}", a, b, res);
    std::process::exit(0);
}
