use std::io::{self, Write};
use std::time::Instant;

pub struct ProgressBar {
    pub total: u64, // Change to an unsigned integer with unbound size to accomodate values larger than 2^64 (reached at 739^7)
    pub inverse_total: f64,
    pub current: u64,
    pub width: u8,
    pub threshold: Option<u64>,
    pub filled: u8,
    pub partial: usize,
    pub time: Instant,
}

impl ProgressBar {
    /// Create a new ProgressBar
    pub fn new(total: u64, width: u8) -> Self {
        Self {
            total,
            inverse_total: 1.0 / (total as f64),
            current: 0,
            width,
            threshold: Some(0),
            filled: 0,
            partial: 0,
            time: Instant::now(),
        }
    }

    /// Add an amount to the current count
    pub fn add(&mut self, count: u64) {
        self.current += count;
        match self.threshold {
            Some(threshold) => {
                if self.current >= threshold {
                    self.update();
                }
            },
            None => (),
        }
        self.output();
    }

    /// Update the threshold to reach before updating the output
    pub fn update_threshold(&mut self) {
        if self.current >= self.total { self.threshold = None; }
        let ticks = (self.filled as usize) * 7 + self.partial + 1;
        let total_ticks = self.width * 7;
        self.threshold = Some(((ticks as f64) / (total_ticks as f64) * (self.total as f64)) as u64);
    }

    /// Update the ProgressBar data
    pub fn update(&mut self) {
        let mut proportion = (self.current as f64) / (self.total as f64);
        if proportion > 1.0 { proportion = 1.0; }
        proportion *= self.width as f64;

        self.filled = proportion as u8;
        self.partial = (7.0 * (proportion % 1.0)) as usize;

        self.update_threshold();
    }

    /// Update the ProgressBar output
    pub fn output(&self) {
        self.close();
        print!("Progress: [");

        for _ in 0..self.filled { print!("▊"); }

        if self.filled < self.width {
            print!("{}", [".", "▏", "▎", "▍", "▌", "▋", "▊"][self.partial as usize]);
        }

        for _ in self.filled + 1..self.width { print!("."); }

        let fraction = (self.current as f64) * self.inverse_total;
        let time = self.time.elapsed().as_secs_f64();
        print!("] {:.2}% - {}/{} [{:.4} seconds] [Over in ~{:.4} seconds]",
            100.0 * fraction,
            self.current,
            self.total,
            time,
            time / fraction);

        io::stdout().flush().unwrap();
    }

    pub fn close(&self) {
        print!("\x1B[2K\x1B[G"); // Clear line and move to start
    }

    pub fn results(&self, message: &str) {
        self.close();
        println!("{} {:.2}% - {}/{} [{:.4} seconds]", message, 100.0 * ((self.current as f64) / (self.total as f64)), self.current, self.total, self.time.elapsed().as_secs_f64());
    }
}