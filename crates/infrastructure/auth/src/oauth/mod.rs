pub mod provider;

// OAuth provider abstraction

pub trait OAuthProvider {
    /// Exchange an authorization code for a user identifier or profile
    fn exchange_code(&self, code: &str) -> anyhow::Result<String>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    struct Dummy;
    impl OAuthProvider for Dummy {
        fn exchange_code(&self, _code: &str) -> anyhow::Result<String> {
            Ok("dummy-user".to_string())
        }
    }

    #[test]
    fn dummy_exchange() -> Result<()> {
        let p = Dummy;
        let r = p.exchange_code("code");
        assert!(r.is_ok());
        Ok(())
    }
}