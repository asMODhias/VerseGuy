// Compatibility shim: re-export the newer `lru` API under the crate name `lru`.
// This allows crates that depend on `lru = "0.12"` to link against the
// `0.16` implementation until upstream updates are available.

pub use lru_real::*;
