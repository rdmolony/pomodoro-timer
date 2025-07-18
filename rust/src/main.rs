use relm4::prelude::*;
use pomodoro_timer_rust::MainApp;

fn main() {
    // Create the Relm4 application
    let app = RelmApp::new("com.example.pomodoro-timer");
    
    println!("🍅 Pomodoro Timer - Rust/Relm4 Implementation");
    println!("===============================================");
    println!();
    println!("Features:");
    println!("  • 25-minute pomodoro timer");
    println!("  • Start/Pause/Reset controls");
    println!("  • Real-time countdown display");
    println!("  • Clean GTK4 interface");
    println!();
    
    // Run the application with Relm4's reactive architecture
    app.run::<MainApp>(());
}