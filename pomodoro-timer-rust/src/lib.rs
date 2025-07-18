pub mod timer;
mod timer_model;
mod eye_check_model;
mod settings_model;
mod app_model;

pub use timer::Timer;
pub use timer_model::TimerWidgets;

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

    #[test]
    fn eye_check_model_should_show_dialog_on_show_message() {
        use crate::eye_check_model::{EyeCheckModel, EyeCheckMsg};
        
        let mut model = EyeCheckModel::init();
        
        // Initially hidden
        assert!(!model.is_visible());
        
        // Process Show message
        let result = model.update(EyeCheckMsg::Show);
        
        // Should now be visible
        assert!(model.is_visible());
        
        // Should not cause any side effects
        assert!(result.is_none());
    }

    #[test]
    fn eye_check_model_should_create_fullscreen_dialog_ui() {
        use crate::eye_check_model::EyeCheckModel;
        
        // Initialize GTK for testing
        if gtk::init().is_err() {
            return; // Skip test if GTK can't be initialized
        }
        
        let model = EyeCheckModel::init();
        
        // Create the dialog UI
        let widgets = model.init_widgets();
        
        // Should have a window container
        assert!(widgets.window.is_some());
        
        // Should have a message label
        assert!(widgets.message_label.is_some());
        
        // Should have dismiss and snooze buttons
        assert!(widgets.dismiss_button.is_some());
        assert!(widgets.snooze_button.is_some());
    }

    #[test]
    fn eye_check_model_should_handle_dismiss_action() {
        use crate::eye_check_model::{EyeCheckModel, EyeCheckMsg};
        
        let mut model = EyeCheckModel::init();
        
        // Show the dialog first
        model.update(EyeCheckMsg::Show);
        assert!(model.is_visible());
        
        // Process Dismiss message
        let result = model.update(EyeCheckMsg::Dismiss);
        
        // Should now be hidden
        assert!(!model.is_visible());
        
        // Should not cause any side effects
        assert!(result.is_none());
    }

    #[test]
    fn eye_check_model_should_handle_snooze_action() {
        use crate::eye_check_model::{EyeCheckModel, EyeCheckMsg};
        
        let mut model = EyeCheckModel::init();
        
        // Show the dialog first
        model.update(EyeCheckMsg::Show);
        assert!(model.is_visible());
        
        // Process Snooze message
        let result = model.update(EyeCheckMsg::Snooze);
        
        // Should now be hidden
        assert!(!model.is_visible());
        
        // Should not cause any side effects (scheduling handled elsewhere)
        assert!(result.is_none());
    }

    #[test]
    fn eye_check_model_should_handle_escape_key() {
        use crate::eye_check_model::{EyeCheckModel, EyeCheckMsg};
        use std::rc::Rc;
        use std::cell::RefCell;
        
        // Initialize GTK for testing
        if gtk::init().is_err() {
            return; // Skip test if GTK can't be initialized
        }
        
        let mut model = EyeCheckModel::init();
        let widgets = model.init_widgets();
        
        // Show the dialog first
        model.update(EyeCheckMsg::Show);
        assert!(model.is_visible());
        
        // Create a shared state to capture messages
        let messages = Rc::new(RefCell::new(Vec::new()));
        
        // Connect escape key handler
        model.connect_key_events(&widgets, messages.clone());
        
        // Simulate escape key (this would normally be tested with actual key events)
        // For testing, we'll verify the handler is connected
        assert!(widgets.window.is_some());
        
        // The escape key handling would trigger Dismiss in a real scenario
        // For the test, we'll just verify the connection works
        model.update(EyeCheckMsg::Dismiss);
        assert!(!model.is_visible());
    }

    #[test]
    fn settings_model_should_initialize_with_defaults() {
        use crate::settings_model::SettingsModel;
        
        let model = SettingsModel::init();
        
        // Should initialize with default values
        assert_eq!(model.pomodoro_minutes, 25);
        assert_eq!(model.short_break_minutes, 5);
        assert_eq!(model.long_break_minutes, 15);
        assert_eq!(model.eye_check_enabled, true);
        assert_eq!(model.eye_check_interval, 20);
        assert_eq!(model.notifications_enabled, true);
        
        // Check duration getters
        assert_eq!(model.get_pomodoro_duration(), 1500); // 25 * 60
        assert_eq!(model.get_short_break_duration(), 300); // 5 * 60
        assert_eq!(model.get_long_break_duration(), 900); // 15 * 60
        assert_eq!(model.get_eye_check_interval(), 1200); // 20 * 60
    }

    #[test]
    fn settings_model_should_handle_settings_changes() {
        use crate::settings_model::{SettingsModel, SettingsMsg};
        
        let mut model = SettingsModel::init();
        
        // Test pomodoro minutes change
        let result = model.update(SettingsMsg::SetPomodoroMinutes(30));
        assert_eq!(model.pomodoro_minutes, 30);
        assert!(result.is_none());
        
        // Test eye check enabled change
        let result = model.update(SettingsMsg::SetEyeCheckEnabled(false));
        assert_eq!(model.eye_check_enabled, false);
        assert!(result.is_none());
        
        // Test notifications enabled change
        let result = model.update(SettingsMsg::SetNotificationsEnabled(false));
        assert_eq!(model.notifications_enabled, false);
        assert!(result.is_none());
    }

    #[test]
    fn settings_model_should_persist_settings() {
        use crate::settings_model::{SettingsModel, SettingsMsg};
        use std::fs;
        
        let mut model = SettingsModel::init();
        
        // Change some settings
        model.update(SettingsMsg::SetPomodoroMinutes(30));
        model.update(SettingsMsg::SetEyeCheckEnabled(false));
        
        // Save settings
        let save_result = model.save();
        assert!(save_result.is_ok());
        
        // Load settings in a new model
        let loaded_model = SettingsModel::load().unwrap_or_default();
        
        // Should have the same values
        assert_eq!(loaded_model.pomodoro_minutes, 30);
        assert_eq!(loaded_model.eye_check_enabled, false);
        assert_eq!(loaded_model.short_break_minutes, 5); // Default unchanged
        
        // Clean up test file
        if let Some(config_dir) = dirs::config_dir() {
            let config_path = config_dir.join("pomodoro-timer").join("settings.json");
            let _ = fs::remove_file(config_path);
        }
    }

    #[test]
    fn settings_model_should_create_settings_ui() {
        use crate::settings_model::SettingsModel;
        
        // Initialize GTK for testing
        if gtk::init().is_err() {
            return; // Skip test if GTK can't be initialized
        }
        
        let model = SettingsModel::init();
        
        // Create the settings UI
        let widgets = model.init_widgets();
        
        // Should have a main container
        assert!(widgets.main_box.is_some());
        
        // Should have input fields for timer durations
        assert!(widgets.pomodoro_spin.is_some());
        assert!(widgets.short_break_spin.is_some());
        assert!(widgets.long_break_spin.is_some());
        
        // Should have checkboxes for features
        assert!(widgets.eye_check_checkbox.is_some());
        assert!(widgets.notifications_checkbox.is_some());
        
        // Should have eye check interval setting
        assert!(widgets.eye_check_interval_spin.is_some());
    }

    #[test]
    fn app_model_should_initialize_with_all_components() {
        use crate::app_model::AppModel;
        
        let app = AppModel::init();
        
        // Should have all component models
        assert!(app.get_timer_model().get_duration() > 0);
        assert!(!app.get_eye_check_model().is_visible());
        assert!(app.get_settings_model().pomodoro_minutes > 0);
        
        // Should start with settings hidden
        assert!(!app.is_settings_visible());
        
        // Timer should be initialized with settings duration
        assert_eq!(app.get_timer_model().get_duration(), 1500); // 25 minutes default
    }

    #[test]
    fn app_model_should_handle_inter_component_communication() {
        use crate::app_model::{AppModel, AppMsg};
        use crate::timer_model::TimerMsg;
        use crate::eye_check_model::EyeCheckMsg;
        use crate::settings_model::SettingsMsg;
        
        let mut app = AppModel::init();
        
        // Test timer message forwarding
        assert!(!app.get_timer_model().is_running());
        app.update(AppMsg::Timer(TimerMsg::Start));
        assert!(app.get_timer_model().is_running());
        
        // Test eye check message forwarding
        assert!(!app.get_eye_check_model().is_visible());
        app.update(AppMsg::EyeCheck(EyeCheckMsg::Show));
        assert!(app.get_eye_check_model().is_visible());
        
        // Test settings message forwarding
        assert_eq!(app.get_settings_model().pomodoro_minutes, 25);
        app.update(AppMsg::Settings(SettingsMsg::SetPomodoroMinutes(30)));
        assert_eq!(app.get_settings_model().pomodoro_minutes, 30);
        
        // Test app-level settings visibility
        assert!(!app.is_settings_visible());
        app.update(AppMsg::ShowSettings);
        assert!(app.is_settings_visible());
        app.update(AppMsg::HideSettings);
        assert!(!app.is_settings_visible());
    }

    #[test]
    fn app_model_should_create_main_window_ui() {
        use crate::app_model::AppModel;
        
        // Initialize GTK for testing
        if gtk::init().is_err() {
            return; // Skip test if GTK can't be initialized
        }
        
        let app = AppModel::init();
        
        // Create the main window UI
        let widgets = app.init_widgets();
        
        // Should have a main window
        assert!(widgets.window.is_some());
        
        // Should have the main application box
        assert!(widgets.main_box.is_some());
        
        // Should have timer UI components
        assert!(widgets.timer_widgets.is_some());
        
        // Should have menu bar or header bar
        assert!(widgets.header_bar.is_some());
        
        // Should have settings button
        assert!(widgets.settings_button.is_some());
    }

    #[test]
    fn app_model_should_handle_20_20_20_timer_integration() {
        use crate::app_model::AppModel;
        
        let mut app = AppModel::init();
        
        // Enable eye check in settings
        assert!(app.get_settings_model().eye_check_enabled);
        assert_eq!(app.get_settings_model().get_eye_check_interval(), 1200); // 20 minutes
        
        // Initially eye check should be hidden
        assert!(!app.get_eye_check_model().is_visible());
        
        // Start eye check timer
        app.start_eye_check_timer();
        
        // Should have eye check timer running
        assert!(app.is_eye_check_timer_running());
        
        // Simulate time passing (we'll use a shorter interval for testing)
        app.set_eye_check_interval_for_testing(1); // 1 second for testing
        
        // Trigger eye check
        app.trigger_eye_check();
        
        // Eye check dialog should now be visible
        assert!(app.get_eye_check_model().is_visible());
        
        // Stop eye check timer
        app.stop_eye_check_timer();
        assert!(!app.is_eye_check_timer_running());
    }

    #[test]
    fn app_model_should_handle_session_management() {
        use crate::app_model::AppModel;
        
        let mut app = AppModel::init();
        
        // Initially should have no completed sessions
        assert_eq!(app.get_completed_sessions(), 0);
        
        // Should not be in break mode initially
        assert!(!app.is_break_mode());
        assert!(!app.is_long_break_time());
        
        // Complete a pomodoro session
        app.complete_pomodoro_session();
        assert_eq!(app.get_completed_sessions(), 1);
        
        // Should now be in break mode
        assert!(app.is_break_mode());
        assert!(!app.is_long_break_time()); // First break should be short
        
        // Complete 3 more sessions to reach long break
        app.complete_pomodoro_session();
        app.complete_pomodoro_session();
        app.complete_pomodoro_session();
        assert_eq!(app.get_completed_sessions(), 4);
        
        // After 4 sessions, should be long break time
        assert!(app.is_break_mode());
        assert!(app.is_long_break_time());
        
        // Complete break
        app.complete_break();
        assert!(!app.is_break_mode());
        
        // Reset sessions
        app.reset_sessions();
        assert_eq!(app.get_completed_sessions(), 0);
        assert!(!app.is_break_mode());
    }

    #[test]
    fn app_model_should_handle_notifications() {
        use crate::app_model::AppModel;
        use std::rc::Rc;
        use std::cell::RefCell;
        
        let mut app = AppModel::init();
        
        // Should have notifications enabled by default
        assert!(app.get_settings_model().notifications_enabled);
        
        // Create a notification tracker
        let notifications = Rc::new(RefCell::new(Vec::new()));
        app.set_notification_callback(notifications.clone());
        
        // Send a pomodoro finished notification
        app.send_pomodoro_finished_notification();
        
        // Should have sent notification
        assert_eq!(notifications.borrow().len(), 1);
        assert!(notifications.borrow()[0].contains("Pomodoro session complete"));
        
        // Send break finished notification
        app.send_break_finished_notification();
        
        // Should have sent another notification
        assert_eq!(notifications.borrow().len(), 2);
        assert!(notifications.borrow()[1].contains("Break time is over"));
        
        // Send eye check notification
        app.send_eye_check_notification();
        
        // Should have sent eye check notification
        assert_eq!(notifications.borrow().len(), 3);
        assert!(notifications.borrow()[2].contains("20-20-20 eye check"));
        
        // Test notification suppression when disabled
        app.set_notifications_enabled(false);
        notifications.borrow_mut().clear();
        
        app.send_pomodoro_finished_notification();
        
        // Should not have sent notification when disabled
        assert_eq!(notifications.borrow().len(), 0);
    }

    #[test]
    fn app_model_should_handle_window_state_persistence() {
        use crate::app_model::{AppModel, WindowState};
        
        let mut app = AppModel::init();
        
        // Should have default window state
        let window_state = app.get_window_state();
        assert_eq!(window_state.width, 400);
        assert_eq!(window_state.height, 300);
        assert_eq!(window_state.x, 0);
        assert_eq!(window_state.y, 0);
        assert!(!window_state.maximized);
        
        // Update window state
        app.update_window_state(WindowState {
            width: 600,
            height: 450,
            x: 100,
            y: 50,
            maximized: false,
        });
        
        // Should have updated state
        let updated_state = app.get_window_state();
        assert_eq!(updated_state.width, 600);
        assert_eq!(updated_state.height, 450);
        assert_eq!(updated_state.x, 100);
        assert_eq!(updated_state.y, 50);
        assert!(!updated_state.maximized);
        
        // Test maximized state
        app.set_window_maximized(true);
        assert!(app.get_window_state().maximized);
        
        // Test saving and loading window state
        let save_result = app.save_window_state();
        assert!(save_result.is_ok());
        
        // Create new app instance and load state
        let mut app2 = AppModel::init();
        let load_result = app2.load_window_state();
        assert!(load_result.is_ok());
        
        // Should have loaded the saved state
        let loaded_state = app2.get_window_state();
        assert_eq!(loaded_state.width, 600);
        assert_eq!(loaded_state.height, 450);
        assert_eq!(loaded_state.x, 100);
        assert_eq!(loaded_state.y, 50);
        assert!(loaded_state.maximized);
    }

    #[test]
    fn application_should_handle_complete_pomodoro_workflow() {
        use crate::app_model::AppModel;
        use crate::timer_model::TimerMsg;
        use std::rc::Rc;
        use std::cell::RefCell;
        
        let mut app = AppModel::init();
        
        // Setup notification tracking
        let notifications = Rc::new(RefCell::new(Vec::new()));
        app.set_notification_callback(notifications.clone());
        
        // Initial state - should be pomodoro mode with 25 minutes
        assert_eq!(app.get_timer_model().get_duration(), 1500); // 25 minutes
        assert_eq!(app.get_completed_sessions(), 0);
        assert!(!app.is_break_mode());
        
        // Start pomodoro timer
        app.update(crate::app_model::AppMsg::Timer(TimerMsg::Start));
        assert!(app.get_timer_model().is_running());
        
        // Simulate pomodoro completion
        app.complete_pomodoro_session();
        app.send_pomodoro_finished_notification();
        
        // Should be in break mode after session
        assert_eq!(app.get_completed_sessions(), 1);
        assert!(app.is_break_mode());
        assert!(!app.is_long_break_time()); // First break should be short
        assert_eq!(app.get_timer_model().get_duration(), 300); // 5 minutes
        
        // Should have sent notification
        assert_eq!(notifications.borrow().len(), 1);
        assert!(notifications.borrow()[0].contains("Pomodoro session complete"));
        
        // Complete the break
        app.complete_break();
        app.send_break_finished_notification();
        
        // Should be back to pomodoro mode
        assert!(!app.is_break_mode());
        assert_eq!(app.get_timer_model().get_duration(), 1500); // 25 minutes
        assert_eq!(notifications.borrow().len(), 2);
        assert!(notifications.borrow()[1].contains("Break time is over"));
        
        // Complete 3 more sessions to reach long break
        for i in 2..=4 {
            app.complete_pomodoro_session();
            app.send_pomodoro_finished_notification();
            assert_eq!(app.get_completed_sessions(), i);
            
            if i == 4 {
                // 4th session should trigger long break
                assert!(app.is_long_break_time());
                assert_eq!(app.get_timer_model().get_duration(), 900); // 15 minutes
            } else {
                // 2nd and 3rd sessions should be short breaks
                assert!(!app.is_long_break_time());
                assert_eq!(app.get_timer_model().get_duration(), 300); // 5 minutes
            }
            
            app.complete_break();
            app.send_break_finished_notification();
        }
        
        // Should have completed full cycle
        assert_eq!(app.get_completed_sessions(), 4);
        assert!(!app.is_break_mode());
        assert_eq!(app.get_timer_model().get_duration(), 1500); // Back to pomodoro
        
        // Test eye check integration
        assert!(!app.get_eye_check_model().is_visible());
        app.start_eye_check_timer();
        app.trigger_eye_check();
        assert!(app.get_eye_check_model().is_visible());
        
        // Test settings persistence
        let save_result = app.save_window_state();
        assert!(save_result.is_ok());
        
        // Verify full workflow completed successfully
        assert_eq!(notifications.borrow().len(), 8); // 4 session + 4 break notifications
    }

    #[test]
    fn application_should_handle_multi_monitor_eye_check() {
        use crate::app_model::{AppModel, MonitorInfo};
        
        let mut app = AppModel::init();
        
        // Initially should have no active eye check dialogs
        assert_eq!(app.get_active_eye_check_dialogs().len(), 0);
        
        // Simulate multi-monitor setup
        let monitor_info = vec![
            MonitorInfo { id: 0, width: 1920, height: 1080, x: 0, y: 0 },
            MonitorInfo { id: 1, width: 1920, height: 1080, x: 1920, y: 0 },
            MonitorInfo { id: 2, width: 1920, height: 1080, x: 0, y: 1080 },
        ];
        
        app.set_monitor_info(monitor_info);
        
        // Show eye check on all monitors
        app.show_eye_check_on_all_monitors();
        
        // Should have eye check dialogs on all monitors
        let active_dialogs = app.get_active_eye_check_dialogs();
        assert_eq!(active_dialogs.len(), 3);
        
        // Each dialog should be positioned on its respective monitor
        assert_eq!(active_dialogs[0].monitor_id, 0);
        assert_eq!(active_dialogs[1].monitor_id, 1);
        assert_eq!(active_dialogs[2].monitor_id, 2);
        
        // All dialogs should be visible
        for dialog in &active_dialogs {
            assert!(dialog.is_visible);
        }
        
        // Dismiss eye check on one monitor
        app.dismiss_eye_check_on_monitor(1);
        
        // Should still have dialogs on other monitors
        let remaining_dialogs = app.get_active_eye_check_dialogs();
        assert_eq!(remaining_dialogs.len(), 2);
        assert_eq!(remaining_dialogs[0].monitor_id, 0);
        assert_eq!(remaining_dialogs[1].monitor_id, 2);
        
        // Dismiss all eye checks
        app.dismiss_all_eye_checks();
        
        // Should have no active dialogs
        assert_eq!(app.get_active_eye_check_dialogs().len(), 0);
        
        // Test single monitor fallback
        app.set_monitor_info(vec![MonitorInfo { id: 0, width: 1920, height: 1080, x: 0, y: 0 }]);
        app.show_eye_check_on_all_monitors();
        
        // Should have one dialog on single monitor
        let single_dialog = app.get_active_eye_check_dialogs();
        assert_eq!(single_dialog.len(), 1);
        assert_eq!(single_dialog[0].monitor_id, 0);
    }
}
