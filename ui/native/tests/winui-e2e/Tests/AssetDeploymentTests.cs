using System;
using System.IO;
using NUnit.Framework;
using VerseguY.WinUI.E2E.Tests.Helpers;

namespace VerseguY.WinUI.E2E.Tests
{
    [TestFixture]
    public class AssetDeploymentTests
    {
        [Test]
        public void CopyWebAssets_WhenDistExists_CopiesFiles()
        {
            // Best-effort source path (CI will build ui/web before running tests)
            var repoRoot = Path.GetFullPath(Path.Combine(AppContext.BaseDirectory, "..", "..", "..", "..", "..", ".."));
            var webDist = Path.Combine(repoRoot, "ui", "web", "dist");
            if (!Directory.Exists(webDist))
            {
                Assert.Ignore("ui/web dist not found; build the web assets before running this test (npm run build)");
            }

            var uiProject = Path.Combine(repoRoot, "ui", "native", "VerseguY.UI");
            if (!Directory.Exists(uiProject)) Assert.Ignore("VerseguY.UI project not found; build the native UI first");

            var target = AssetDeployer.CopyWebAssets(webDist, uiProject);
            Assert.IsTrue(Directory.Exists(target), "Target assets directory was not created");

            // A minimal sanity check: index.html should exist in dist root
            var indexInTarget = Path.Combine(target, "index.html");
            Assert.IsTrue(File.Exists(indexInTarget), "index.html missing in deployed assets");
        }
    }
}
