use relm4::prelude::*;
use gtk::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum EyeCheckMsg {
    Show,
    Dismiss,
    Snooze,
}

pub struct EyeCheckModel {
    visible: bool,
}

impl EyeCheckModel {
    pub fn init() -> Self {
        EyeCheckModel { visible: false }
    }
    
    pub fn is_visible(&self) -> bool {
        self.visible
    }
    
    pub fn update(&mut self, msg: EyeCheckMsg) -> Option<()> {
        match msg {
            EyeCheckMsg::Show => {
                self.visible = true;
                None
            }
            EyeCheckMsg::Dismiss => {
                self.visible = false;
                None
            }
            EyeCheckMsg::Snooze => {
                self.visible = false;
                None
            }
        }
    }
}