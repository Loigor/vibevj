/// Frequency data from FFT analysis
#[derive(Debug, Clone)]
pub struct FrequencyData {
    pub magnitudes: Vec<f32>,
}

impl FrequencyData {
    pub fn new(magnitudes: Vec<f32>) -> Self {
        Self { magnitudes }
    }

    /// Get magnitude at a specific frequency bin
    pub fn magnitude_at(&self, index: usize) -> f32 {
        self.magnitudes.get(index).copied().unwrap_or(0.0)
    }

    /// Get the peak frequency bin
    pub fn peak_bin(&self) -> usize {
        self.magnitudes
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0)
    }

    /// Get the average magnitude
    pub fn average(&self) -> f32 {
        if self.magnitudes.is_empty() {
            return 0.0;
        }
        self.magnitudes.iter().sum::<f32>() / self.magnitudes.len() as f32
    }
}

/// Frequency bands for audio-reactive visualizations
#[derive(Debug, Clone, Copy)]
pub struct FrequencyBands {
    /// Sub-bass (20-60 Hz)
    pub sub_bass: f32,
    /// Bass (60-250 Hz)
    pub bass: f32,
    /// Low midrange (250-500 Hz)
    pub low_mid: f32,
    /// Midrange (500-2000 Hz)
    pub mid: f32,
    /// High midrange (2000-4000 Hz)
    pub high_mid: f32,
    /// Presence (4000-6000 Hz)
    pub presence: f32,
    /// Brilliance (6000-20000 Hz)
    pub brilliance: f32,
}

impl FrequencyBands {
    /// Create frequency bands from frequency data
    pub fn from_frequency_data(data: &FrequencyData, sample_rate: u32, fft_size: usize) -> Self {
        let bin_width = sample_rate as f32 / fft_size as f32;

        // Helper to sum magnitudes in a frequency range
        let sum_range = |low_freq: f32, high_freq: f32| -> f32 {
            let low_bin = (low_freq / bin_width) as usize;
            let high_bin = (high_freq / bin_width) as usize;
            
            data.magnitudes
                .iter()
                .skip(low_bin)
                .take(high_bin - low_bin)
                .sum::<f32>()
                / (high_bin - low_bin).max(1) as f32
        };

        Self {
            sub_bass: sum_range(20.0, 60.0),
            bass: sum_range(60.0, 250.0),
            low_mid: sum_range(250.0, 500.0),
            mid: sum_range(500.0, 2000.0),
            high_mid: sum_range(2000.0, 4000.0),
            presence: sum_range(4000.0, 6000.0),
            brilliance: sum_range(6000.0, 20000.0),
        }
    }

    /// Get the overall energy level
    pub fn energy(&self) -> f32 {
        (self.sub_bass + self.bass + self.low_mid + self.mid + 
         self.high_mid + self.presence + self.brilliance) / 7.0
    }

    /// Get bass energy (sub-bass + bass)
    pub fn bass_energy(&self) -> f32 {
        (self.sub_bass + self.bass) / 2.0
    }

    /// Get treble energy (presence + brilliance)
    pub fn treble_energy(&self) -> f32 {
        (self.presence + self.brilliance) / 2.0
    }
}

impl Default for FrequencyBands {
    fn default() -> Self {
        Self {
            sub_bass: 0.0,
            bass: 0.0,
            low_mid: 0.0,
            mid: 0.0,
            high_mid: 0.0,
            presence: 0.0,
            brilliance: 0.0,
        }
    }
}
