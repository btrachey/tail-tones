use std::f32::consts::PI;
use std::time::Duration;

use rodio::Source;

/// An infinite source that produces a square wave.
///
/// Always has a rate of 48kHz and one channel.
#[derive(Clone, Debug)]
pub struct SquareWave {
    freq: f32,
    num_sample: usize,
}

impl SquareWave {
    /// The frequency of the wave.
    #[inline]
    pub fn new(freq: f32) -> SquareWave {
        SquareWave {
            freq,
            num_sample: 0,
        }
    }
}

impl Iterator for SquareWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);

        let value = 2.0 * PI * self.freq * self.num_sample as f32 / 48000.0;
        let value_sin = value.sin();
        if value_sin == 0.0 {
            Some(0.0)
        } else if value_sin < 0.0 {
            Some(-1.0)
        } else {
            Some(1.0)
        }
        // Some(value.sin())
    }
}

impl Source for SquareWave {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        48000
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
