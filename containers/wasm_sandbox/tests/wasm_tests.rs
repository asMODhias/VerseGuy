use verseguy_wasm_sandbox::run_add_module;

#[test]
fn wasm_adds_correctly() {
    let res = match run_add_module(3, 4) {
        Ok(v) => v,
        Err(e) => panic!("run_add_module failed: {}", e),
    };
    assert_eq!(res, 7);
}
