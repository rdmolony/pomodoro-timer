use crate::app_model::{AppModel, AppMsg, AppWidgets};
use relm4::prelude::*;
use gtk::prelude::*;
use gtk::glib;

/// Main application component following Relm4's reactive architecture
/// 
/// This component implements the Model-View-Update pattern where:
/// - Model: AppModel manages application state
/// - View: GTK4 widgets defined in init()
/// - Update: Messages trigger state changes and UI updates
pub struct MainApp {
    app_model: AppModel,
}

impl MainApp {
    // Only keep what's needed for the Relm4 SimpleComponent
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
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let app_model = AppModel::init();
        let main_app = MainApp {
            app_model,
        };
        
        // Create fresh widgets for this component
        let timer_widgets = main_app.app_model.timer_model.init_widgets();
        
        // Create main content box
        let main_box = gtk::Box::new(gtk::Orientation::Vertical, 12);
        main_box.set_margin_top(12);
        main_box.set_margin_bottom(12);
        main_box.set_margin_start(12);
        main_box.set_margin_end(12);
        
        // Add timer widgets to main box
        if let Some(timer_box) = &timer_widgets.main_box {
            main_box.append(timer_box);
        }
        
        // Create header bar
        let header_bar = gtk::HeaderBar::new();
        header_bar.set_title_widget(Some(&gtk::Label::new(Some("Pomodoro Timer"))));
        
        // Create settings button
        let settings_button = gtk::Button::with_label("Settings");
        header_bar.pack_end(&settings_button);
        
        // Connect button signals to Relm4 messages
        if let Some(start_button) = &timer_widgets.start_button {
            let sender_clone = sender.clone();
            start_button.connect_clicked(move |_| {
                sender_clone.input(crate::app_model::AppMsg::Timer(crate::timer_model::TimerMsg::Start));
            });
        }
        
        if let Some(pause_button) = &timer_widgets.pause_button {
            let sender_clone = sender.clone();
            pause_button.connect_clicked(move |_| {
                sender_clone.input(crate::app_model::AppMsg::Timer(crate::timer_model::TimerMsg::Pause));
            });
        }
        
        if let Some(reset_button) = &timer_widgets.reset_button {
            let sender_clone = sender.clone();
            reset_button.connect_clicked(move |_| {
                sender_clone.input(crate::app_model::AppMsg::Timer(crate::timer_model::TimerMsg::Reset));
            });
        }
        
        // Set up the window
        root.set_child(Some(&main_box));
        root.set_titlebar(Some(&header_bar));
        root.set_title(Some("Pomodoro Timer"));
        root.set_default_size(400, 300);
        
        // Show the window
        root.present();

        ComponentParts {
            model: main_app,
            widgets: AppWidgets {
                window: Some(root),
                main_box: Some(main_box),
                timer_widgets: Some(timer_widgets),
                header_bar: Some(header_bar),
                settings_button: Some(settings_button),
            },
        }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        let was_running = self.app_model.timer_model.is_running();
        
        self.app_model.update(message.clone());
        
        // Set up periodic timer tick when timer starts
        if let crate::app_model::AppMsg::Timer(crate::timer_model::TimerMsg::Start) = message {
            if self.app_model.timer_model.is_running() && !was_running {
                let sender_clone = sender.clone();
                glib::timeout_add_seconds_local(1, move || {
                    sender_clone.input(crate::app_model::AppMsg::Timer(crate::timer_model::TimerMsg::Tick));
                    glib::ControlFlow::Continue
                });
            }
        }
    }
    
    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        // Update the timer display
        if let Some(timer_widgets) = &widgets.timer_widgets {
            self.app_model.timer_model.update_ui(timer_widgets);
        }
    }
}