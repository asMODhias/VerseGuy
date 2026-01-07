# Auth Container — OAuth Example & Tests

This folder implements OAuth support per TEIL 13 of the implementation guide.

Environment variables (example):
- `GOOGLE_CLIENT_ID`
- `GOOGLE_CLIENT_SECRET`

Run example:

- Set environment variables:
  - Windows (PowerShell):
    ```powershell
    $env:GOOGLE_CLIENT_ID = "your-client-id"
    $env:GOOGLE_CLIENT_SECRET = "your-secret"
    ```
  - Linux/macOS:
    ```bash
    export GOOGLE_CLIENT_ID="your-client-id"
    export GOOGLE_CLIENT_SECRET="your-secret"
    ```

- Run example:
  ```bash
  cargo run --example oauth_example
  ```

Integration tests with mocked provider (mockito):

- To run the mocked OAuth integration test, enable the feature:
  ```bash
  cargo test --features oauth_integration
  ```

Notes:
- Real-provider E2E tests should be run manually and require valid OAuth client credentials and a reachable callback endpoint per the guide.
- The implementation follows TEIL 13 strictly (state handling, token exchange, refresh, user creation/mapping).
