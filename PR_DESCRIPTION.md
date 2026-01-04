PR Title: chore/init-repo-scripts — split initial commit, add CTest, scripts, audit, wasm, compliance, and tests

Summary:
- Split the previous large initial commit into focused commits (scripts, CMake/CI, audit, wasm, compliance, master-server tests, docs).
- Added CTest integration and updated CI to call `ctest --output-on-failure -C Release`.
- Added cross-platform scripts: `scripts/build.sh` and `scripts/test.sh` (WSL/Git-Bash friendly) and updated `scripts/ci-local.ps1` to run `ctest` when available.
- Implemented `verseguy_audit` crate (append-only SHA256 hash chain) and tests.
- Implemented GDPR export/delete functions in `containers/compliance` and tests.
- Added `verseguy_wasm_sandbox` POC using Wasmtime and tests.
- Added launcher headless mode (`--no-gui` / `VERSEGUY_HEADLESS`) and a launcher smoke test registered with CTest.
- Added master-server E2E test for register → login → publish → verify flow.
- Updated documentation: `VERSEGUY_COPILOT_COMPLETE.md` and `VERSE_GUY_V2_ULTIMATE_SPECIFICATION.md`, and tidy `TODO.md`.

Tests:
- Ran `cargo test` for modified crates (audit, wasm_sandbox, compliance, master-server E2E) — all pass locally.
- Ran `cmake` build and `ctest` locally — C++ tests discovered and run via CTest.

Notes & Next Steps (requires approval):
- I will not push or open a PR without your explicit approval. If you approve, I will push branch `chore/init-repo-scripts` and create a PR with this description.
- Optionally: I can add pre-commit hooks (format/check tests) and a short changelog entry in `README.md` or `docs/`.

Files / Commits (high-level):
- `chore(scripts): add cross-platform build/test scripts and update local CI script`
- `chore(ci): enable CTest in CMake and update CI to use ctest`
- `feat(audit): add verseguy_audit crate with append-only hash chain and tests`
- `feat(wasm): add verseguy_wasm_sandbox POC with wasmtime + tests`
- `feat(compliance): add GDPR export/delete APIs and tests`
- `test(master-server): add E2E publish flow test`
- `docs: update specs and TODO to reflect CTest, scripts, and new containers`
- `docs(changelog): add recent changes, scripts usage notes, update last-updated to 2026-01-04`

Request: Please confirm if I should push `chore/init-repo-scripts` to remote and open a PR, or if you want to review locally first.