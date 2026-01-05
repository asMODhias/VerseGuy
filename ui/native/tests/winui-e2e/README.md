WinUI E2E Tests

How to run locally:

1. Build the app: open `ui/native/VerseguY.UI` and build (Debug).
2. Option A: Set env var `VERSEGUY_UI_PATH` to the built `VerseguY.UI.exe` path.
3. Option B: Ensure the default path `ui/native/VerseguY.UI/bin/Debug/net7.0-windows10.0.19041.0/VerseguY.UI.exe` exists.
4. Run tests: `dotnet test` in this folder.

Notes:
- Tests start the app with `VERSEGUY_TEST_MODE=1` environment variable (app should honor it to enable test hooks).
- Artifacts (screenshots) are saved to `bin/Debug/net7.0-windows10.0.19041.0/artifacts` on failures.
- CI will build native + web assets then run tests; a workflow will be added next.