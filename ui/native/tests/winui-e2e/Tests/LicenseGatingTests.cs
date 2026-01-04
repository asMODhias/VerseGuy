using System;
using System.IO;
using System.Threading;
using FlaUI.Core.AutomationElements;
using NUnit.Framework;
using VerseguY.WinUI.E2E.Tests.Helpers;

namespace VerseguY.WinUI.E2E.Tests
{
    [TestFixture]
    public class LicenseGatingTests
    {
        [Test]
        public void Organization_ProSections_LockedForFreeLicense()
        {
            var exe = Helpers.AppLauncher.ResolveExePath();
            using var app = Helpers.AppLauncher.Launch(exe);
            using var automation = new FlaUI.UIA3.UIA3Automation();

            var main = app.GetMainWindow(automation);
            Assert.IsNotNull(main, "Main window not found");

            // Navigate to Organization tab
            var orgTab = main.FindFirstDescendant(cf => cf.ByAutomationId("Tab:Organization"))?.AsButton();
            if (orgTab == null)
            {
                // fallback: find by name
                orgTab = main.FindFirstDescendant(cf => cf.ByName("Organization"))?.AsButton();
            }

            if (orgTab == null)
            {
                Assert.Ignore("Organization tab not found; skip license gating test");
            }

            orgTab.Invoke();
            Thread.Sleep(400);

            // Check for Recruitment section or upgrade prompt
            var recruitSection = main.FindFirstDescendant(cf => cf.ByAutomationId("Section:Recruitment"));
            var upgradeButton = main.FindFirstDescendant(cf => cf.ByAutomationId("Upgrade:Recruitment"));

            if (recruitSection == null && upgradeButton == null)
            {
                Assert.Ignore("Recruitment UI not present — ensure Organization plugin sections have automation ids for E2E tests");
            }

            if (upgradeButton != null)
            {
                Assert.IsTrue(upgradeButton.Properties.IsEnabled.Value, "Upgrade button should be enabled for Free license users");
                Assert.Pass("Recruitment is gated and shows upgrade button for free license");
            }

            // If Recruitment section exists, it's either Pro or test scenario—fail to indicate unexpected access
            Assert.Fail("Recruitment section visible without upgrade prompt — license gating might be misconfigured or test preconditions not met");
        }
    }
}
