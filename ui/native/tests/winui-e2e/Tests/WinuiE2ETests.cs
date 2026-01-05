using System;
using System.Diagnostics;
using System.IO;
using System.Threading;
using FlaUI.Core;
using FlaUI.Core.AutomationElements;
using FlaUI.UIA3;
using NUnit.Framework;

namespace VerseguY.WinUI.E2E.Tests
{
    [TestFixture]
    public class WinuiE2ETests
    {
        private Application app;
        private UIA3Automation automation;

        [SetUp]
        public void SetUp()
        {
            var exe = Helpers.AppLauncher.ResolveExePath();
            app = Helpers.AppLauncher.Launch(exe);
            automation = new UIA3Automation();
        }

        [TearDown]
        public void TearDown()
        {
            try
            {
                var outcome = TestContext.CurrentContext.Result.Outcome.Status;
                if (outcome == NUnit.Framework.Interfaces.TestStatus.Failed)
                {
                    try
                    {
                        var wnd = app.GetMainWindow(automation);
                        var dir = Path.Combine(AppContext.BaseDirectory, "artifacts");
                        Directory.CreateDirectory(dir);
                        var file = Path.Combine(dir, $"failure_{DateTime.UtcNow:yyyyMMdd_HHmmss}.png");
                        using (var bmp = wnd.Capture())
                        {
#if NET7_0_OR_GREATER
                            bmp.Save(file, System.Drawing.Imaging.ImageFormat.Png);
#else
                            bmp.Save(file, System.Drawing.Imaging.ImageFormat.Png);
#endif
                        }

                        TestContext.AddTestAttachment(file);
                    }
                    catch { /* best-effort */ }
                }
            }
            finally
            {
                try { app?.Close(); } catch { }
                try { automation?.Dispose(); } catch { }
            }
        }

        [Test]
        public void Start_ShowsMainWindow()
        {
            var timeout = TimeSpan.FromSeconds(20);
            var sw = Stopwatch.StartNew();
            Window mainWindow = null;
            while (sw.Elapsed < timeout)
            {
                try
                {
                    mainWindow = app.GetMainWindow(automation);
                    if (mainWindow != null && !string.IsNullOrEmpty(mainWindow.Title)) break;
                }
                catch { }
                Thread.Sleep(500);
            }

            Assert.IsNotNull(mainWindow, "Main window not found within timeout");
            AssertContainsAny(new[] { "Verse", "Versegu" }, mainWindow.Title, "Unexpected window title");
        }

        private static void AssertContainsAny(string[] parts, string actual, string message)
        {
            foreach (var p in parts)
            {
                if (actual.IndexOf(p, StringComparison.OrdinalIgnoreCase) >= 0) return;
            }
            Assert.Fail(message + $" (was: '{actual}')");
        }
    }
}
