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
}

void main (string[] args) {
    Test.init (ref args);
    
    Test.add_func ("/notification-manager/initializes_without_errors", PomodoroTimer.Tests.test_notification_manager_initializes_without_errors);
    
    Test.run ();
}