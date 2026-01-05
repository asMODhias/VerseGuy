using NUnit.Framework;
using System;
using FlaUI.Core;
using FlaUI.Core.AutomationElements;
using FlaUI.UIA3;
using System.Threading;

namespace VerseguY.WinUI.E2E.Tests
{
    [TestFixture]
    public class WebViewRoundTripTests
    {
        [Test]
        public void WebView2_MessageRoundTrip_ReportedByHost()
        {
            var exe = Helpers.AppLauncher.ResolveExePath();
            using var app = Helpers.AppLauncher.Launch(exe);
            using var automation = new UIA3Automation();

            var mainWindow = app.GetMainWindow(automation);
            if (mainWindow == null) Assert.Ignore("Main window not available; ensure UI builds and starts");

            var btn = mainWindow.FindFirstDescendant(cf => cf.ByAutomationId("Test:SendWebView2Message"));
            var statusElem = mainWindow.FindFirstDescendant(cf => cf.ByAutomationId("Test:WebView2Status"));
            if (btn == null || statusElem == null) Assert.Ignore("Requires native test hooks: set VERSEGUY_TEST_MODE=1 and implement WebView test controls");

            // Click the button and wait briefly for simulated round trip
            btn.AsButton().Invoke();
            System.Threading.Thread.Sleep(1000);

            var status = statusElem.Name ?? string.Empty;
            Assert.IsTrue(status.Contains("RoundTrip"), $"Unexpected status value: '{status}'");
        }
    }
}
