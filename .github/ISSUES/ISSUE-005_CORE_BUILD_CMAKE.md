# ISSUE-005: Core build blocked — CMake not available on PATH

## Problem
While building TEIL 2 (Core DLL), an attempt to run CMake on the local machine failed because `cmake` is not available in PATH. The repository provides `core/CMakeLists.txt` and CTest tests; building locally requires CMake to be installed and available.

## Suggested steps
1. Install CMake (>= 3.25) and add it to PATH.
2. On Windows (PowerShell):

```powershell
cmake -S core -B core/build
cmake --build core/build --config Debug
ctest -C Debug --test-dir core/build
```

3. Confirm `VerseguY.Core.dll` is produced in `core/build` and `ctest` reports success.

## Notes
- I implemented the `PluginHost` and plugin discovery in `core/src/Core.cpp` and committed it to `feat/setup-part1`.
- If you want, I can install CMake locally (requires admin / package manager access) or continue with TEIL 3 while leaving the Core build for you to run.
