use std::time::{Duration, Instant};

pub struct Delay {
    time: Instant,
    count: u8,
}
impl Delay {
    pub fn new() -> Self {
        Delay {
            time: Instant::now(),
            count: 0,
        }
    }
    pub fn check(&mut self) {
        if self.count != 0 && self.time.elapsed() >= Duration::from_millis((1000 / 60) as u64) {
            self.count -= 1;
            self.time = Instant::now();
        }
    }
    pub fn set_count(&mut self, count: u8) {
        self.count = count;
        self.time = Instant::now();
    }
    pub fn get_count(&self) -> u8 {
        self.count
    }
}
