//! Plugin template crate — use as basis for new plugins

pub fn template_name() -> &'static str {
    "template plugin"
}

#[cfg(test)]
mod tests {
    use super::template_name;

    #[test]
    fn template_smoke() {
        assert_eq!(template_name(), "template plugin");
    }
}
