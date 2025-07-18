use relm4::prelude::*;
use pomodoro_timer_rust::MainApp;

fn main() {
    // Create the Relm4 application
    let app = RelmApp::new("com.example.pomodoro-timer");
    
    println!("Starting Pomodoro Timer with Relm4!");
    println!("Features:");
    println!("- 25 minute pomodoro sessions");
    println!("- 5 minute short breaks");
    println!("- 15 minute long breaks (every 4 sessions)");
    println!("- Eye check reminders every 20 minutes");
    println!("- Settings button to configure timers");
    println!("- Window state persistence");
    
    // Run the application
    app.run::<MainApp>(());
}