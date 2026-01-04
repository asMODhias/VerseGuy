# WinUI E2E Test Runbook

This document describes how to run WinUI end-to-end tests locally and in CI.

Scope
- Startup and splash screen
- Onboarding/login flows (local auth)
- License gating visibility
- WebView2 bridge round-trip tests (secure storage message)

Requirements
- Windows runner (windows-latest)
- WebView2 runtime installed
- Visual Studio / dotnet SDK 7.0

How to run locally
1. Build `ui/native/VerseguY.UI` in Debug configuration.
2. Build `ui/web` (optional, to embed latest web assets).
3. From `ui/native/tests/winui-e2e`: `dotnet test`.

CI notes
- CI workflow should: build native UI, build web, copy web assets into `VerseguY.UI` assets folder, run `dotnet test`, upload artifacts on failure.

Diagnostics
- Tests capture screenshots on failure and attach them to the test report.
- Capture WebView2 DevTools logs when running the WebView2 round-trip tests (future step).

Test Hooks
- The WebView2 round-trip E2E tests require a small test hook in the native app that is only enabled when `VERSEGUY_TEST_MODE=1` is set.
- The native hook should expose an automation-visible status element (AutomationId `Test:WebView2Status`) and a debug button (AutomationId `Test:SendWebView2Message`) used by tests. Implementations must keep these hooks disabled in production builds.

Automation IDs expected by E2E tests (implement in WinUI views where applicable):
- `Login:Username`, `Login:Password`, `Login:Submit` — Local login flow
- `Nav:Auth` or `Tab:Organization`, `Dashboard:Root` — navigation points
- `Section:Recruitment`, `Upgrade:Recruitment` — license gating checks

Tests are defensive: they will `Ignore` (not fail) if required automation ids are not present, to avoid false failures on incomplete UIs.