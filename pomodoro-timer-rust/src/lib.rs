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
}
