use anyhow::Result;

/// Policy language (simple expression syntax):
/// - "allow_all" → allows everyone
/// - "role:<name>" → true if user has role with name `<name>`
/// - "any(expr,expr,...)" → true if any subexpression is true
/// - "all(expr,expr,...)" → true if all subexpressions are true
/// - "not(expr)" → negation
pub fn evaluate_policy(policy: &str, roles: &[&str]) -> Result<bool> {
    // Default checker returns false for license expressions (no license context)
    evaluate_policy_with_checker(policy, roles, &|_feat: &str| Ok(false))
}

/// Evaluate policy with an external license-feature checker callback.
pub fn evaluate_policy_with_checker<F>(policy: &str, roles: &[&str], checker: &F) -> Result<bool>
where
    F: Fn(&str) -> Result<bool>,
{
    let p = policy.trim();
    if p == "allow_all" {
        return Ok(true);
    }

    if let Some(want) = p.strip_prefix("role:") {
        for r in roles {
            if r == &want {
                return Ok(true);
            }
        }
        return Ok(false);
    }

    if let Some(feat) = p.strip_prefix("license:") {
        return checker(feat);
    }

    if let Some(inner) = p.strip_prefix("any(") {
        if let Some(inner) = inner.strip_suffix(")") {
            let parts = split_top_level_commas(inner);
            for part in parts {
                if evaluate_policy_with_checker(part, roles, checker)? {
                    return Ok(true);
                }
            }
            return Ok(false);
        }
    }

    if let Some(inner) = p.strip_prefix("all(") {
        if let Some(inner) = inner.strip_suffix(")") {
            let parts = split_top_level_commas(inner);
            for part in parts {
                if !evaluate_policy_with_checker(part, roles, checker)? {
                    return Ok(false);
                }
            }
            return Ok(true);
        }
    }

    if let Some(inner) = p.strip_prefix("not(") {
        if let Some(inner) = inner.strip_suffix(")") {
            return Ok(!evaluate_policy_with_checker(inner, roles, checker)?);
        }
    }

    // Unknown expression → deny by default
    Ok(false)
}

fn split_top_level_commas(s: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut depth = 0usize;
    let mut start = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                parts.push(s[start..i].trim());
                start = i + 1;
            }
            _ => {}
        }
    }
    if start <= s.len() {
        parts.push(s[start..].trim());
    }
    parts
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

    #[test]
    fn test_any_all_not_combinations() -> Result<()> {
        let roles = &["user", "admin"];
        assert!(evaluate_policy("any(role:guest, role:admin)", roles)?);
        assert!(evaluate_policy("all(role:user, role:admin)", roles)?);
        assert!(!evaluate_policy("all(role:user, role:owner)", roles)?);
        assert!(evaluate_policy("not(role:owner)", roles)?);
        assert!(!evaluate_policy("not(role:admin)", roles)?);
        // Nested
        assert!(evaluate_policy("any(all(role:user, role:admin), role:owner)", roles)?);
        Ok(())
    }
}
