using System;
using System.IO;
using FlaUI.Core;
using FlaUI.Core.AutomationElements;
using FlaUI.Core.Conditions;
using FlaUI.UIA3;

namespace VerseguY.WinUI.E2E.Tests.Helpers
{
    public static class Diagnostics
    {
        public static void DumpWindowTree(Window window, string outPath)
        {
            try
            {
                using (var writer = new StreamWriter(outPath, false))
                {
                    DumpElement(window, writer, 0);
                }
            }
            catch { /* best-effort */ }
        }

        private static void DumpElement(AutomationElement element, StreamWriter writer, int depth)
        {
            if (element == null) return;
            var indent = new string(' ', depth * 2);
            string ctl = "?";
            try { ctl = element.ControlType.ToString(); } catch { }
            writer.WriteLine($"{indent}{ctl} - Name:'{element.Name}' - AutomationId:'{element.AutomationId}'");
            try
            {
                foreach (var child in element.FindAllChildren())
                {
                    DumpElement(child, writer, depth + 1);
                }
            }
            catch { }
        }
    }
}
