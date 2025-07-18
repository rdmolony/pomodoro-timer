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

pub struct EyeCheckWidgets {
    pub window: Option<gtk::Window>,
    pub message_label: Option<gtk::Label>,
    pub dismiss_button: Option<gtk::Button>,
    pub snooze_button: Option<gtk::Button>,
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
    
    pub fn init_widgets(&self) -> EyeCheckWidgets {
        let window = gtk::Window::new();
        window.set_title(Some("Eye Check Reminder"));
        window.set_default_size(800, 600);
        window.set_modal(true);
        window.set_resizable(false);
        
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 20);
        vbox.set_margin_top(100);
        vbox.set_margin_bottom(100);
        vbox.set_margin_start(100);
        vbox.set_margin_end(100);
        
        let message_label = gtk::Label::new(Some("Time for an eye break!\n\nLook at something 20 feet away for 20 seconds."));
        message_label.set_markup("<span size='xx-large'>Time for an eye break!\n\nLook at something 20 feet away for 20 seconds.</span>");
        message_label.set_justify(gtk::Justification::Center);
        
        let button_box = gtk::Box::new(gtk::Orientation::Horizontal, 12);
        button_box.set_halign(gtk::Align::Center);
        
        let dismiss_button = gtk::Button::with_label("I'm done (Dismiss)");
        let snooze_button = gtk::Button::with_label("Remind me later (5 min)");
        
        button_box.append(&dismiss_button);
        button_box.append(&snooze_button);
        
        vbox.append(&message_label);
        vbox.append(&button_box);
        
        window.set_child(Some(&vbox));
        
        EyeCheckWidgets {
            window: Some(window),
            message_label: Some(message_label),
            dismiss_button: Some(dismiss_button),
            snooze_button: Some(snooze_button),
        }
    }
}