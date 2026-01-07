use verseguy_compliance::validate_tos_acceptance;
use verseguy_test_utils::must;

#[test]
fn valid_tos() {
    let s = r#"{"user_id":"u1","accepted_at":1609459200,"version":"1.0"}"#.to_string();
    let t = must(validate_tos_acceptance(&s));
    assert_eq!(t.user_id, "u1");
}
