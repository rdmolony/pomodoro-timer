pub struct Timer {
    duration: u32,
    remaining: u32,
}

impl Timer {
    pub fn new() -> Self {
        Self { 
            duration: 0,
            remaining: 0,
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
}