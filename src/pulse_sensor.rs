//! Pulse Sensor Raspberry Pi peripheral abstraction

use rand::{thread_rng, Rng};

/// A pulse sensor on a Pi
pub struct PulseSensor {}

impl PulseSensor {
    /// Constructs a new Pulse Sensor instance
    pub fn new() -> Self {
        Self {}
    }

    /// Gets the current BPM of the Pulse Sensor
    pub fn get_current_bpm(&self) -> f32 {
        let mut rng = thread_rng();
        rng.gen_range(60f32..120f32)
    }
}
