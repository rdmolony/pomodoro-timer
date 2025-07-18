use crate::app_model::{AppModel, AppMsg, AppWidgets};
use relm4::prelude::*;
use gtk::prelude::*;

pub struct MainApp {
    app_model: AppModel,
    initialized: bool,
}

impl MainApp {
    pub fn new(_init_data: ()) -> Self {
        let app_model = AppModel::init();
        MainApp {
            app_model,
            initialized: true,
        }
    }
    
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
    
    pub fn get_app_id(&self) -> &str {
        "com.example.pomodoro-timer"
    }
    
    pub fn setup_window_content(&self, window: &gtk::ApplicationWindow) -> Result<(), Box<dyn std::error::Error>> {
        // Create fresh widgets specifically for this window
        let timer_widgets = self.app_model.timer_model.init_widgets();
        
        // Create a new main box for this window
        let main_box = gtk::Box::new(gtk::Orientation::Vertical, 12);
        main_box.set_margin_top(12);
        main_box.set_margin_bottom(12);
        main_box.set_margin_start(12);
        main_box.set_margin_end(12);
        
        // Add timer widgets to main box
        if let Some(timer_box) = &timer_widgets.main_box {
            main_box.append(timer_box);
        }
        
        // Create a new header bar
        let header_bar = gtk::HeaderBar::new();
        header_bar.set_title_widget(Some(&gtk::Label::new(Some("Pomodoro Timer"))));
        
        // Create settings button
        let settings_button = gtk::Button::with_label("Settings");
        header_bar.pack_end(&settings_button);
        
        // Set up the window
        window.set_child(Some(&main_box));
        window.set_titlebar(Some(&header_bar));
        window.set_title(Some("Pomodoro Timer"));
        window.set_default_size(400, 300);
        
        Ok(())
    }
    
    pub fn get_timer_widgets(&self) -> Option<crate::timer_model::TimerWidgets> {
        let widgets = self.app_model.init_widgets();
        widgets.timer_widgets
    }
}

impl SimpleComponent for MainApp {
    type Init = ();
    type Input = AppMsg;
    type Output = ();
    type Root = gtk::ApplicationWindow;
    type Widgets = AppWidgets;

    fn init_root() -> Self::Root {
        gtk::ApplicationWindow::new(&gtk::Application::new(
            Some("com.example.pomodoro-timer"),
            Default::default(),
        ))
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let app_model = AppModel::init();
        let main_app = MainApp {
            app_model,
            initialized: true,
        };
        
        // Set up the window content
        main_app.setup_window_content(&root).expect("Failed to setup window content");
        
        // Show the window
        root.present();

        let widgets = main_app.app_model.init_widgets();
        ComponentParts {
            model: main_app,
            widgets: AppWidgets {
                window: Some(root),
                main_box: widgets.main_box,
                timer_widgets: widgets.timer_widgets,
                header_bar: widgets.header_bar,
                settings_button: widgets.settings_button,
            },
        }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        self.app_model.update(message);
    }
}