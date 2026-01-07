This PR updates `rcgen` to depend on `ring = "^0.17"` in order to address advisory **RUSTSEC-2025-0009** (Some AES functions may panic when overflow checking is enabled).

Summary:
- Bump `ring` dependency to `^0.17`.
- Enable `yasna/time` feature where needed for tests.

Verification:
- Ran `cargo check -p rcgen` locally.
- Ran `cargo audit` in the workspace; RUSTSEC-2025-0009 is no longer present.

Notes:
- This is a minimal, targeted change to remove the vulnerable transitive dependency. Upstream PRs for broader API changes (if desired) can follow.
- See `docs/security/RCGEN_PATCH_PLAN.md` in the repo for the proposed long-term process.

Requesting review from upstream maintainers and marking as security-related. If accepted and merged, we will remove the temporary `[patch.crates-io]` entry in our workspace.
