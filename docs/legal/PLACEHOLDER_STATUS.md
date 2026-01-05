# Legal Placeholder Status

This file tracks legal placeholders that require Legal team input before release.

Last updated: 2026-01-04

## Open items

- legal/ThirdPartyNotices.md
  - Placeholder: "(Placeholder) — run `cargo-about` or equivalent to generate an up-to-date list of third-party attributions."
  - Owner: Legal
  - Action: Run `cargo-about` (or equivalent), add third-party notice output, verify licenses and attributions.

- legal/CountrySpecific/UK.md
  - Placeholder: country-specific wording required.
  - Owner: Legal
  - Action: Provide UK-specific DPA/retention wording and references.

- legal/CountrySpecific/EU.md
  - Placeholder: country-specific wording required.
  - Owner: Legal
  - Action: Provide EU-specific clauses / references and ensure DPA compliance text.

- legal/CountrySpecific/US.md
  - Placeholder: state-level clauses and retention windows.
  - Owner: Legal
  - Action: Provide US-specific language and retention policy details.

## Suggested process
1. Legal provides final text as PRs targeting the `legal/` files above.
2. Add small unit/integration tests that assert the presence of required clauses for major jurisdictions (optional).
3. Update release checklist to verify these files are present and signed off before any public release.

## Notes
- These placeholders are **blocking** for a compliance-enabled release and should be prioritized prior to a public release.
- If you want, I can open GitHub Issues for each placeholder and draft suggested text/PR templates for Legal to use.
