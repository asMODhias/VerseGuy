pub fn adapters_example() -> &'static str {
    "adapters plugin"
}

#[cfg(test)]
mod tests {
    use super::adapters_example;

    #[test]
    fn adapters_smoke() {
        assert_eq!(adapters_example(), "adapters plugin");
    }
}
