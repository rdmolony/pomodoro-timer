pub mod timer;

pub use timer::Timer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timer_should_initialize_with_zero_duration() {
        let timer = Timer::new();
        assert_eq!(timer.get_duration(), 0);
    }

    #[test]
    fn timer_should_set_duration_correctly() {
        let mut timer = Timer::new();
        timer.set_duration(1500);
        assert_eq!(timer.get_duration(), 1500);
    }

    #[test]
    fn timer_should_report_remaining_time() {
        let mut timer = Timer::new();
        timer.set_duration(1500);
        assert_eq!(timer.get_remaining(), 1500);
    }

    #[test]
    fn timer_should_report_total_duration() {
        let mut timer = Timer::new();
        timer.set_duration(1500);
        assert_eq!(timer.get_total_duration(), 1500);
    }

    #[test]
    fn timer_should_start_countdown() {
        let mut timer = Timer::new();
        timer.set_duration(1500);
        timer.start();
        assert!(timer.is_running());
    }

    #[test]
    fn timer_should_pause_countdown() {
        let mut timer = Timer::new();
        timer.set_duration(1500);
        timer.start();
        timer.pause();
        assert!(!timer.is_running());
    }
}
