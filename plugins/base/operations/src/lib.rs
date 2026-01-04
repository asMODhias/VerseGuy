pub fn ops_example() -> &'static str {
    "operations plugin"
}

#[cfg(test)]
mod tests {
    use super::ops_example;

    #[test]
    fn ops_smoke() {
        assert_eq!(ops_example(), "operations plugin");
    }
}
