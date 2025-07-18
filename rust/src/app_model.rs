use crate::timer_model::{TimerModel, TimerMsg, TimerWidgets};

#[derive(Debug, Clone, PartialEq)]
pub enum AppMsg {
    Timer(TimerMsg),
}


pub struct AppModel {
    pub timer_model: TimerModel,
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