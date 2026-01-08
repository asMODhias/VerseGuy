use serde::{Deserialize, Serialize};

/// Ship type value object (id, name, slots)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ShipType {
    pub id: String,
    pub name: String,
    pub slots: u32,
}

impl ShipType {
    pub fn new(id: String, name: String, slots: u32) -> Self {
        Self { id, name, slots }
    }
}

/// Loadout value object
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Loadout {
    pub id: String,
    pub name: String,
    pub components: Vec<String>,
}

impl Loadout {
    pub fn new(id: String, name: String, components: Vec<String>) -> Self {
        Self {
            id,
            name,
            components,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shiptype_basic() {
        let st = ShipType::new("st-1".into(), "Fighter".into(), 3);
        assert_eq!(st.id, "st-1");
        assert_eq!(st.name, "Fighter");
        assert_eq!(st.slots, 3);
    }

    #[test]
    fn loadout_basic() {
        let lo = Loadout::new(
            "l-1".into(),
            "Alpha".into(),
            vec!["gun".into(), "shield".into()],
        );
        assert_eq!(lo.id, "l-1");
        assert_eq!(lo.name, "Alpha");
        assert_eq!(lo.components.len(), 2);
    }
}
