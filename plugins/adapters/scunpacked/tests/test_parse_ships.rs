use serde_json::json;
use plugins_adapter_scunpacked::parse_ships_from_value;

#[test]
fn parse_array_of_ships() {
    let v = json!([
        {"id":"a","name":"A"},
        {"id":"b","name":"B","role":"r"}
    ]);
    let ships = parse_ships_from_value(&v).unwrap();
    assert_eq!(ships.len(), 2);
}

#[test]
fn parse_single_object_ship() {
    let v = json!({"id":"x","name":"X"});
    let ships = parse_ships_from_value(&v).unwrap();
    assert_eq!(ships.len(), 1);
    assert_eq!(ships[0].id, "x");
}

#[test]
fn parse_mapping_of_ships() {
    let v = json!({"origin-100i": {"id":"origin-100i","name":"Origin 100i"}, "constellation": {"id":"constellation","name":"Aegis Constellation"}});
    let ships = parse_ships_from_value(&v).unwrap();
    assert_eq!(ships.len(), 2);
}
