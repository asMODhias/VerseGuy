use crate::entity::Fleet;

/// Lightweight aggregate helpers (placeholder)
impl Fleet {
    /// Return number of ships
    pub fn ship_count(&self) -> usize {
        self.ships.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::entity::Fleet;

    #[test]
    fn ship_count_returns_zero_for_new() {
        let f = Fleet::new("f1".to_string(), "TestFleet".to_string());
        assert_eq!(f.ship_count(), 0);
    }
}
