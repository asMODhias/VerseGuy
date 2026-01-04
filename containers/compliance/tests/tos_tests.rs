use verseguy_compliance::validate_tos_acceptance;
use serde_json::json;

#[test]
fn valid_tos() {
    let payload = json!({"user_id":"u1","accepted_at": 1609459200, "version":"1.0"});
    let s = payload.to_string();
    let t = validate_tos_acceptance(&s).unwrap();
    assert_eq!(t.user_id, "u1");
}
