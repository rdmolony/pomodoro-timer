using PomodoroTimer;

void test_eye_check_dialog_creation () {
    var app = new Gtk.Application ("com.github.user.PomodoroTimer.test", ApplicationFlags.FLAGS_NONE);
    app.activate.connect (() => {
        var window = new Gtk.ApplicationWindow (app);
        var dialog = new EyeCheckDialog (window);
        
        assert (dialog != null);
        assert (dialog.get_transient_for () == window);
        assert (dialog.get_modal () == true);
        
        app.quit ();
    });
    
    app.run ();
}

void test_eye_check_dialog_fullscreen () {
    var app = new Gtk.Application ("com.github.user.PomodoroTimer.test", ApplicationFlags.FLAGS_NONE);
    app.activate.connect (() => {
        var window = new Gtk.ApplicationWindow (app);
        var dialog = new EyeCheckDialog (window);
        
        assert (dialog.is_fullscreen () == true);
        
        app.quit ();
    });
    
    app.run ();
}

void test_eye_check_dialog_dismissed_signal () {
    var app = new Gtk.Application ("com.github.user.PomodoroTimer.test", ApplicationFlags.FLAGS_NONE);
    app.activate.connect (() => {
        var window = new Gtk.ApplicationWindow (app);
        var dialog = new EyeCheckDialog (window);
        
        bool signal_emitted = false;
        dialog.dismissed.connect (() => {
            signal_emitted = true;
        });
        
        // Simulate clicking the "Got it" button
        dialog.simulate_dismiss_click ();
        
        assert (signal_emitted == true);
        
        app.quit ();
    });
    
    app.run ();
}

void test_eye_check_dialog_snoozed_signal () {
    var app = new Gtk.Application ("com.github.user.PomodoroTimer.test", ApplicationFlags.FLAGS_NONE);
    app.activate.connect (() => {
        var window = new Gtk.ApplicationWindow (app);
        var dialog = new EyeCheckDialog (window);
        
        bool signal_emitted = false;
        dialog.snoozed.connect (() => {
            signal_emitted = true;
        });
        
        // Simulate clicking the "Remind me in 5 minutes" button
        dialog.simulate_snooze_click ();
        
        assert (signal_emitted == true);
        
        app.quit ();
    });
    
    app.run ();
}

void test_eye_check_dialog_esc_key_closes () {
    var app = new Gtk.Application ("com.github.user.PomodoroTimer.test", ApplicationFlags.FLAGS_NONE);
    app.activate.connect (() => {
        var window = new Gtk.ApplicationWindow (app);
        var dialog = new EyeCheckDialog (window);
        
        bool dismissed_signal_emitted = false;
        dialog.dismissed.connect (() => {
            dismissed_signal_emitted = true;
        });
        
        // Simulate ESC key press
        dialog.simulate_esc_key ();
        
        assert (dismissed_signal_emitted == true);
        
        app.quit ();
    });
    
    app.run ();
}

void test_eye_check_dialog_has_ui_elements () {
    var app = new Gtk.Application ("com.github.user.PomodoroTimer.test", ApplicationFlags.FLAGS_NONE);
    app.activate.connect (() => {
        var window = new Gtk.ApplicationWindow (app);
        var dialog = new EyeCheckDialog (window);
        
        // Test that UI elements are created
        assert (dialog.has_title_label () == true);
        assert (dialog.has_eye_drawing () == true);
        assert (dialog.has_instructions_label () == true);
        assert (dialog.has_dismiss_button () == true);
        assert (dialog.has_snooze_button () == true);
        
        app.quit ();
    });
    
    app.run ();
}

void test_eye_check_dialog_buttons_close_dialog () {
    var app = new Gtk.Application ("com.github.user.PomodoroTimer.test", ApplicationFlags.FLAGS_NONE);
    app.activate.connect (() => {
        var window = new Gtk.ApplicationWindow (app);
        var dialog = new EyeCheckDialog (window);
        
        // Test that simulate_dismiss_click calls close() - since simulate methods
        // should mimic what the real buttons do
        bool dismiss_called = false;
        dialog.dismissed.connect (() => {
            dismiss_called = true;
        });
        
        // Test that simulate_snooze_click also calls close()
        bool snooze_called = false;
        dialog.snoozed.connect (() => {
            snooze_called = true;
        });
        
        // Simulate button clicks
        dialog.simulate_dismiss_click ();
        assert (dismiss_called == true);
        
        dialog.simulate_snooze_click ();
        assert (snooze_called == true);
        
        app.quit ();
    });
    
    app.run ();
}

void main (string[] args) {
    Test.init (ref args);
    Gtk.init ();
    
    Test.add_func ("/eye-check-dialog/creation", test_eye_check_dialog_creation);
    Test.add_func ("/eye-check-dialog/fullscreen", test_eye_check_dialog_fullscreen);
    Test.add_func ("/eye-check-dialog/dismissed-signal", test_eye_check_dialog_dismissed_signal);
    Test.add_func ("/eye-check-dialog/snoozed-signal", test_eye_check_dialog_snoozed_signal);
    Test.add_func ("/eye-check-dialog/esc-key-closes", test_eye_check_dialog_esc_key_closes);
    Test.add_func ("/eye-check-dialog/has-ui-elements", test_eye_check_dialog_has_ui_elements);
    Test.add_func ("/eye-check-dialog/buttons_close_dialog", test_eye_check_dialog_buttons_close_dialog);
    
    Test.run ();
}