# Compliance Check — Guidance

This page outlines the compliance checks that should run as part of CI/CD.

Checks to include:
- Legal doc presence checks (ToS, PrivacyPolicy, DPA for regions used)
- Linting of privacy/ToS text for required clauses
- Automated checks for telemetry opt-in flags and PII handling
- Export & deletion API smoke tests to ensure GDPR/CCPA flows are functional

Notes:
- Compliance checks rely on legal team input for clause validation.
- Some checks may be manual or semi-automated until legal texts are finalized.