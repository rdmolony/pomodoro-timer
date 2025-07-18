namespace PomodoroTimer.Tests {
    static void test_timer_initializes_with_zero_duration_and_not_running () {
        var timer = new Timer ();
        
        assert (timer.get_total_duration () == 0);
        assert (timer.get_remaining_seconds () == 0);
        assert (timer.is_running () == false);
    }
    
    static void test_timer_sets_duration_correctly () {
        var timer = new Timer ();
        
        timer.set_duration (300);
        
        assert (timer.get_total_duration () == 300);
        assert (timer.get_remaining_seconds () == 300);
    }
    
    static void test_timer_starts_and_changes_running_state () {
        var timer = new Timer ();
        timer.set_duration (300);
        
        timer.start ();
        
        assert (timer.is_running () == true);
    }
    
    static void test_timer_handles_multiple_start_calls_gracefully () {
        var timer = new Timer ();
        timer.set_duration (300);
        
        timer.start ();
        bool first_running = timer.is_running ();
        timer.start ();
        bool second_running = timer.is_running ();
        
        assert (first_running == true);
        assert (second_running == true);
    }
    
    static void test_timer_pauses_correctly () {
        var timer = new Timer ();
        timer.set_duration (300);
        timer.start ();
        
        timer.pause ();
        
        assert (timer.is_running () == false);
    }
    
    static void test_timer_handles_pause_when_not_running () {
        var timer = new Timer ();
        timer.set_duration (300);
        
        timer.pause ();
        
        assert (timer.is_running () == false);
    }
    
    static void test_timer_stops_correctly () {
        var timer = new Timer ();
        timer.set_duration (300);
        timer.start ();
        
        timer.stop ();
        
        assert (timer.is_running () == false);
    }
    
    static void test_timer_tracks_remaining_time_correctly () {
        var timer = new Timer ();
        timer.set_duration (300);
        
        // Initial remaining time should equal duration
        assert (timer.get_remaining_seconds () == 300);
        
        // After starting, remaining time should still be 300 initially
        timer.start ();
        assert (timer.get_remaining_seconds () == 300);
        
        // We can't easily test the actual countdown without waiting or mocking
        // But we can verify the getter works
        timer.stop ();
        assert (timer.get_remaining_seconds () <= 300);
    }
    
    static void test_timer_stops_when_reaching_zero_and_emits_finished_signal () {
        var timer = new Timer ();
        timer.set_duration (1); // 1 second for quick test
        
        bool finished_signal_received = false;
        timer.finished.connect (() => {
            finished_signal_received = true;
        });
        
        timer.start ();
        assert (timer.is_running () == true);
        
        // Wait for timer to finish (1 second + small buffer)
        var main_loop = new MainLoop ();
        Timeout.add (1500, () => {
            main_loop.quit ();
            return false;
        });
        main_loop.run ();
        
        // Timer should have stopped automatically
        assert (timer.is_running () == false);
        assert (timer.get_remaining_seconds () == 0);
        assert (finished_signal_received == true);
    }
}

void main (string[] args) {
    Test.init (ref args);
    
    Test.add_func ("/timer/initializes_with_zero_duration_and_not_running", PomodoroTimer.Tests.test_timer_initializes_with_zero_duration_and_not_running);
    Test.add_func ("/timer/sets_duration_correctly", PomodoroTimer.Tests.test_timer_sets_duration_correctly);
    Test.add_func ("/timer/starts_and_changes_running_state", PomodoroTimer.Tests.test_timer_starts_and_changes_running_state);
    Test.add_func ("/timer/handles_multiple_start_calls_gracefully", PomodoroTimer.Tests.test_timer_handles_multiple_start_calls_gracefully);
    Test.add_func ("/timer/pauses_correctly", PomodoroTimer.Tests.test_timer_pauses_correctly);
    Test.add_func ("/timer/handles_pause_when_not_running", PomodoroTimer.Tests.test_timer_handles_pause_when_not_running);
    Test.add_func ("/timer/stops_correctly", PomodoroTimer.Tests.test_timer_stops_correctly);
    Test.add_func ("/timer/tracks_remaining_time_correctly", PomodoroTimer.Tests.test_timer_tracks_remaining_time_correctly);
    Test.add_func ("/timer/stops_when_reaching_zero_and_emits_finished_signal", PomodoroTimer.Tests.test_timer_stops_when_reaching_zero_and_emits_finished_signal);
    
    Test.run ();
}