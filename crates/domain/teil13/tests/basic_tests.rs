use verseguy_domain_teil13::entity::Teil13Aggregate;

#[test]
fn create_has_name_and_id() {
    let a = Teil13Aggregate::new("test");
    assert_eq!(a.name, "test");
    assert!(!a.id.to_string().is_empty());
}
