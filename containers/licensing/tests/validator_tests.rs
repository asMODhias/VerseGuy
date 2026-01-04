use base64::{Engine as _, engine::general_purpose};
use verseguy_licensing::validate_license;
use verseguy_licensing::validator::create_signed_token;

#[test]
fn create_and_validate_token() {
    let secret = b"supersecretkey";
    let payload = serde_json::json!({"id":"user1","tier":"Pro","exp":4102444800i64}); // year 2100
    let payload_b64 = general_purpose::STANDARD.encode(payload.to_string().as_bytes());
    let token = create_signed_token(&payload_b64, secret).unwrap();

    // validate with now < exp
    let ok = validate_license(&token, secret, 1609459200).unwrap(); // 2021-01-01
    assert!(ok);

    // expired
    let expired = validate_license(&token, secret, 4202444800i64).unwrap();
    assert!(!expired);
}
