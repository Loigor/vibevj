use wgpu::util::DeviceExt;
use winit::window::Window;
use vibevj_common::{Result, VibeVJError};

/// Main renderer managing the WGPU device, queue, and surface
pub struct Renderer {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
}

impl Renderer {
    /// Create a new renderer instance
    pub async fn new(window: std::sync::Arc<Window>) -> Result<Self> {
        let size = window.inner_size();

        // Create WGPU instance
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // Create surface
        let surface = instance.create_surface(window.clone())
            .map_err(|e| VibeVJError::RenderError(format!("Failed to create surface: {}", e)))?;

        // Request adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| VibeVJError::RenderError("Failed to find suitable adapter".to_string()))?;

        // Request device and queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("VibeVJ Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .map_err(|e| VibeVJError::RenderError(format!("Failed to create device: {}", e)))?;

        // Configure surface
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        Ok(Self {
            surface,
            device,
            queue,
            config,
            size,
        })
    }

    /// Resize the surface
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    /// Get the current surface texture
    pub fn get_current_texture(&self) -> Result<wgpu::SurfaceTexture> {
        self.surface
            .get_current_texture()
            .map_err(|e| VibeVJError::RenderError(format!("Failed to acquire surface texture: {}", e)))
    }

    /// Get surface format
    pub fn surface_format(&self) -> wgpu::TextureFormat {
        self.config.format
    }

    /// Get aspect ratio
    pub fn aspect_ratio(&self) -> f32 {
        self.size.width as f32 / self.size.height as f32
    }
}
