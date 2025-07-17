namespace PomodoroTimer {
    public class NotificationManager : Object {
        private Notify.Notification? current_notification;
        private Gtk.Application app;
        private GSound.Context? sound_context;
        private Gtk.Window? main_window;
        
        public signal void eye_check_dialog_shown ();
        public signal void eye_check_dialog_dismissed ();
        public signal void eye_check_dialog_snoozed ();
        
        public NotificationManager (Gtk.Application application) {
            app = application;
            Notify.init ("Pomodoro Timer");
            
            try {
                sound_context = new GSound.Context ();
            } catch (Error e) {
                warning ("Could not initialize sound context: %s", e.message);
            }
        }
        
        public void set_main_window (Gtk.Window window) {
            main_window = window;
        }
        
        public void show_eye_check_dialog () {
            eye_check_dialog_shown ();
        }
        
        public EyeCheckDialog create_eye_check_dialog () {
            if (main_window == null) {
                warning ("Main window not set, cannot create eye check dialog");
                return null;
            }
            
            return new EyeCheckDialog (main_window);
        }
        
        public void connect_eye_check_dialog_signals (EyeCheckDialog dialog) {
            dialog.dismissed.connect (() => {
                eye_check_dialog_dismissed ();
            });
            
            dialog.snoozed.connect (() => {
                eye_check_dialog_snoozed ();
            });
        }
        
        public void show_notification (string title, string body) {
            try {
                current_notification = new Notify.Notification (title, body, "com.github.user.PomodoroTimer");
                current_notification.set_urgency (Notify.Urgency.NORMAL);
                current_notification.set_timeout (5000);
                current_notification.show ();
                
                play_notification_sound ();
            } catch (Error e) {
                warning ("Could not show notification: %s", e.message);
            }
        }
        
        public void show_20_20_20_notification () {
            try {
                var notification = new Notify.Notification (
                    "20-20-20 Rule Reminder",
                    "Look at something 20 feet away for 20 seconds to rest your eyes.",
                    "com.github.user.PomodoroTimer"
                );
                
                notification.set_urgency (Notify.Urgency.NORMAL);
                notification.set_timeout (Notify.EXPIRES_NEVER);
                
                notification.add_action ("dismiss", "Got it!", (notification, action) => {
                    try {
                        notification.close ();
                    } catch (Error e) {
                        warning ("Could not close notification: %s", e.message);
                    }
                });
                
                notification.add_action ("snooze", "Remind me in 5 minutes", (notification, action) => {
                    try {
                        notification.close ();
                        schedule_snooze_reminder ();
                    } catch (Error e) {
                        warning ("Could not close notification: %s", e.message);
                    }
                });
                
                notification.show ();
                play_notification_sound ();
                
            } catch (Error e) {
                warning ("Could not show 20-20-20 notification: %s", e.message);
            }
        }
        
        private void schedule_snooze_reminder () {
            Timeout.add_seconds (5 * 60, () => {
                show_20_20_20_notification ();
                return false;
            });
        }
        
        private void play_notification_sound () {
            if (!Application.settings.get_boolean ("notification-sound")) {
                return;
            }
            
            if (sound_context == null) {
                return;
            }
            
            try {
                sound_context.play_simple (null, 
                    GSound.Attribute.EVENT_ID, "message-new-instant",
                    GSound.Attribute.EVENT_DESCRIPTION, "Pomodoro notification"
                );
            } catch (Error e) {
                warning ("Could not play notification sound: %s", e.message);
            }
        }
    }
}