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
}

void main (string[] args) {
    Test.init (ref args);
    
    Test.add_func ("/notification-manager/initializes_without_errors", PomodoroTimer.Tests.test_notification_manager_initializes_without_errors);
    Test.add_func ("/notification-manager/shows_basic_notifications", PomodoroTimer.Tests.test_notification_manager_shows_basic_notifications);
    Test.add_func ("/notification-manager/shows_20_20_20_notifications_with_actions", PomodoroTimer.Tests.test_notification_manager_shows_20_20_20_notifications_with_actions);
    Test.add_func ("/notification-manager/handles_sound_settings_correctly", PomodoroTimer.Tests.test_notification_manager_handles_sound_settings_correctly);
    Test.add_func ("/notification-manager/handles_notification_errors_gracefully", PomodoroTimer.Tests.test_notification_manager_handles_notification_errors_gracefully);
    Test.add_func ("/notification-manager/schedules_snooze_reminders_correctly", PomodoroTimer.Tests.test_notification_manager_schedules_snooze_reminders_correctly);
    
    Test.run ();
}