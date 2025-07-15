namespace PomodoroTimer {
    public class Application : Adw.Application {
        public static Settings settings;
        private MainWindow window;
        private NotificationManager notification_manager;

        public Application () {
            Object (
                application_id: "com.github.user.PomodoroTimer",
                flags: ApplicationFlags.FLAGS_NONE
            );
        }

        static construct {
            settings = new Settings ("com.github.user.PomodoroTimer");
        }

        protected override void activate () {
            base.activate ();
            
            if (window == null) {
                window = new MainWindow (this);
                notification_manager = new NotificationManager (this);
            }
            
            window.present ();
        }

        protected override void startup () {
            base.startup ();
            
            var quit_action = new SimpleAction ("quit", null);
            quit_action.activate.connect (quit);
            add_action (quit_action);
            set_accels_for_action ("app.quit", {"<Control>q"});
            
            var about_action = new SimpleAction ("about", null);
            about_action.activate.connect (show_about);
            add_action (about_action);
            set_accels_for_action ("app.about", {"F1"});
        }

        private void show_about () {
            var about = new Adw.AboutWindow () {
                transient_for = window,
                application_name = "Pomodoro Timer",
                application_icon = "com.github.user.PomodoroTimer",
                developer_name = "Developer",
                version = "1.0.0",
                website = "https://github.com/user/pomodoro-timer",
                issue_url = "https://github.com/user/pomodoro-timer/issues",
                license_type = Gtk.License.GPL_3_0,
                copyright = "© 2025 Developer",
                comments = "A GNOME Pomodoro timer with 20-20-20 rule reminders"
            };
            
            about.present ();
        }

        public NotificationManager get_notification_manager () {
            return notification_manager;
        }
    }
}