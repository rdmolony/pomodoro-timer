pub struct Timer {
    duration: u32,
    remaining: u32,
    running: bool,
    tick_callback: Option<Box<dyn FnMut(u32)>>,
    finished_callback: Option<Box<dyn FnMut()>>,
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