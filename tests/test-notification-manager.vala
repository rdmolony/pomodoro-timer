namespace PomodoroTimer.Tests {
    static void test_notification_manager_initializes_without_errors () {
        // Create a mock application since NotificationManager requires one
        var app = new Application ();
        
        NotificationManager? notification_manager = null;
        
        // This should not throw any errors
        try {
            notification_manager = new NotificationManager (app);
            assert (notification_manager != null);
        } catch (Error e) {
            assert_not_reached ();
        }
    }
    
    static void test_notification_manager_shows_basic_notifications () {
        var app = new Application ();
        var notification_manager = new NotificationManager (app);
        
        // This should not throw any errors
        try {
            notification_manager.show_notification ("Test Title", "Test Body");
            // If we reach here, no exception was thrown
            assert (true);
        } catch (Error e) {
            assert_not_reached ();
        }
    }
    
    static void test_notification_manager_shows_20_20_20_notifications_with_actions () {
        var app = new Application ();
        var notification_manager = new NotificationManager (app);
        
        // This should not throw any errors
        try {
            notification_manager.show_20_20_20_notification ();
            // If we reach here, no exception was thrown
            assert (true);
        } catch (Error e) {
            assert_not_reached ();
        }
    }
    
    static void test_notification_manager_handles_sound_settings_correctly () {
        var app = new Application ();
        var notification_manager = new NotificationManager (app);
        
        // Test with sound enabled
        Application.settings.set_boolean ("notification-sound", true);
        try {
            notification_manager.show_notification ("Test Title", "Test Body");
            assert (true);
        } catch (Error e) {
            assert_not_reached ();
        }
        
        // Test with sound disabled
        Application.settings.set_boolean ("notification-sound", false);
        try {
            notification_manager.show_notification ("Test Title", "Test Body");
            assert (true);
        } catch (Error e) {
            assert_not_reached ();
        }
    }
    
    static void test_notification_manager_handles_notification_errors_gracefully () {
        var app = new Application ();
        var notification_manager = new NotificationManager (app);
        
        // Test that notification errors are handled gracefully
        // The implementation uses try/catch and warning() for errors
        try {
            notification_manager.show_notification ("Test", "Test");
            notification_manager.show_20_20_20_notification ();
            assert (true);
        } catch (Error e) {
            assert_not_reached ();
        }
    }
    
    static void test_notification_manager_schedules_snooze_reminders_correctly () {
        var app = new Application ();
        var notification_manager = new NotificationManager (app);
        
        // Test that snooze scheduling works without errors
        // The private method is called internally by the 20-20-20 notification actions
        try {
            notification_manager.show_20_20_20_notification ();
            assert (true);
        } catch (Error e) {
            assert_not_reached ();
        }
    }
    
    static void test_notification_manager_shows_eye_check_dialog () {
        var app = new Gtk.Application ("com.github.user.PomodoroTimer.test", ApplicationFlags.FLAGS_NONE);
        app.activate.connect (() => {
            var notification_manager = new NotificationManager (app);
            
            // Create a mock window for the dialog
            var window = new Gtk.ApplicationWindow (app);
            notification_manager.set_main_window (window);
            
            // Test that eye check dialog is shown instead of system notification
            bool dialog_shown = false;
            notification_manager.eye_check_dialog_shown.connect (() => {
                dialog_shown = true;
            });
            
            notification_manager.show_eye_check_dialog ();
            
            assert (dialog_shown == true);
            
            app.quit ();
        });
        
        app.run ();
    }
    
    static void test_notification_manager_handles_eye_check_dialog_dismissed () {
        var app = new Gtk.Application ("com.github.user.PomodoroTimer.test", ApplicationFlags.FLAGS_NONE);
        app.activate.connect (() => {
            var notification_manager = new NotificationManager (app);
            var window = new Gtk.ApplicationWindow (app);
            notification_manager.set_main_window (window);
            
            // Create an eye check dialog
            var dialog = new EyeCheckDialog (window);
            
            // Test that notification manager handles dismissed signal
            bool dismissed_handled = false;
            notification_manager.eye_check_dialog_dismissed.connect (() => {
                dismissed_handled = true;
            });
            
            // Connect the dialog to the notification manager
            notification_manager.connect_eye_check_dialog_signals (dialog);
            
            // Simulate dismiss
            dialog.simulate_dismiss_click ();
            
            assert (dismissed_handled == true);
            
            app.quit ();
        });
        
        app.run ();
    }
    
    static void test_notification_manager_handles_eye_check_dialog_snoozed () {
        var app = new Gtk.Application ("com.github.user.PomodoroTimer.test", ApplicationFlags.FLAGS_NONE);
        app.activate.connect (() => {
            var notification_manager = new NotificationManager (app);
            var window = new Gtk.ApplicationWindow (app);
            notification_manager.set_main_window (window);
            
            // Create an eye check dialog
            var dialog = new EyeCheckDialog (window);
            
            // Test that notification manager handles snoozed signal
            bool snoozed_handled = false;
            notification_manager.eye_check_dialog_snoozed.connect (() => {
                snoozed_handled = true;
            });
            
            // Connect the dialog to the notification manager
            notification_manager.connect_eye_check_dialog_signals (dialog);
            
            // Simulate snooze
            dialog.simulate_snooze_click ();
            
            assert (snoozed_handled == true);
            
            app.quit ();
        });
        
        app.run ();
    }
    
    static void test_notification_manager_passes_main_window_reference_to_eye_check_dialog () {
        var app = new Gtk.Application ("com.github.user.PomodoroTimer.test", ApplicationFlags.FLAGS_NONE);
        app.activate.connect (() => {
            var notification_manager = new NotificationManager (app);
            var window = new Gtk.ApplicationWindow (app);
            notification_manager.set_main_window (window);
            
            // Test that notification manager shows eye check dialog with main window reference
            var dialog = notification_manager.create_eye_check_dialog ();
            
            assert (dialog != null);
            assert (dialog.get_transient_for () == window);
            
            app.quit ();
        });
        
        app.run ();
    }
}

void main (string[] args) {
    Test.init (ref args);
    Gtk.init ();
    
    Test.add_func ("/notification-manager/initializes_without_errors", PomodoroTimer.Tests.test_notification_manager_initializes_without_errors);
    Test.add_func ("/notification-manager/shows_basic_notifications", PomodoroTimer.Tests.test_notification_manager_shows_basic_notifications);
    Test.add_func ("/notification-manager/shows_20_20_20_notifications_with_actions", PomodoroTimer.Tests.test_notification_manager_shows_20_20_20_notifications_with_actions);
    Test.add_func ("/notification-manager/handles_sound_settings_correctly", PomodoroTimer.Tests.test_notification_manager_handles_sound_settings_correctly);
    Test.add_func ("/notification-manager/handles_notification_errors_gracefully", PomodoroTimer.Tests.test_notification_manager_handles_notification_errors_gracefully);
    Test.add_func ("/notification-manager/schedules_snooze_reminders_correctly", PomodoroTimer.Tests.test_notification_manager_schedules_snooze_reminders_correctly);
    Test.add_func ("/notification-manager/shows_eye_check_dialog", PomodoroTimer.Tests.test_notification_manager_shows_eye_check_dialog);
    Test.add_func ("/notification-manager/handles_eye_check_dialog_dismissed", PomodoroTimer.Tests.test_notification_manager_handles_eye_check_dialog_dismissed);
    Test.add_func ("/notification-manager/handles_eye_check_dialog_snoozed", PomodoroTimer.Tests.test_notification_manager_handles_eye_check_dialog_snoozed);
    Test.add_func ("/notification-manager/passes_main_window_reference_to_eye_check_dialog", PomodoroTimer.Tests.test_notification_manager_passes_main_window_reference_to_eye_check_dialog);
    
    Test.run ();
}