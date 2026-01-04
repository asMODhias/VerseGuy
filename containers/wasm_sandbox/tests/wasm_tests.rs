use verseguy_wasm_sandbox::run_add_module;

#[test]
fn wasm_adds_correctly() {
    let res = run_add_module(3, 4).unwrap();
    assert_eq!(res, 7);
}