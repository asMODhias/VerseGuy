VerseguY WinUI 3 Shell

This is a minimal WinUI 3 skeleton used as the native application shell.

Files:
- `App.xaml`, `App.xaml.cs` — Application entry
- `MainWindow.xaml`, `MainWindow.xaml.cs` — Shell window and main navigation frame
- `Auth/*` — Login screen and OAuth buttons
- `Onboarding/*` — Onboarding flow placeholder

Next steps:
- Wire startup flow to `is_first_run` detection (Rust container or native call)
- Implement actual OAuth flows via adapters
- Add WebView2 integration for dashboards
