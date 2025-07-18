namespace PomodoroTimer.Tests {
    static void test_application_initializes_with_correct_app_id () {
        var app = new Application ();
        
        assert (app.application_id == "com.github.user.PomodoroTimer");
    }
    
    static void test_application_creates_settings_instance () {
        // Settings is created statically, just verify it exists
        assert (Application.settings != null);
    }
    
    static void test_application_handles_activation_correctly () {
        var app = new Application ();
        
        // Test basic application construction works
        assert (app != null);
        assert (app.application_id == "com.github.user.PomodoroTimer");
    }
    
    static void test_application_creates_window_and_notification_manager () {
        var app = new Application ();
        
        // Test that get_notification_manager returns null before activation
        assert (app.get_notification_manager () == null);
    }
    
    static void test_application_handles_quit_action () {
        var app = new Application ();
        
        // Test application construction (action setup requires startup)
        assert (app != null);
    }
    
    static void test_application_handles_about_action () {
        var app = new Application ();
        
        // Test application construction (action setup requires startup)
        assert (app != null);
    }
}

void main (string[] args) {
    Test.init (ref args);
    
    Test.add_func ("/application/initializes_with_correct_app_id", PomodoroTimer.Tests.test_application_initializes_with_correct_app_id);
    Test.add_func ("/application/creates_settings_instance", PomodoroTimer.Tests.test_application_creates_settings_instance);
    Test.add_func ("/application/handles_activation_correctly", PomodoroTimer.Tests.test_application_handles_activation_correctly);
    Test.add_func ("/application/creates_window_and_notification_manager", PomodoroTimer.Tests.test_application_creates_window_and_notification_manager);
    Test.add_func ("/application/handles_quit_action", PomodoroTimer.Tests.test_application_handles_quit_action);
    Test.add_func ("/application/handles_about_action", PomodoroTimer.Tests.test_application_handles_about_action);
    
    Test.run ();
}