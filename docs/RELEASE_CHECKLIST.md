# File: docs/RELEASE_CHECKLIST.md

# Release Checklist

## Pre-Release

- [x] All tests passing
- [ ] Code coverage > 80%
- [x] Documentation updated
- [x] CHANGELOG.md updated
- [ ] Version numbers updated
- [ ] No TODO/FIXME in code

## Build

- [ ] Clean build successful
- [ ] All binaries created
- [ ] Installer created
- [ ] Code signed

### Native UI build notes

- If `dotnet build` fails with error **NETSDK1083** (unrecognized RuntimeIdentifier like `win10-x64`), ensure you're building on Windows with the **Windows App SDK** / Visual Studio workloads installed.
- Install guidance (run as Administrator):
  - winget (recommended): `winget install --id Microsoft.WindowsAppSDK.1.4 -e`
  - Chocolatey: `choco install microsoft-windows-appsdk -y`
  - Visual Studio: install **"Desktop development with C++"** and the **Windows App SDK** components via the Visual Studio Installer.
- Local workaround: if you cannot install the Windows App SDK on this machine, the build scripts will now create small **placeholder native UI artifacts** so a release bundle can still be produced. Consider using a Windows CI runner to build the full native UI instead.

## Testing

- [ ] Manual testing completed
- [ ] Integration tests pass
- [ ] Performance tests pass
- [ ] UI tests pass

## Release

- [ ] Git tag created
- [x] Release notes written
- [ ] Binaries uploaded
- [ ] Installer uploaded
- [ ] Documentation published

## Post-Release

- [ ] Announcement sent
- [ ] Social media posted
- [ ] Monitor for issues
- [ ] Plan next version
