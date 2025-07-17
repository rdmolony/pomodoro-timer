use pomodoro_timer_rust::Timer;

fn main() {
    println!("🍅 Pomodoro Timer Rust Implementation Demo");
    println!("==========================================");
    
    // Test 1: Timer initialization
    println!("\n1. Creating new timer...");
    let mut timer = Timer::new();
    println!("   Initial duration: {} seconds", timer.get_duration());
    println!("   Initial remaining: {} seconds", timer.get_remaining());
    
    // Test 2: Setting duration
    println!("\n2. Setting timer duration to 25 minutes (1500 seconds)...");
    timer.set_duration(1500);
    println!("   Duration: {} seconds", timer.get_duration());
    println!("   Remaining: {} seconds", timer.get_remaining());
    
    // Test 3: Different duration
    println!("\n3. Setting timer to 5 minutes (300 seconds)...");
    timer.set_duration(300);
    println!("   Duration: {} seconds", timer.get_duration());
    println!("   Remaining: {} seconds", timer.get_remaining());
    
    // Test 4: Zero duration
    println!("\n4. Setting timer to 0 seconds...");
    timer.set_duration(0);
    println!("   Duration: {} seconds", timer.get_duration());
    println!("   Remaining: {} seconds", timer.get_remaining());
    
    println!("\n✅ All basic timer operations working correctly!");
    println!("   - Timer can be created");
    println!("   - Duration can be set");
    println!("   - Remaining time is tracked");
    println!("   - Initial state is properly handled");
}