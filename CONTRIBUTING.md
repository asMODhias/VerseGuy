# Contributing to VerseGuy

Thanks for contributing! A few quick guidelines to keep development smooth:

- Run `cargo fmt --all` and `cargo clippy --all-targets --all-features` before opening PRs.
- **Do not use `unwrap` or `expect` in production/library code.** Tests/examples may use them for brevity, but add a comment when intentional.
- Follow the code style in `.editorconfig`.
- Add tests for new features and run `cargo test --all --workspace` locally.
- Write small, focused commits and open a PR for review.

## Quickstart

See `README.md` and `docs/DEV_SETUP.md` for quickstart and development commands (format, clippy, tests, CI badge notes).
