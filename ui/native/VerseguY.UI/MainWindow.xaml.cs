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
        }
    }
}