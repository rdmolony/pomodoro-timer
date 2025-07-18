use crate::timer_model::{TimerModel, TimerMsg};
use crate::eye_check_model::{EyeCheckModel, EyeCheckMsg};
use crate::settings_model::{SettingsModel, SettingsMsg};

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

pub struct AppModel {
    pub timer_model: TimerModel,
    pub eye_check_model: EyeCheckModel,
    pub settings_model: SettingsModel,
    pub settings_visible: bool,
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
}