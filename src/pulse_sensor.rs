//! Pulse Sensor async code

use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use rppal::i2c::I2c;
use tokio::sync::RwLock;

/// ADS1115 default I2C address
const ADS1115_ADDR: u16 = 0x48;

/// ADS1115 convsersion register
const CONVERSION_REG: u8 = 0x00;
/// ADS1115 config register
const CONFIG_REG: u8 = 0x01;

// ADS1115 configuration bits
/// A0 Single Ended bits
const A0_SINGLE_ENDED: u16 = 0x4000;
/// FSR 4.096v
const FSR_4_096V: u16 = 0x0200;
/// Single Shot Mode
const MODE_SINGLE_SHOT: u16 = 0x0100;
/// Data rate
const DATA_RATE_128SPS: u16 = 0x0080;
/// Start sampling
const OS_START_SINGLE: u16 = 0x8000;

// Parameters for BPM calculation
/// How often we sample
const SAMPLE_INTERVAL_MS: u64 = 10;
/// How many seconds before we calculate BPM
const BPM_CALCULATION_PERIOD: Duration = Duration::from_secs(5);
/// Beat peak threshold
const THRESHOLD: f32 = 2.7;

/// An asynchronous pulsesensor that can control a RwLock contained BPM variable
pub struct PulseSensor {
    /// The i2c chip reading the pulse
    i2c: I2c,
    /// RwLocked BPM
    bpm: Arc<RwLock<f32>>,
}

impl PulseSensor {
    /// Creates a new pulsesensor
    pub fn new(bpm: Arc<RwLock<f32>>) -> rppal::i2c::Result<Self> {
        let mut i2c = I2c::new()?;
        i2c.set_slave_address(ADS1115_ADDR)?;

        Ok(Self { i2c, bpm })
    }

    /// Begins running the pulsesensor BPM calculation logic, will block the current thread so it's
    /// advised that you spawn a new thread to handler BPM calculation and refer to the RwLock for
    /// BPM info
    pub async fn run(mut self) -> rppal::i2c::Result<()> {
        let mut beat_count = 0;
        let mut last_beat_time = Instant::now();
        let mut start_time = Instant::now();

        let bpm_clone = self.bpm.clone();

        loop {
            let config = OS_START_SINGLE
                | A0_SINGLE_ENDED
                | FSR_4_096V
                | MODE_SINGLE_SHOT
                | DATA_RATE_128SPS;

            let config_bytes = config.to_be_bytes();
            self.i2c
                .write(&[CONFIG_REG, config_bytes[0], config_bytes[1]])?;

            std::thread::sleep(Duration::from_millis(10));

            let mut buffer = [0; 2];
            self.i2c.write_read(&[CONVERSION_REG], &mut buffer)?;
            let value = i16::from_be_bytes(buffer);

            let voltage = value as f32 * 4.096 / 32768.0;

            if voltage > THRESHOLD && last_beat_time.elapsed() > Duration::from_millis(300) {
                beat_count += 1;
                last_beat_time = Instant::now();
            }

            if start_time.elapsed() >= BPM_CALCULATION_PERIOD {
                let bpm = (beat_count as f32 / BPM_CALCULATION_PERIOD.as_secs_f32()) * 60.0;
                tracing::info!("Current BPM: {bpm}");
                *bpm_clone.write().await = bpm;

                beat_count = 0;
                last_beat_time = Instant::now();
                start_time = Instant::now();
            }

            std::thread::sleep(Duration::from_millis(SAMPLE_INTERVAL_MS));
        }
    }
}
