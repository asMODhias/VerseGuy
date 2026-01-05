using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;

namespace VerseguY.UI.Auth
{
    public sealed partial class LoginScreen : Page
    {
        public LoginScreen()
        {
            this.InitializeComponent();
            OAuthButtons.Content = new OAuthButtons();
        }

        private void OnLogin(object sender, RoutedEventArgs e)
        {
            // If VERSEGUY_TEST_MODE=1 accept a deterministic test account (e2e_user/e2e_pass)
            var testMode = System.Environment.GetEnvironmentVariable("VERSEGUY_TEST_MODE");
            if (testMode == "1")
            {
                var u = UsernameBox.Text ?? string.Empty;
                var p = PasswordBox.Password ?? string.Empty;
                if (u == "e2e_user" && p == "e2e_pass")
                {
                    // Navigate to test dashboard page
                    if (this.Frame != null)
                    {
                        this.Frame.Navigate(typeof(VerseguY.UI.DashboardPage));
                        return;
                    }
                }
            }

            // Placeholder: invoke auth container when available
        }
    }
}