use rustfft::{FftPlanner, num_complex::Complex};
use vibevj_common::Result;
use crate::frequency::{FrequencyBands, FrequencyData};

/// Audio analyzer with FFT
pub struct AudioAnalyzer {
    fft_planner: FftPlanner<f32>,
    fft_size: usize,
    window: Vec<f32>,
}

impl AudioAnalyzer {
    /// Create a new audio analyzer
    pub fn new(fft_size: usize) -> Self {
        let window = Self::hann_window(fft_size);
        
        Self {
            fft_planner: FftPlanner::new(),
            fft_size,
            window,
        }
    }

    /// Generate a Hann window for FFT
    fn hann_window(size: usize) -> Vec<f32> {
        (0..size)
            .map(|i| {
                let t = i as f32 / (size - 1) as f32;
                0.5 * (1.0 - (2.0 * std::f32::consts::PI * t).cos())
            })
            .collect()
    }

    /// Analyze audio samples and extract frequency data
    pub fn analyze(&mut self, samples: &[f32]) -> Result<FrequencyData> {
        let mut buffer: Vec<Complex<f32>> = samples
            .iter()
            .take(self.fft_size)
            .enumerate()
            .map(|(i, &s)| Complex::new(s * self.window[i], 0.0))
            .collect();

        // Pad with zeros if needed
        buffer.resize(self.fft_size, Complex::new(0.0, 0.0));

        // Perform FFT
        let fft = self.fft_planner.plan_fft_forward(self.fft_size);
        fft.process(&mut buffer);

        // Calculate magnitudes
        let magnitudes: Vec<f32> = buffer
            .iter()
            .take(self.fft_size / 2)
            .map(|c| c.norm())
            .collect();

        Ok(FrequencyData::new(magnitudes))
    }

    /// Analyze and extract frequency bands
    pub fn analyze_bands(&mut self, samples: &[f32], sample_rate: u32) -> Result<FrequencyBands> {
        let freq_data = self.analyze(samples)?;
        Ok(FrequencyBands::from_frequency_data(&freq_data, sample_rate, self.fft_size))
    }
}

impl Default for AudioAnalyzer {
    fn default() -> Self {
        Self::new(2048)
    }
}
