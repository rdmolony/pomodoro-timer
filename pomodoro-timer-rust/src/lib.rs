pub mod timer;
mod timer_model;

pub use timer::Timer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timer_should_initialize_with_zero_duration() {
        let timer = Timer::new();
        assert_eq!(timer.get_duration(), 0);
    }

    #[test]
    fn timer_should_set_duration_correctly() {
        let mut timer = Timer::new();
        timer.set_duration(1500);
        assert_eq!(timer.get_duration(), 1500);
    }

    #[test]
    fn timer_should_report_remaining_time() {
        let mut timer = Timer::new();
        timer.set_duration(1500);
        assert_eq!(timer.get_remaining(), 1500);
    }

    #[test]
    fn timer_should_report_total_duration() {
        let mut timer = Timer::new();
        timer.set_duration(1500);
        assert_eq!(timer.get_total_duration(), 1500);
    }

    #[test]
    fn timer_should_start_countdown() {
        let mut timer = Timer::new();
        timer.set_duration(1500);
        timer.start();
        assert!(timer.is_running());
    }

    #[test]
    fn timer_should_pause_countdown() {
        let mut timer = Timer::new();
        timer.set_duration(1500);
        timer.start();
        timer.pause();
        assert!(!timer.is_running());
    }

    #[test]
    fn timer_should_reset_to_original_duration() {
        let mut timer = Timer::new();
        timer.set_duration(1500);
        timer.start();
        timer.reset();
        assert_eq!(timer.get_remaining(), 1500);
        assert!(!timer.is_running());
    }

    #[test]
    fn timer_should_emit_tick_events() {
        use std::rc::Rc;
        use std::cell::RefCell;
        
        let mut timer = Timer::new();
        timer.set_duration(10);
        
        let tick_count = Rc::new(RefCell::new(0));
        let tick_count_clone = tick_count.clone();
        
        timer.on_tick(move |remaining| {
            *tick_count_clone.borrow_mut() += 1;
            assert!(remaining < 10); // Should be less than original duration
        });
        
        timer.start();
        timer.tick(); // Simulate one tick
        assert_eq!(*tick_count.borrow(), 1);
    }

    #[test]
    fn timer_should_emit_finished_event_when_complete() {
        use std::rc::Rc;
        use std::cell::RefCell;
        
        let mut timer = Timer::new();
        timer.set_duration(1);
        
        let finished_called = Rc::new(RefCell::new(false));
        let finished_called_clone = finished_called.clone();
        
        timer.on_finished(move || {
            *finished_called_clone.borrow_mut() = true;
        });
        
        timer.start();
        timer.tick(); // This should finish the timer (1 -> 0)
        assert!(*finished_called.borrow());
    }

    #[test]
    fn timer_should_stop_running_when_finished() {
        let mut timer = Timer::new();
        timer.set_duration(1);
        timer.start();
        
        assert!(timer.is_running());
        timer.tick(); // This should finish the timer (1 -> 0)
        assert!(!timer.is_running());
    }

    #[test]
    fn timer_model_should_initialize_with_default_state() {
        use crate::timer_model::TimerModel;
        
        let model = TimerModel::init();
        
        // Should initialize with default pomodoro duration (25 minutes = 1500 seconds)
        assert_eq!(model.get_duration(), 1500);
        assert_eq!(model.get_remaining(), 1500);
        assert!(!model.is_running());
    }

    #[test]
    fn timer_model_should_handle_start_message() {
        use crate::timer_model::{TimerModel, TimerMsg};
        
        let mut model = TimerModel::init();
        
        // Process Start message
        let result = model.update(TimerMsg::Start);
        
        // Timer should now be running
        assert!(model.is_running());
        
        // Should not cause any side effects (no commands returned)
        assert!(result.is_none());
    }

    #[test]
    fn timer_model_should_handle_pause_message() {
        use crate::timer_model::{TimerModel, TimerMsg};
        
        let mut model = TimerModel::init();
        
        // Start the timer first
        model.update(TimerMsg::Start);
        assert!(model.is_running());
        
        // Process Pause message
        let result = model.update(TimerMsg::Pause);
        
        // Timer should now be paused
        assert!(!model.is_running());
        
        // Should not cause any side effects (no commands returned)
        assert!(result.is_none());
    }

    #[test]
    fn timer_model_should_handle_reset_message() {
        use crate::timer_model::{TimerModel, TimerMsg};
        
        let mut model = TimerModel::init();
        
        // Start the timer and simulate some time passing
        model.update(TimerMsg::Start);
        assert!(model.is_running());
        
        // Process Reset message
        let result = model.update(TimerMsg::Reset);
        
        // Timer should be reset to original duration and not running
        assert_eq!(model.get_remaining(), 1500);
        assert!(!model.is_running());
        
        // Should not cause any side effects (no commands returned)
        assert!(result.is_none());
    }
}
