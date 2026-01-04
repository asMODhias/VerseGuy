# P2P Container

This container is a small P2P runtime used for local development and testing.

Features implemented:
- TCP transport (Tokio) using libp2p
- Ping behaviour and connection establishment
- Local echo server for quick local tests (`local::start_echo_server`)

How to run tests:
- Unit tests: `cargo test -p verseguy_p2p`
- Verbose libp2p debug test (ignored by default):

  RUST_LOG=debug RUST_BACKTRACE=1 cargo test -p verseguy_p2p -- --nocapture ping_between_two_peers_libp2p_debug_mpsc --ignored

- mDNS discovery test (ignored by default):

  RUST_LOG=debug RUST_BACKTRACE=1 cargo test -p verseguy_p2p -- --nocapture ping_between_two_peers_mdns_discovery --ignored

Notes:
- The libp2p integration is actively being stabilized; transient dial errors (AddrInUse) are retried automatically in tests.
- The mDNS test includes bounded timeouts and a deterministic one-way fallback dial to make it stable in CI; keep it ignored by default on CI unless you explicitly opt-in to run it on VM (it may rely on UDP/mDNS availability).
