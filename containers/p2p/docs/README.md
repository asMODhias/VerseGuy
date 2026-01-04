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

Notes:
- The libp2p integration is actively being stabilized; transient dial errors (AddrInUse) are retried automatically in tests.
- mDNS integration is next on the roadmap.
