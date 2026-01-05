# User Guide

This user guide describes common flows and UI interactions for Verse Guy and provides actionable instructions for end users.

## Onboarding & First Run

- Launch `VerseguY.exe`.
- On first run you will be guided through a lightweight onboarding flow (create local account or sign in with OAuth) and asked to accept the Terms of Service.
- If offline, choose Local Auth to continue without network connectivity.

## Login & Session Management

- Local Auth: username + password (stored Argon2 hashed locally).
- OAuth: Google / Discord / Twitch — permissions are requested via the provider's consent screen.
- Sessions are JWTs; tokens expire by default after 24–30 hours depending on settings. Refresh flows happen automatically when available.

## Plugin Installation & Management

- Open the Plugins tab → Available → Install.
- Installed plugins appear under Installed; enable/disable subplugins per-plugin in the Settings for the plugin.
- Native DLL plugins require a signature; unsigned native plugins will be blocked unless the user explicitly overrides (not recommended).

## Backup & Restore

- Use the UI Settings → Backup to create an encrypted backup file. Backups are AES-256 encrypted and protected by your profile password.
- Restore by selecting a backup file in Settings → Restore.

## Data Export & Deletion (GDPR / CCPA)

- Export: Settings → Privacy → Export Data to generate a JSON export (audit logs, profile, plugin data where appropriate).
- Delete: Settings → Privacy → Delete Account/Export & Delete will remove local data and mark server-side references for removal where applicable.
- For server-assisted exports/deletions, use the Master Server admin endpoints (admins only) to process requests.

## Troubleshooting

- If UI fails to start, check `core` logs (`core/build/bin` artifacts) and verify that required runtimes (WinUI / WebView2) are installed.
- For build/test issues, see `scripts/README.md` and run `./scripts/test.sh` or the local PowerShell wrapper.

## Where to get help

- For developer questions see `docs/developer/PLUGIN_DEV.md` and `docs/developer/CONTAINER_DEV.md`.
- For legal or ToS questions contact the legal team and refer to `legal/ToS.md` and `legal/PrivacyPolicy.md`.
