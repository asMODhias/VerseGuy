use anyhow::Result;

/// Very small policy evaluator: policy is a simple string like "role:admin" or "allow_all".
pub fn evaluate_policy(policy: &str, roles: &[&str]) -> Result<bool> {
    if policy == "allow_all" {
        return Ok(true);
    }

    if let Some(want) = policy.strip_prefix("role:") {
        for r in roles {
            if r == &want {
                return Ok(true);
            }
        }
        return Ok(false);
    }

    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_allow_all() -> Result<()> {
        assert!(evaluate_policy("allow_all", &[])?);
        Ok(())
    }

    #[test]
    fn test_role_match() -> Result<()> {
        assert!(evaluate_policy("role:admin", &["user", "admin"])?);
        assert!(!evaluate_policy("role:owner", &["user", "admin"])?);
        Ok(())
    }
}
