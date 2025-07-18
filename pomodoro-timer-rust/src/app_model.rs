use crate::timer_model::{TimerModel, TimerMsg, TimerWidgets};
use crate::eye_check_model::{EyeCheckModel, EyeCheckMsg};
use crate::settings_model::{SettingsModel, SettingsMsg};
use relm4::prelude::*;
use gtk::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum AppMsg {
    Timer(TimerMsg),
    EyeCheck(EyeCheckMsg),
    Settings(SettingsMsg),
    ShowSettings,
    HideSettings,
    SaveSettings,
    LoadSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowState {
    pub width: i32,
    pub height: i32,
    pub x: i32,
    pub y: i32,
    pub maximized: bool,
}

impl Default for WindowState {
    fn default() -> Self {
        WindowState {
            width: 400,
            height: 300,
            x: 0,
            y: 0,
            maximized: false,
        }
    }
}

pub struct AppModel {
    pub timer_model: TimerModel,
    pub eye_check_model: EyeCheckModel,
    pub settings_model: SettingsModel,
    pub settings_visible: bool,
    pub eye_check_timer_running: bool,
    pub eye_check_interval_for_testing: Option<u64>, // seconds
    pub completed_sessions: u32,
    pub is_break_mode: bool,
    pub notification_callback: Option<Rc<RefCell<Vec<String>>>>,
    pub window_state: WindowState,
}

pub struct AppWidgets {
    pub window: Option<gtk::ApplicationWindow>,
    pub main_box: Option<gtk::Box>,
    pub timer_widgets: Option<TimerWidgets>,
    pub header_bar: Option<gtk::HeaderBar>,
    pub settings_button: Option<gtk::Button>,
}

impl AppModel {
    pub fn init() -> Self {
        let settings_model = SettingsModel::load().unwrap_or_default();
        let mut timer_model = TimerModel::init();
        
        // Initialize timer with settings
        timer_model.set_duration(settings_model.get_pomodoro_duration());
        
        let eye_check_model = EyeCheckModel::init();
        
        AppModel {
            timer_model,
            eye_check_model,
            settings_model,
            settings_visible: false,
            eye_check_timer_running: false,
            eye_check_interval_for_testing: None,
            completed_sessions: 0,
            is_break_mode: false,
            notification_callback: None,
            window_state: WindowState::default(),
        }
    }
    
    pub fn get_timer_model(&self) -> &TimerModel {
        &self.timer_model
    }
    
    pub fn get_eye_check_model(&self) -> &EyeCheckModel {
        &self.eye_check_model
    }
    
    pub fn get_settings_model(&self) -> &SettingsModel {
        &self.settings_model
    }
    
    pub fn is_settings_visible(&self) -> bool {
        self.settings_visible
    }
    
    pub fn update(&mut self, msg: AppMsg) -> Option<()> {
        match msg {
            AppMsg::Timer(timer_msg) => {
                self.timer_model.update(timer_msg);
                None
            }
            AppMsg::EyeCheck(eye_check_msg) => {
                self.eye_check_model.update(eye_check_msg);
                None
            }
            AppMsg::Settings(settings_msg) => {
                self.settings_model.update(settings_msg);
                None
            }
            AppMsg::ShowSettings => {
                self.settings_visible = true;
                None
            }
            AppMsg::HideSettings => {
                self.settings_visible = false;
                None
            }
            AppMsg::SaveSettings => {
                let _ = self.settings_model.save();
                None
            }
            AppMsg::LoadSettings => {
                if let Ok(settings) = SettingsModel::load() {
                    self.settings_model = settings;
                    // Update timer with new settings
                    self.timer_model.set_duration(self.settings_model.get_pomodoro_duration());
                }
                None
            }
        }
    }
    
    pub fn init_widgets(&self) -> AppWidgets {
        // Create main window
        let window = gtk::ApplicationWindow::new(&gtk::Application::new(
            Some("com.example.pomodoro-timer"),
            Default::default(),
        ));
        window.set_title(Some("Pomodoro Timer"));
        window.set_default_size(400, 300);
        
        // Create header bar
        let header_bar = gtk::HeaderBar::new();
        header_bar.set_title_widget(Some(&gtk::Label::new(Some("Pomodoro Timer"))));
        
        // Create settings button
        let settings_button = gtk::Button::with_label("Settings");
        header_bar.pack_end(&settings_button);
        
        window.set_titlebar(Some(&header_bar));
        
        // Create main content box
        let main_box = gtk::Box::new(gtk::Orientation::Vertical, 12);
        main_box.set_margin_top(12);
        main_box.set_margin_bottom(12);
        main_box.set_margin_start(12);
        main_box.set_margin_end(12);
        
        // Create timer widgets
        let timer_widgets = self.timer_model.init_widgets();
        
        // Add timer widgets to main box
        if let Some(timer_box) = &timer_widgets.main_box {
            main_box.append(timer_box);
        }
        
        window.set_child(Some(&main_box));
        
        AppWidgets {
            window: Some(window),
            main_box: Some(main_box),
            timer_widgets: Some(timer_widgets),
            header_bar: Some(header_bar),
            settings_button: Some(settings_button),
        }
    }
    
    pub fn start_eye_check_timer(&mut self) {
        self.eye_check_timer_running = true;
    }
    
    pub fn stop_eye_check_timer(&mut self) {
        self.eye_check_timer_running = false;
    }
    
    pub fn is_eye_check_timer_running(&self) -> bool {
        self.eye_check_timer_running
    }
    
    pub fn set_eye_check_interval_for_testing(&mut self, seconds: u64) {
        self.eye_check_interval_for_testing = Some(seconds);
    }
    
    pub fn trigger_eye_check(&mut self) {
        use crate::eye_check_model::EyeCheckMsg;
        self.eye_check_model.update(EyeCheckMsg::Show);
    }
    
    pub fn get_completed_sessions(&self) -> u32 {
        self.completed_sessions
    }
    
    pub fn is_break_mode(&self) -> bool {
        self.is_break_mode
    }
    
    pub fn is_long_break_time(&self) -> bool {
        // Every 4 sessions, take a long break
        self.is_break_mode && self.completed_sessions % 4 == 0
    }
    
    pub fn complete_pomodoro_session(&mut self) {
        self.completed_sessions += 1;
        self.is_break_mode = true;
        
        // Set timer to appropriate break duration
        if self.is_long_break_time() {
            self.timer_model.set_duration(self.settings_model.get_long_break_duration());
        } else {
            self.timer_model.set_duration(self.settings_model.get_short_break_duration());
        }
    }
    
    pub fn complete_break(&mut self) {
        self.is_break_mode = false;
        // Reset timer to pomodoro duration
        self.timer_model.set_duration(self.settings_model.get_pomodoro_duration());
    }
    
    pub fn reset_sessions(&mut self) {
        self.completed_sessions = 0;
        self.is_break_mode = false;
        self.timer_model.set_duration(self.settings_model.get_pomodoro_duration());
    }
    
    pub fn set_notification_callback(&mut self, callback: Rc<RefCell<Vec<String>>>) {
        self.notification_callback = Some(callback);
    }
    
    pub fn send_pomodoro_finished_notification(&mut self) {
        if self.settings_model.notifications_enabled {
            self.send_notification("Pomodoro session complete! Time for a break.");
        }
    }
    
    pub fn send_break_finished_notification(&mut self) {
        if self.settings_model.notifications_enabled {
            self.send_notification("Break time is over! Ready for another pomodoro?");
        }
    }
    
    pub fn send_eye_check_notification(&mut self) {
        if self.settings_model.notifications_enabled {
            self.send_notification("20-20-20 eye check reminder: Look at something 20 feet away for 20 seconds.");
        }
    }
    
    pub fn set_notifications_enabled(&mut self, enabled: bool) {
        use crate::settings_model::SettingsMsg;
        self.settings_model.update(SettingsMsg::SetNotificationsEnabled(enabled));
    }
    
    fn send_notification(&self, message: &str) {
        if let Some(callback) = &self.notification_callback {
            callback.borrow_mut().push(message.to_string());
        }
    }
    
    pub fn get_window_state(&self) -> WindowState {
        self.window_state.clone()
    }
    
    pub fn update_window_state(&mut self, state: WindowState) {
        self.window_state = state;
    }
    
    pub fn set_window_maximized(&mut self, maximized: bool) {
        self.window_state.maximized = maximized;
    }
    
    pub fn save_window_state(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(config_path) = Self::get_window_config_path() {
            // Create parent directories if they don't exist
            if let Some(parent) = config_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            let json = serde_json::to_string_pretty(&self.window_state)?;
            fs::write(config_path, json)?;
        }
        Ok(())
    }
    
    pub fn load_window_state(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(config_path) = Self::get_window_config_path() {
            if config_path.exists() {
                let json = fs::read_to_string(config_path)?;
                let window_state: WindowState = serde_json::from_str(&json)?;
                self.window_state = window_state;
            }
        }
        Ok(())
    }
    
    fn get_window_config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|dir| dir.join("pomodoro-timer").join("window.json"))
    }
}