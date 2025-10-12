use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use vibevj_common::{Result, VibeVJError};
use std::sync::{Arc, Mutex};

/// Audio input handler
pub struct AudioInput {
    stream: Option<cpal::Stream>,
    sample_buffer: Arc<Mutex<Vec<f32>>>,
    sample_rate: u32,
}

impl AudioInput {
    /// Create a new audio input
    pub fn new() -> Result<Self> {
        Ok(Self {
            stream: None,
            sample_buffer: Arc::new(Mutex::new(Vec::new())),
            sample_rate: 44100,
        })
    }

    /// Start capturing audio from the default input device
    pub fn start(&mut self) -> Result<()> {
        let host = cpal::default_host();
        
        let device = host
            .default_input_device()
            .ok_or_else(|| VibeVJError::AudioError("No input device available".to_string()))?;

        let config = device
            .default_input_config()
            .map_err(|e| VibeVJError::AudioError(format!("Failed to get input config: {}", e)))?;

        self.sample_rate = config.sample_rate().0;

        let sample_buffer = Arc::clone(&self.sample_buffer);
        let err_fn = |err| log::error!("Audio stream error: {}", err);

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => self.build_input_stream::<f32>(&device, &config.into(), sample_buffer, err_fn)?,
            cpal::SampleFormat::I16 => self.build_input_stream::<i16>(&device, &config.into(), sample_buffer, err_fn)?,
            cpal::SampleFormat::U16 => self.build_input_stream::<u16>(&device, &config.into(), sample_buffer, err_fn)?,
            _ => return Err(VibeVJError::AudioError("Unsupported sample format".to_string())),
        };

        stream
            .play()
            .map_err(|e| VibeVJError::AudioError(format!("Failed to play stream: {}", e)))?;

        self.stream = Some(stream);
        Ok(())
    }

    fn build_input_stream<T>(
        &self,
        device: &cpal::Device,
        config: &cpal::StreamConfig,
        sample_buffer: Arc<Mutex<Vec<f32>>>,
        err_fn: impl FnMut(cpal::StreamError) + Send + 'static,
    ) -> Result<cpal::Stream>
    where
        T: cpal::Sample + cpal::SizedSample + cpal::FromSample<f32>,
        f32: cpal::FromSample<T>,
    {
        let stream = device
            .build_input_stream(
                config,
                move |data: &[T], _: &cpal::InputCallbackInfo| {
                    let mut buffer = sample_buffer.lock().unwrap();
                    buffer.clear();
                    
                    for &sample in data {
                        let float_sample: f32 = cpal::Sample::from_sample(sample);
                        buffer.push(float_sample);
                    }
                },
                err_fn,
                None,
            )
            .map_err(|e| VibeVJError::AudioError(format!("Failed to build input stream: {}", e)))?;

        Ok(stream)
    }

    /// Get the current sample buffer
    pub fn get_samples(&self) -> Vec<f32> {
        self.sample_buffer.lock().unwrap().clone()
    }

    /// Get sample rate
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    /// Stop the audio stream
    pub fn stop(&mut self) {
        self.stream = None;
    }
}

impl Default for AudioInput {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            stream: None,
            sample_buffer: Arc::new(Mutex::new(Vec::new())),
            sample_rate: 44100,
        })
    }
}
