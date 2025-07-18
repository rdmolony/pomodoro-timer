use crate::timer_model::{TimerModel, TimerMsg, TimerWidgets};

/// Messages that can be sent to the application
/// This enum represents all possible user interactions
#[derive(Debug, Clone, PartialEq)]
pub enum AppMsg {
    Timer(TimerMsg),
}

/// The main application model that holds all state
/// In a larger app, this would contain multiple sub-models
pub struct AppModel {
    pub timer_model: TimerModel,
}

/// Widget references for the main application UI
/// These are held by Relm4 and passed to update_view()
pub struct AppWidgets {
    pub window: Option<gtk::ApplicationWindow>,
    pub main_box: Option<gtk::Box>,
    pub timer_widgets: Option<TimerWidgets>,
    pub header_bar: Option<gtk::HeaderBar>,
    pub settings_button: Option<gtk::Button>,
}

impl AppModel {
    pub fn init() -> Self {
        let timer_model = TimerModel::init();
        
        AppModel {
            timer_model,
        }
    }
    
    pub fn get_timer_model(&self) -> &TimerModel {
        &self.timer_model
    }
    
    pub fn update(&mut self, msg: AppMsg) -> Option<()> {
        match msg {
            AppMsg::Timer(timer_msg) => {
                self.timer_model.update(timer_msg);
                None
            }
        }
    }
}