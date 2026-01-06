# RCGEN ring bump / patch plan

Goal: eliminate the transitive `ring 0.16.20` advisory (RUSTSEC-2025-0009) introduced by `rcgen v0.11.3` by either upstream upgrade or a temporary patch.

Options (recommended order):

1) Upgrade upstream dependencies
- Check if newer `libp2p` / `libp2p-tls` / `libp2p-quic` versions depend on `rcgen` that uses `ring >= 0.17.12`.
- If available, bump our dependency and run full test + audit.

2) Patch rcgen via a fork (short-term workaround)
- Fork `https://github.com/est31/rcgen` and create branch `bump-ring-0.17`.
- In the fork, update `rcgen/Cargo.toml` to depend on `ring = "^0.17"` and update any code that needs to change due to ring API changes.
- Run `cargo test` in the rcgen fork and fix compile/test failures.
- In this repo, add a temporary patch to `Cargo.toml`:

```toml
[patch.crates-io]
rcgen = { git = "https://github.com/<your-org>/rcgen", branch = "bump-ring-0.17" }
```

- Run `cargo update -p rcgen` (or `cargo metadata`) and then `cargo audit` and full `cargo test` in workspace.
- If all green, push branch and open a PR to upstream `rcgen` with the same changes; keep the patch in our repo until upstream releases a version that fixes the advisory.

3) Long-term: Upstream merge & revert patch
- After upstream merges and publishes a release, remove `[patch.crates-io]` entry and update to the published version.

Notes / Caveats:
- Bumping `ring` across major versions may require code changes in `rcgen`. Tests in `rcgen` must pass.
- Keep the security team / PR reviewers in the loop; include cargo-audit outputs and motivating text in the PR.

Commands (local workflow):

- Fork & clone:
  git clone https://github.com/<your-org>/rcgen
  cd rcgen
  git checkout -b bump-ring-0.17

- Update Cargo.toml, adjust code and run tests:
  cargo test

- In this repo (root), add patch to Cargo.toml and then:
  cargo update -p rcgen
  cargo audit
  cargo test --all

- After upstream PR is merged and release is available:
  Remove patch from Cargo.toml and update Cargo.lock to use upstream release.

If you'd like, I can:
- Draft the fork changes (bump dep and attempt compile) locally to see the required code changes, or
- Prepare the branch template and PR text for you to push to a fork and open on GitHub.
