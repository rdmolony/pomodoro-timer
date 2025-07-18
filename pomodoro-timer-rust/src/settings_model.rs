use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use relm4::prelude::*;
use gtk::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum SettingsMsg {
    SetPomodoroMinutes(u32),
    SetShortBreakMinutes(u32),
    SetLongBreakMinutes(u32),
    SetEyeCheckEnabled(bool),
    SetEyeCheckInterval(u32),
    SetNotificationsEnabled(bool),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsModel {
    pub pomodoro_minutes: u32,
    pub short_break_minutes: u32,
    pub long_break_minutes: u32,
    pub eye_check_enabled: bool,
    pub eye_check_interval: u32, // in minutes
    pub notifications_enabled: bool,
}

pub struct SettingsWidgets {
    pub main_box: Option<gtk::Box>,
    pub pomodoro_spin: Option<gtk::SpinButton>,
    pub short_break_spin: Option<gtk::SpinButton>,
    pub long_break_spin: Option<gtk::SpinButton>,
    pub eye_check_checkbox: Option<gtk::CheckButton>,
    pub eye_check_interval_spin: Option<gtk::SpinButton>,
    pub notifications_checkbox: Option<gtk::CheckButton>,
}

impl Default for SettingsModel {
    fn default() -> Self {
        SettingsModel {
            pomodoro_minutes: 25,
            short_break_minutes: 5,
            long_break_minutes: 15,
            eye_check_enabled: true,
            eye_check_interval: 20,
            notifications_enabled: true,
        }
    }
}

impl SettingsModel {
    pub fn init() -> Self {
        Self::default()
    }
    
    pub fn get_pomodoro_duration(&self) -> u32 {
        self.pomodoro_minutes * 60
    }
    
    pub fn get_short_break_duration(&self) -> u32 {
        self.short_break_minutes * 60
    }
    
    pub fn get_long_break_duration(&self) -> u32 {
        self.long_break_minutes * 60
    }
    
    pub fn get_eye_check_interval(&self) -> u32 {
        self.eye_check_interval * 60
    }
    
    pub fn update(&mut self, msg: SettingsMsg) -> Option<()> {
        match msg {
            SettingsMsg::SetPomodoroMinutes(minutes) => {
                self.pomodoro_minutes = minutes;
                None
            }
            SettingsMsg::SetShortBreakMinutes(minutes) => {
                self.short_break_minutes = minutes;
                None
            }
            SettingsMsg::SetLongBreakMinutes(minutes) => {
                self.long_break_minutes = minutes;
                None
            }
            SettingsMsg::SetEyeCheckEnabled(enabled) => {
                self.eye_check_enabled = enabled;
                None
            }
            SettingsMsg::SetEyeCheckInterval(minutes) => {
                self.eye_check_interval = minutes;
                None
            }
            SettingsMsg::SetNotificationsEnabled(enabled) => {
                self.notifications_enabled = enabled;
                None
            }
        }
    }
    
    fn get_config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|dir| dir.join("pomodoro-timer").join("settings.json"))
    }
    
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(config_path) = Self::get_config_path() {
            // Create parent directories if they don't exist
            if let Some(parent) = config_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            let json = serde_json::to_string_pretty(self)?;
            fs::write(config_path, json)?;
        }
        Ok(())
    }
    
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        if let Some(config_path) = Self::get_config_path() {
            if config_path.exists() {
                let json = fs::read_to_string(config_path)?;
                let settings: SettingsModel = serde_json::from_str(&json)?;
                return Ok(settings);
            }
        }
        Ok(Self::default())
    }
    
    pub fn init_widgets(&self) -> SettingsWidgets {
        let main_box = gtk::Box::new(gtk::Orientation::Vertical, 12);
        main_box.set_margin_top(12);
        main_box.set_margin_bottom(12);
        main_box.set_margin_start(12);
        main_box.set_margin_end(12);
        
        // Timer Settings Section
        let timer_frame = gtk::Frame::new(Some("Timer Settings"));
        let timer_box = gtk::Box::new(gtk::Orientation::Vertical, 6);
        timer_box.set_margin_top(6);
        timer_box.set_margin_bottom(6);
        timer_box.set_margin_start(6);
        timer_box.set_margin_end(6);
        
        // Pomodoro duration
        let pomodoro_hbox = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        let pomodoro_label = gtk::Label::new(Some("Pomodoro Duration (minutes):"));
        pomodoro_label.set_halign(gtk::Align::Start);
        let pomodoro_spin = gtk::SpinButton::with_range(1.0, 60.0, 1.0);
        pomodoro_spin.set_value(self.pomodoro_minutes as f64);
        pomodoro_hbox.append(&pomodoro_label);
        pomodoro_hbox.append(&pomodoro_spin);
        
        // Short break duration
        let short_break_hbox = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        let short_break_label = gtk::Label::new(Some("Short Break Duration (minutes):"));
        short_break_label.set_halign(gtk::Align::Start);
        let short_break_spin = gtk::SpinButton::with_range(1.0, 30.0, 1.0);
        short_break_spin.set_value(self.short_break_minutes as f64);
        short_break_hbox.append(&short_break_label);
        short_break_hbox.append(&short_break_spin);
        
        // Long break duration
        let long_break_hbox = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        let long_break_label = gtk::Label::new(Some("Long Break Duration (minutes):"));
        long_break_label.set_halign(gtk::Align::Start);
        let long_break_spin = gtk::SpinButton::with_range(1.0, 60.0, 1.0);
        long_break_spin.set_value(self.long_break_minutes as f64);
        long_break_hbox.append(&long_break_label);
        long_break_hbox.append(&long_break_spin);
        
        timer_box.append(&pomodoro_hbox);
        timer_box.append(&short_break_hbox);
        timer_box.append(&long_break_hbox);
        timer_frame.set_child(Some(&timer_box));
        
        // Feature Settings Section
        let feature_frame = gtk::Frame::new(Some("Features"));
        let feature_box = gtk::Box::new(gtk::Orientation::Vertical, 6);
        feature_box.set_margin_top(6);
        feature_box.set_margin_bottom(6);
        feature_box.set_margin_start(6);
        feature_box.set_margin_end(6);
        
        // Eye check enabled
        let eye_check_checkbox = gtk::CheckButton::with_label("Enable 20-20-20 Eye Check Reminders");
        eye_check_checkbox.set_active(self.eye_check_enabled);
        
        // Eye check interval
        let eye_check_interval_hbox = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        let eye_check_interval_label = gtk::Label::new(Some("Eye Check Interval (minutes):"));
        eye_check_interval_label.set_halign(gtk::Align::Start);
        let eye_check_interval_spin = gtk::SpinButton::with_range(1.0, 60.0, 1.0);
        eye_check_interval_spin.set_value(self.eye_check_interval as f64);
        eye_check_interval_hbox.append(&eye_check_interval_label);
        eye_check_interval_hbox.append(&eye_check_interval_spin);
        
        // Notifications enabled
        let notifications_checkbox = gtk::CheckButton::with_label("Enable Notifications");
        notifications_checkbox.set_active(self.notifications_enabled);
        
        feature_box.append(&eye_check_checkbox);
        feature_box.append(&eye_check_interval_hbox);
        feature_box.append(&notifications_checkbox);
        feature_frame.set_child(Some(&feature_box));
        
        main_box.append(&timer_frame);
        main_box.append(&feature_frame);
        
        SettingsWidgets {
            main_box: Some(main_box),
            pomodoro_spin: Some(pomodoro_spin),
            short_break_spin: Some(short_break_spin),
            long_break_spin: Some(long_break_spin),
            eye_check_checkbox: Some(eye_check_checkbox),
            eye_check_interval_spin: Some(eye_check_interval_spin),
            notifications_checkbox: Some(notifications_checkbox),
        }
    }
}