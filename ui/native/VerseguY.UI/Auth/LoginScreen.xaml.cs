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
            // Placeholder: invoke auth container when available
        }
    }
}