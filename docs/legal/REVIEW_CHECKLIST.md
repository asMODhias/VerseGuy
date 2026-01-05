# Legal Review Checklist ✅

Use this checklist when preparing a legal document (ToS, Privacy Policy, DPA) for publishing.

- [ ] Identify document type (`tos`, `privacy`, `dpa`, `third_party_notices`).
- [ ] Fill metadata: `version`, `author`, `date`, `change_summary`.
- [ ] Legal sign-off: **Legal Counsel** reviewed and approved.
- [ ] Product sign-off: Product Manager reviewed for UX impact.
- [ ] Privacy sign-off: Privacy Officer reviewed retention & export flows.
- [ ] Security sign-off: Security reviewed any enforcement points.
- [ ] Localization: if needed, translations are queued and labeled with the source version.
- [ ] Audit entry: create an audit record for the publish event and keep the admin rationale.
- [ ] Post-publish monitoring: set a review reminder and plan for customer communication (UI banners, release notes).

> Keep approvals as records (ticket links, signatures) and reference them in the Admin API call `reason` or the admin audit.
