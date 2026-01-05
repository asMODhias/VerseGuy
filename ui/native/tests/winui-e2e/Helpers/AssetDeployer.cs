using System;
using System.IO;

namespace VerseguY.WinUI.E2E.Tests.Helpers
{
    public static class AssetDeployer
    {
        /// <summary>
        /// Copies the Web build assets from a source (e.g., ui/web/dist) into the VerseguY.UI assets folder.
        /// Returns the target path on success.
        /// </summary>
        public static string CopyWebAssets(string webDistPath, string uiProjectPath)
        {
            if (string.IsNullOrEmpty(webDistPath)) throw new ArgumentNullException(nameof(webDistPath));
            if (string.IsNullOrEmpty(uiProjectPath)) throw new ArgumentNullException(nameof(uiProjectPath));

            if (!Directory.Exists(webDistPath)) throw new DirectoryNotFoundException($"Web dist not found: {webDistPath}");
            if (!Directory.Exists(uiProjectPath)) throw new DirectoryNotFoundException($"UI project path not found: {uiProjectPath}");

            var target = Path.Combine(uiProjectPath, "www");
            if (Directory.Exists(target)) Directory.Delete(target, true);
            Directory.CreateDirectory(target);

            foreach (var dirPath in Directory.GetDirectories(webDistPath, "*", SearchOption.AllDirectories))
            {
                Directory.CreateDirectory(dirPath.Replace(webDistPath, target));
            }

            // Copy files
            foreach (var newPath in Directory.GetFiles(webDistPath, "*.*", SearchOption.AllDirectories))
            {
                File.Copy(newPath, newPath.Replace(webDistPath, target), true);
            }

            return target;
        }
    }
}