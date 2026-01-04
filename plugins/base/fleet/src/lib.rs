pub fn fleet_example() -> &'static str {
    "fleet plugin"
}

#[cfg(test)]
mod tests {
    use super::fleet_example;

    #[test]
    fn fleet_smoke() {
        assert_eq!(fleet_example(), "fleet plugin");
    }
}
