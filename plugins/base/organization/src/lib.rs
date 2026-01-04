pub mod types;

pub fn example() -> String {
    "organization plugin loaded".to_string()
}

#[cfg(test)]
mod tests {
    use super::types::{Member, Organization};

    #[test]
    fn types_roundtrip() {
        let org = Organization {
            id: "org1".into(),
            name: "Org".into(),
            tag: "ORG".into(),
            member_count: 5,
        };
        let mem = Member {
            id: "m1".into(),
            org_id: "org1".into(),
            handle: "bob".into(),
            rank_id: "r1".into(),
        };
        assert_eq!(org.id, "org1");
        assert_eq!(mem.org_id, "org1");
    }
}
