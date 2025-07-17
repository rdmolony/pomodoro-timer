use crate::timer::Timer;

pub struct TimerModel {
    timer: Timer,
}

impl TimerModel {
    pub fn init() -> Self {
        let mut timer = Timer::new();
        timer.set_duration(1500); // Default 25 minutes
        
        TimerModel { timer }
    }
    
    pub fn get_duration(&self) -> u32 {
        self.timer.get_total_duration()
    }
    
    pub fn get_remaining(&self) -> u32 {
        self.timer.get_remaining()
    }
    
    pub fn is_running(&self) -> bool {
        self.timer.is_running()
    }
}