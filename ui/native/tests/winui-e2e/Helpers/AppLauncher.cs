using System;
using System.Diagnostics;
using System.IO;
using System.Reflection;
using FlaUI.Core;

namespace VerseguY.WinUI.E2E.Tests.Helpers
{
    public static class AppLauncher
    {
        public static string ResolveExePath()
        {
            var env = Environment.GetEnvironmentVariable("VERSEGUY_UI_PATH");
            if (!string.IsNullOrEmpty(env) && File.Exists(env)) return env;

            // Resolve relative to repo root (best-effort)
            var baseDir = Path.GetFullPath(Path.Combine(AppContext.BaseDirectory, "..", "..", "..", "..", "..", ".."));
            var candidates = new[] {
                Path.Combine(baseDir, "ui", "native", "VerseguY.UI", "bin", "Debug", "net7.0-windows10.0.19041.0", "VerseguY.UI.exe"),
                Path.Combine(baseDir, "ui", "native", "VerseguY.UI", "bin", "Debug", "net7.0-windows10.0.19041.0", "VerseguY.UI.exe")
            };
            foreach (var c in candidates) if (File.Exists(c)) return c;

            throw new FileNotFoundException("VerseguY.UI.exe not found. Set VERSEGUY_UI_PATH environment variable or build the UI project first.");
        }

        public static Application Launch(string exePath)
        {
            var psi = new ProcessStartInfo(exePath)
            {
                UseShellExecute = false,
                WorkingDirectory = Path.GetDirectoryName(exePath)
            };
            // Test-mode env flag consumed by the app (if implemented)
            psi.EnvironmentVariables["VERSEGUY_TEST_MODE"] = "1";

            var proc = Process.Start(psi);
            if (proc == null) throw new InvalidOperationException("Failed to start VerseguY.UI");
            // Wait a short time for UI thread to initialize
            System.Threading.Thread.Sleep(500);
            return Application.Attach(proc.Id);
        }
    }
}