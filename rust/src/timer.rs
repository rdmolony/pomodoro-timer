/// Core timer logic - pure business logic with no UI dependencies
/// This demonstrates separation of concerns: timer logic is independent of UI
pub struct Timer {
    duration: u32,      // Total duration in seconds
    remaining: u32,     // Current remaining time in seconds
    running: bool,      // Whether the timer is actively counting down
    tick_callback: Option<Box<dyn FnMut(u32)>>,     // Called every second
    finished_callback: Option<Box<dyn FnMut()>>,    // Called when timer reaches 0
}

impl Timer {
    pub fn new() -> Self {
        Self { 
            duration: 0,
            remaining: 0,
            running: false,
            tick_callback: None,
            finished_callback: None,
        }
    }

    pub fn get_duration(&self) -> u32 {
        self.duration
    }

    pub fn set_duration(&mut self, duration: u32) {
        self.duration = duration;
        self.remaining = duration;
    }

    pub fn get_remaining(&self) -> u32 {
        self.remaining
    }

    pub fn get_total_duration(&self) -> u32 {
        self.duration
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn pause(&mut self) {
        self.running = false;
    }

    pub fn reset(&mut self) {
        self.remaining = self.duration;
        self.running = false;
    }

    pub fn on_tick<F>(&mut self, callback: F)
    where
        F: FnMut(u32) + 'static,
    {
        self.tick_callback = Some(Box::new(callback));
    }

    pub fn tick(&mut self) {
        if self.running && self.remaining > 0 {
            self.remaining -= 1;
            if let Some(callback) = &mut self.tick_callback {
                callback(self.remaining);
            }
            if self.remaining == 0 {
                self.running = false;
                if let Some(callback) = &mut self.finished_callback {
                    callback();
                }
            }
        }
    }

    pub fn on_finished<F>(&mut self, callback: F)
    where
        F: FnMut() + 'static,
    {
        self.finished_callback = Some(Box::new(callback));
    }
}