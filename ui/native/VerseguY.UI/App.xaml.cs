using Microsoft.UI.Xaml;

namespace VerseguY.UI
{
    public partial class App : Application
    {
        public static MainWindow? MainWindowInstance { get; private set; }

        public App()
        {
            this.InitializeComponent();
        }

        protected override void OnLaunched(LaunchActivatedEventArgs args)
        {
            base.OnLaunched(args);
            var window = new MainWindow();
            MainWindowInstance = window;
            window.Activate();
        }
    }
}