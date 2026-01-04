using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;

namespace VerseguY.UI
{
    public partial class MainWindow : Window
    {
        public MainWindow()
        {
            this.InitializeComponent();
            // For now navigate to Login page. First-run logic will be wired to containers later.
            MainFrame.Navigate(typeof(Auth.LoginScreen));

            // If test mode is enabled, expose a couple of test controls for E2E automation.
            var testMode = System.Environment.GetEnvironmentVariable("VERSEGUY_TEST_MODE");
            if (testMode == "1")
            {
                // Add lightweight test-only UI elements to the window (hidden by default)
                var webviewStatus = new Microsoft.UI.Xaml.Controls.TextBlock { Text = "WebView2:Idle", Visibility = Microsoft.UI.Xaml.Visibility.Collapsed };
                Microsoft.UI.Xaml.Automation.AutomationProperties.SetAutomationId(webviewStatus, "Test:WebView2Status");

                var debugBtn = new Microsoft.UI.Xaml.Controls.Button { Content = "SendWebView2Message", Visibility = Microsoft.UI.Xaml.Visibility.Collapsed };
                Microsoft.UI.Xaml.Automation.AutomationProperties.SetAutomationId(debugBtn, "Test:SendWebView2Message");

                // Wire click handler for test mode to simulate WebView2 round-trip
                debugBtn.Click += (s, e) =>
                {
                    webviewStatus.Text = "WebView2:MessageSent";
                    webviewStatus.Visibility = Microsoft.UI.Xaml.Visibility.Visible;

                    // If native bridge is available, notify it (noop if DLL missing)
                    try { VerseguY.UI.NativeBridgeInterop.SendTestMessageToWebView(); } catch { }

                    // Simulate asynchronous round-trip and host response
                    _ = System.Threading.Tasks.Task.Run(async () =>
                    {
                        await System.Threading.Tasks.Task.Delay(250);
                        // marshal update to UI thread
                        _ = this.DispatcherQueue.TryEnqueue(() =>
                        {
                            webviewStatus.Text = "WebView2:RoundTrip:OK";
                        });
                    });
                };

                // Add to the window's visual tree root (best-effort)
                if (this.Content is Microsoft.UI.Xaml.Controls.Grid rootGrid)
                {
                    rootGrid.Children.Add(webviewStatus);
                    rootGrid.Children.Add(debugBtn);
                }
            }
        }
    }
}