use plugins_adapter_scunpacked::parse_ships_from_value;
use verseguy_test_utils::must;

#[test]
fn parse_array_of_ships() {
    let v = must(serde_json::from_str(r#"[{"id":"a","name":"A"},{"id":"b","name":"B","role":"r"}]"#));
    let ships = must(parse_ships_from_value(&v));
    assert_eq!(ships.len(), 2);
}

#[test]
fn parse_single_object_ship() {
    let v = must(serde_json::from_str(r#"{"id":"x","name":"X"}"#));
    let ships = must(parse_ships_from_value(&v));
    assert_eq!(ships.len(), 1);
    assert_eq!(ships[0].id, "x");
}

#[test]
fn parse_mapping_of_ships() {
    let v = must(serde_json::from_str(r#"{"origin-100i":{"id":"origin-100i","name":"Origin 100i"},"constellation":{"id":"constellation","name":"Aegis Constellation"}}"#));
    let ships = must(parse_ships_from_value(&v));
    assert_eq!(ships.len(), 2);
}
