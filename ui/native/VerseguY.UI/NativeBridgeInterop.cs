using System.Runtime.InteropServices;

namespace VerseguY.UI
{
    internal static class NativeBridgeInterop
    {
        [DllImport("VerseguY.NativeBridge.dll", CallingConvention = CallingConvention.Cdecl, EntryPoint = "SendTestMessageToWebView")]
        public static extern void SendTestMessageToWebView();
    }
}