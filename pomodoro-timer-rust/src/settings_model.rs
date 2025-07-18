use serde::{Deserialize, Serialize};

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
}