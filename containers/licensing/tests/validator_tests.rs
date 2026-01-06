use base64::{Engine as _, engine::general_purpose};
use verseguy_licensing::validate_license;
use verseguy_licensing::validator::create_signed_token;

#[test]
fn create_and_validate_token() {
    let secret = b"supersecretkey";
    // Construct payload without using the `json!` macro to avoid disallowed `unwrap` in macro expansion
    let payload = r#"{"id":"user1","tier":"Pro","exp":4102444800}"#; // year 2100
    let payload_b64 = general_purpose::STANDARD.encode(payload.as_bytes());
    let token = match create_signed_token(&payload_b64, secret) {
        Ok(t) => t,
        Err(e) => panic!("create_signed_token failed: {}", e),
    };

    // validate with now < exp
    let ok = match validate_license(&token, secret, 1609459200) {
        Ok(v) => v,
        Err(e) => panic!("validate_license failed: {}", e),
    }; // 2021-01-01
    assert!(ok);

    // expired
    let expired = match validate_license(&token, secret, 4202444800i64) {
        Ok(v) => v,
        Err(e) => panic!("validate_license failed: {}", e),
    };
    assert!(!expired);
}
