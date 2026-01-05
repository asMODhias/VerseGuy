# Installer Usage & Signing

## Übersicht
Dieses Dokument beschreibt, wie Installationsartefakte (MSI, DMG, DEB/RPM) lokal gebaut, signiert und in der CI signiert/verifiziert werden.

> Hinweis: Artefakt‑Signierung MUSS über CI mit sicheren Secrets erfolgen. Niemals Zertifikate oder Passwörter in das Repo einchecken.

---

## Windows (MSI)

### Voraussetzungen
- WiX Toolset (`candle.exe`, `light.exe`) in PATH
- `signtool.exe` (Teil des Windows SDK) für Signierung
- CI‑Secret: `CODESIGN_CERT_PFX` (base64-encodiertes PFX) und `CODESIGN_PFX_PASSWORD`

### Beispiel (lokal)
PowerShell:

```powershell
# Entpacke PFX in temporäre Datei
[System.IO.File]::WriteAllBytes("C:\\tmp\\codesign.pfx", [Convert]::FromBase64String($env:CODESIGN_CERT_PFX))
& "C:\\Program Files (x86)\\Windows Kits\\10\\bin\\x64\\signtool.exe" sign /f C:\\tmp\\codesign.pfx /p $env:CODESIGN_PFX_PASSWORD /fd SHA256 /tr http://timestamp.digicert.com /td SHA256 "path\\to\\VerseguY-1.0.0.msi"

# Verifizieren
& "C:\\Program Files (x86)\\Windows Kits\\10\\bin\\x64\\signtool.exe" verify /pa "path\\to\\VerseguY-1.0.0.msi"
```

### CI (GitHub Actions) — Beispiel

```yaml
jobs:
  sign-windows-artifact:
    runs-on: windows-latest
    if: contains(github.event.head_commit.message, '[release]')
    steps:
      - uses: actions/checkout@v4
      - name: Download artifact
        uses: actions/download-artifact@v4
        with:
          name: verseguy-msi
          path: ./artifacts
      - name: Decode PFX
        run: |
          echo "${{ secrets.CODESIGN_CERT_PFX }}" | Out-File -FilePath cert.b64 -Encoding ascii
          $bytes = [System.Convert]::FromBase64String((Get-Content cert.b64 -Raw))
          [System.IO.File]::WriteAllBytes('codesign.pfx', $bytes)
        shell: powershell
      - name: Sign MSI
        run: |
          & "C:\\Program Files (x86)\\Windows Kits\\10\\bin\\x64\\signtool.exe" sign /f codesign.pfx /p ${{ secrets.CODESIGN_PFX_PASSWORD }} /fd SHA256 /tr http://timestamp.digicert.com /td SHA256 "artifacts\\VerseguY-*.msi"
        shell: powershell
      - name: Upload signed artifact
        uses: actions/upload-artifact@v4
        with:
          name: verseguy-msi-signed
          path: artifacts
```

---

## macOS (DMG)

### Voraussetzungen
- `codesign` (macOS), Xcode command line tools
- Apple notarization (requires Apple ID & App Specific Password in CI)
- CI‑Secrets: `APPLE_ID`, `APPLE_APP_PASSWORD`, `CODESIGN_CERT_P12` (base64)

### Beispiel (lokal)

```bash
# Entpacke cert
echo "$CODESIGN_CERT_P12" | base64 --decode > cert.p12
security import cert.p12 -P "$CERT_P12_PASSWORD" -k ~/Library/Keychains/login.keychain
codesign --sign "Developer ID Application: Your Org (TEAMID)" --timestamp --options runtime "path/to/YourApp.app"
# Create DMG (hdiutil) and notarize (xcrun altool / notarytool)
```

---

## Linux (DEB / RPM)

### Voraussetzungen
- `fpm` or `cargo-deb` for creating packages
- Optional: GPG for package signing (DEB/RPM) — CI secret `GPG_PRIVATE_KEY` (base64) and `GPG_PASSPHRASE`

### Beispiel (CI)
- Import GPG key in CI runner
- Sign packages with `gpg --batch --yes --import` and `gpg --detach-sign --armor --output package.deb.asc package.deb`

---

## Verifikation & Notarization
- Windows: `signtool verify /pa <file>`
- macOS notarization: `xcrun notarytool store-credential`, `notarytool submit --wait` and `notarytool staple`
- Linux: Verify GPG detached signatures with `gpg --verify package.deb.asc package.deb`

---

## CI Best Practices
- Keep signing keys in secure secret stores (GitHub Secrets, Azure KeyVault, HashiCorp Vault)
- Gate signing jobs: only run on `tags` or protected branches and **require manual approval** if necessary
- Fail fast if secrets are missing; provide clear logs saying which secret is absent
- Use ephemeral files for keys and securely delete them after signing
- Provide a verification step in CI that re-verifies the signature and uploads only verified artifacts

---

## Checkliste vor Release
- [ ] Legal documents present (`legal/ToS.md`, `legal/PrivacyPolicy.md`)
- [ ] Release tag created and signed commits verified
- [ ] Packaging pipeline executed and artifacts verified
- [ ] Signed artifacts uploaded to release
- [ ] Optional: Code signing certificates rotated and documented

---

## Kontakt
Für Hilfe bei der CI‑Signierung oder beim Einrichten von Secrets schreib mir kurz — ich helfe beim Erstellen des GitHub Actions Jobs oder Azure DevOps Task Beispiele.
