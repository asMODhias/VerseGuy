pub fn pro_example() -> &'static str {
    "pro plugin"
}

#[cfg(test)]
mod tests {
    use super::pro_example;

    #[test]
    fn pro_smoke() {
        assert_eq!(pro_example(), "pro plugin");
    }
}
