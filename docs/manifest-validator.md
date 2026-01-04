# Manifest Validator

This tool validates `manifest.toml` files in `plugins/*` against a small set of required fields.

Usage:

- Basic: `cargo run -p manifest-validator --`
  - by default scans `plugins/**/manifest.toml`
- Specific files or globs: `cargo run -p manifest-validator -- "plugins/base/*/manifest.toml"`
- Shell wrapper: `scripts/validate-manifests.sh [<pattern> ...]`

Checks performed:

- `[plugin]` section present
  - `id`, `name`, `version` present and non-empty
  - `core_version_min`, `sdk_version` present
- `[capabilities]` section present and `required` not empty

If any manifest fails validation the binary exits with a non-zero error code and prints details.
