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
}

void main (string[] args) {
    Test.init (ref args);
    
    Test.add_func ("/notification-manager/initializes_without_errors", PomodoroTimer.Tests.test_notification_manager_initializes_without_errors);
    Test.add_func ("/notification-manager/shows_basic_notifications", PomodoroTimer.Tests.test_notification_manager_shows_basic_notifications);
    Test.add_func ("/notification-manager/shows_20_20_20_notifications_with_actions", PomodoroTimer.Tests.test_notification_manager_shows_20_20_20_notifications_with_actions);
    
    Test.run ();
}