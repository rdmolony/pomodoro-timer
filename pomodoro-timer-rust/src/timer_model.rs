use crate::timer::Timer;
use relm4::prelude::*;
use gtk::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone, PartialEq)]
pub enum TimerMsg {
    Start,
    Pause,
    Reset,
    Tick,
}

pub struct TimerModel {
    timer: Timer,
}

pub struct TimerWidgets {
    pub main_box: Option<gtk::Box>,
    pub time_label: Option<gtk::Label>,
    pub start_button: Option<gtk::Button>,
    pub pause_button: Option<gtk::Button>,
    pub reset_button: Option<gtk::Button>,
}

impl TimerModel {
    pub fn init() -> Self {
        let mut timer = Timer::new();
        timer.set_duration(1500); // Default 25 minutes
        
        TimerModel { timer }
    }
    
    pub fn get_duration(&self) -> u32 {
        self.timer.get_total_duration()
    }
    
    pub fn get_remaining(&self) -> u32 {
        self.timer.get_remaining()
    }
    
    pub fn is_running(&self) -> bool {
        self.timer.is_running()
    }
    
    pub fn set_duration(&mut self, duration: u32) {
        self.timer.set_duration(duration);
    }
    
    pub fn update(&mut self, msg: TimerMsg) -> Option<()> {
        match msg {
            TimerMsg::Start => {
                self.timer.start();
                None
            }
            TimerMsg::Pause => {
                self.timer.pause();
                None
            }
            TimerMsg::Reset => {
                self.timer.reset();
                None
            }
            TimerMsg::Tick => {
                self.timer.tick();
                None
            }
        }
    }
    
    pub fn init_widgets(&self) -> TimerWidgets {
        let main_box = gtk::Box::new(gtk::Orientation::Vertical, 12);
        
        let time_label = gtk::Label::new(Some("25:00"));
        time_label.set_markup("<span size='xx-large'>25:00</span>");
        
        let button_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        
        let start_button = gtk::Button::with_label("Start");
        let pause_button = gtk::Button::with_label("Pause");
        let reset_button = gtk::Button::with_label("Reset");
        
        button_box.append(&start_button);
        button_box.append(&pause_button);
        button_box.append(&reset_button);
        
        main_box.append(&time_label);
        main_box.append(&button_box);
        
        TimerWidgets {
            main_box: Some(main_box),
            time_label: Some(time_label),
            start_button: Some(start_button),
            pause_button: Some(pause_button),
            reset_button: Some(reset_button),
        }
    }
    
    pub fn update_ui(&self, widgets: &TimerWidgets) {
        if let Some(label) = &widgets.time_label {
            let remaining = self.get_remaining();
            let time_str = Self::format_time(remaining);
            label.set_markup(&format!("<span size='xx-large'>{}</span>", time_str));
        }
    }
    
    fn format_time(seconds: u32) -> String {
        let minutes = seconds / 60;
        let seconds = seconds % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }
    
    pub fn connect_signals(&self, widgets: &TimerWidgets, messages: Rc<RefCell<Vec<TimerMsg>>>) {
        if let Some(start_button) = &widgets.start_button {
            let messages_clone = messages.clone();
            start_button.connect_clicked(move |_| {
                messages_clone.borrow_mut().push(TimerMsg::Start);
            });
        }
        
        if let Some(pause_button) = &widgets.pause_button {
            let messages_clone = messages.clone();
            pause_button.connect_clicked(move |_| {
                messages_clone.borrow_mut().push(TimerMsg::Pause);
            });
        }
        
        if let Some(reset_button) = &widgets.reset_button {
            let messages_clone = messages.clone();
            reset_button.connect_clicked(move |_| {
                messages_clone.borrow_mut().push(TimerMsg::Reset);
            });
        }
    }
}