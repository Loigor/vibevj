/// Audio analysis module for VibeVJ
/// 
/// Provides real-time audio analysis including:
/// - Audio input capture
/// - FFT analysis
/// - Frequency band extraction (bass, mid, treble)
/// - Beat detection
/// - Audio reactivity for visualizations

pub mod analyzer;
pub mod input;
pub mod frequency;

pub use analyzer::AudioAnalyzer;
pub use input::AudioInput;
pub use frequency::{FrequencyBands, FrequencyData};
