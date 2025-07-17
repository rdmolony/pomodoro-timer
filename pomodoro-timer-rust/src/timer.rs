pub struct Timer {
    duration: u32,
}

impl Timer {
    pub fn new() -> Self {
        Self { duration: 0 }
    }

    pub fn get_duration(&self) -> u32 {
        self.duration
    }
}