pub mod timer;
mod timer_model;
mod eye_check_model;

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

    #[test]
    fn timer_model_should_handle_tick_message() {
        use crate::timer_model::{TimerModel, TimerMsg};
        
        let mut model = TimerModel::init();
        model.update(TimerMsg::Start);
        
        let initial_remaining = model.get_remaining();
        
        // Process Tick message
        let result = model.update(TimerMsg::Tick);
        
        // Timer should have decremented by 1
        assert_eq!(model.get_remaining(), initial_remaining - 1);
        assert!(model.is_running());
        
        // Should not cause any side effects (no commands returned)
        assert!(result.is_none());
    }

    #[test]
    fn timer_model_should_create_timer_display_ui() {
        use crate::timer_model::TimerModel;
        
        // Initialize GTK for testing
        if gtk::init().is_err() {
            return; // Skip test if GTK can't be initialized
        }
        
        let model = TimerModel::init();
        
        // Create the widget tree
        let widgets = model.init_widgets();
        
        // Should have a main container
        assert!(widgets.main_box.is_some());
        
        // Should have a time display label
        assert!(widgets.time_label.is_some());
        
        // Should have start, pause, and reset buttons
        assert!(widgets.start_button.is_some());
        assert!(widgets.pause_button.is_some());
        assert!(widgets.reset_button.is_some());
    }

    #[test]
    fn timer_model_should_update_ui_when_state_changes() {
        use crate::timer_model::{TimerModel, TimerMsg};
        
        // Initialize GTK for testing
        if gtk::init().is_err() {
            return; // Skip test if GTK can't be initialized
        }
        
        let mut model = TimerModel::init();
        let widgets = model.init_widgets();
        
        // Initial state should show 25:00
        model.update_ui(&widgets);
        let initial_text = widgets.time_label.as_ref().unwrap().text();
        assert!(initial_text.contains("25:00"));
        
        // Start the timer - should still show 25:00
        model.update(TimerMsg::Start);
        model.update_ui(&widgets);
        let running_text = widgets.time_label.as_ref().unwrap().text();
        assert!(running_text.contains("25:00"));
        
        // Tick once - should show 24:59
        model.update(TimerMsg::Tick);
        model.update_ui(&widgets);
        let ticked_text = widgets.time_label.as_ref().unwrap().text();
        assert!(ticked_text.contains("24:59"));
    }

    #[test]
    fn timer_model_should_handle_button_clicks() {
        use crate::timer_model::{TimerModel, TimerMsg};
        use std::rc::Rc;
        use std::cell::RefCell;
        use gtk::prelude::ButtonExt;
        
        // Initialize GTK for testing
        if gtk::init().is_err() {
            return; // Skip test if GTK can't be initialized
        }
        
        let model = TimerModel::init();
        let widgets = model.init_widgets();
        
        // Create a shared state to capture messages
        let messages = Rc::new(RefCell::new(Vec::new()));
        
        // Connect button signals
        model.connect_signals(&widgets, messages.clone());
        
        // Simulate button clicks
        if let Some(start_button) = &widgets.start_button {
            start_button.emit_clicked();
        }
        
        // Check that Start message was captured
        assert_eq!(messages.borrow().len(), 1);
        assert!(matches!(messages.borrow()[0], TimerMsg::Start));
        
        // Clear messages and test pause button
        messages.borrow_mut().clear();
        if let Some(pause_button) = &widgets.pause_button {
            pause_button.emit_clicked();
        }
        
        assert_eq!(messages.borrow().len(), 1);
        assert!(matches!(messages.borrow()[0], TimerMsg::Pause));
    }

    #[test]
    fn eye_check_model_should_initialize_hidden() {
        use crate::eye_check_model::EyeCheckModel;
        
        let model = EyeCheckModel::init();
        
        // Should initialize as hidden
        assert!(!model.is_visible());
    }
}
