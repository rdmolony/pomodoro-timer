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
        let mut widgets = app_model.init_widgets();
        
        // Use the provided root window instead of creating a new one
        widgets.window = Some(root.clone());
        
        // Show the window
        root.present();

        let main_app = MainApp {
            app_model,
            initialized: true,
        };

        ComponentParts {
            model: main_app,
            widgets,
        }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        self.app_model.update(message);
    }
}