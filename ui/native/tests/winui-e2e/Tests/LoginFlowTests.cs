using System;
using System.IO;
using System.Threading;
using FlaUI.Core.AutomationElements;
using NUnit.Framework;
using VerseguY.WinUI.E2E.Tests.Helpers;

namespace VerseguY.WinUI.E2E.Tests
{
    [TestFixture]
    public class LoginFlowTests
    {
        [Test]
        public void LocalLogin_AllowsAccessToMain()
        {
            var exe = Helpers.AppLauncher.ResolveExePath();
            using var app = Helpers.AppLauncher.Launch(exe);
            using var automation = new FlaUI.UIA3.UIA3Automation();

            var main = app.GetMainWindow(automation);
            Assert.IsNotNull(main, "Main window did not appear");

            // Navigate to Login screen if onboarding present
            var login = main.FindFirstDescendant(cf => cf.ByAutomationId("Login:Username"))?.AsTextBox();
            if (login == null)
            {
                // Maybe app navigates to onboarding first or uses other ids; try to click AuthSelection
                var authSelection = main.FindFirstDescendant(cf => cf.ByAutomationId("Nav:Auth"))?.AsButton();
                if (authSelection != null) authSelection.Invoke();
                Thread.Sleep(500);
                login = main.FindFirstDescendant(cf => cf.ByAutomationId("Login:Username"))?.AsTextBox();
            }

            if (login == null)
            {
                Assert.Ignore("Login UI not found (AutomationId 'Login:Username') — skip local login test");
            }

            var password = main.FindFirstDescendant(cf => cf.ByAutomationId("Login:Password"))?.AsTextBox();
            var submit = main.FindFirstDescendant(cf => cf.ByAutomationId("Login:Submit"))?.AsButton();

            if (password == null || submit == null)
            {
                Assert.Ignore("Login controls incomplete — ensure 'Login:Password' and 'Login:Submit' are implemented for E2E tests");
            }

            // Use a deterministic test account flow: the application MUST accept test credentials when VERSEGUY_TEST_MODE=1 is set.
            login.Text = "e2e_user";
            password.Text = "e2e_pass";
            submit.Invoke();

            // Wait for main dashboard or license section to appear
            var timeout = TimeSpan.FromSeconds(10);
            var sw = System.Diagnostics.Stopwatch.StartNew();
            AutomationElement postLogin = null;
            while (sw.Elapsed < timeout)
            {
                postLogin = main.FindFirstDescendant(cf => cf.ByAutomationId("Dashboard:Root") ) ?? main.FindFirstDescendant(cf => cf.ByAutomationId("Tab:Organization"));
                if (postLogin != null) break;
                Thread.Sleep(300);
            }

            if (postLogin == null)
            {
                // Dump diagnostics for debugging
                var dir = Path.Combine(AppContext.BaseDirectory, "artifacts");
                Directory.CreateDirectory(dir);
                var dump = Path.Combine(dir, $"login_hierarchy_{DateTime.UtcNow:yyyyMMdd_HHmmss}.txt");
                Diagnostics.DumpWindowTree(main, dump);
                TestContext.AddTestAttachment(dump);
                Assert.Fail("Login did not navigate to dashboard within timeout");
            }

            Assert.Pass("Login flow completed and dashboard visible");
        }
    }
}
