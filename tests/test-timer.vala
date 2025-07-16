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
    
    Test.run ();
}