use anyhow::Result;
use std::sync::Arc;
use winit::window::{Window, WindowBuilder, Fullscreen};
use winit::event_loop::EventLoopWindowTarget;
use winit::event::{WindowEvent, KeyEvent};
use winit::keyboard::{KeyCode, PhysicalKey};
use vibevj_engine::RenderTarget;

/// Manages a separate preview window for displaying the rendered scene
pub struct PreviewWindow {
    pub window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    pub enabled: bool,
    blit_pipeline: wgpu::RenderPipeline,
    blit_bind_group_layout: wgpu::BindGroupLayout,
    blit_bind_group: Option<wgpu::BindGroup>,
    sampler: wgpu::Sampler,
}

impl PreviewWindow {
    /// Create a new preview window using the shared device
    pub async fn new<T>(
        event_loop: &EventLoopWindowTarget<T>,
        device: &wgpu::Device,
        instance: &wgpu::Instance,
    ) -> Result<Self> {
        let window = Arc::new(
            WindowBuilder::new()
                .with_title("VibeVJ - Preview (Press F for fullscreen)")
                .with_inner_size(winit::dpi::LogicalSize::new(1280, 720))
                .build(event_loop)?
        );
        
        let size = window.inner_size();
        
        // Create surface for the preview window (using shared device)
        let surface = instance.create_surface(window.clone())
            .map_err(|e| anyhow::anyhow!("Failed to create surface: {}", e))?;
        
        // Get an adapter for this surface and create a dedicated device
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| anyhow::anyhow!("Failed to find suitable adapter for preview window"))?;
        
        // Create device for preview window
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Preview Window Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .map_err(|e| anyhow::anyhow!("Failed to create device for preview window: {}", e))?;
        
        // Get surface capabilities
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
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        
        surface.configure(&device, &config);
        
        // Create sampler for texture sampling
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Preview Blit Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        // Create bind group layout for blit
        let blit_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Preview Blit Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        // Load and compile blit shader
        let blit_shader_source = include_str!("../assets/shaders/blit.wgsl");
        let blit_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Preview Blit Shader"),
            source: wgpu::ShaderSource::Wgsl(blit_shader_source.into()),
        });

        // Create pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Preview Blit Pipeline Layout"),
            bind_group_layouts: &[&blit_bind_group_layout],
            push_constant_ranges: &[],
        });

        // Create blit pipeline
        let blit_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Preview Blit Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &blit_shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &blit_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Ok(Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            enabled: true,
            blit_pipeline,
            blit_bind_group_layout,
            blit_bind_group: None,
            sampler,
        })
    }

    /// Update the bind group to render the given render target  
    /// Note: This creates a texture on the preview device and copies data from the main render target
    pub fn set_render_target(&mut self, render_target: &RenderTarget, main_device: &wgpu::Device, main_queue: &wgpu::Queue) {
        // Copy the render target texture to a buffer on the main device
        let (buffer, padded_bytes_per_row, unpadded_bytes_per_row) = 
            render_target.copy_to_buffer(main_device, main_queue);
        
        let width = render_target.width;
        let height = render_target.height;
        
        // Create or get texture on preview device
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Preview Texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: render_target.format, // Use exact same format!
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        
        // Map the buffer and copy to preview window texture
        let buffer_slice = buffer.slice(..);
        buffer_slice.map_async(wgpu::MapMode::Read, |_| {});
        main_device.poll(wgpu::Maintain::Wait);
        
        {
            let data = buffer_slice.get_mapped_range();
            
            // Unpad if necessary
            let mut unpadded_data = Vec::with_capacity((width * height * 4) as usize);
            for row in 0..height {
                let start = (row * padded_bytes_per_row) as usize;
                let end = start + unpadded_bytes_per_row as usize;
                unpadded_data.extend_from_slice(&data[start..end]);
            }
            
            // Upload to preview device texture
            self.queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                &unpadded_data,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * width),
                    rows_per_image: Some(height),
                },
                wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
            );
        }
        
        buffer.unmap();
        
        // Create bind group with the new texture view
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Preview Blit Bind Group"),
            layout: &self.blit_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
        });
        self.blit_bind_group = Some(bind_group);
    }
    
    /// Render the render target texture to this window
    pub fn render(&self) -> Result<()> {
        // Get the window's surface texture
        let output = self.surface
            .get_current_texture()
            .map_err(|e| anyhow::anyhow!("Failed to acquire surface texture: {}", e))?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create encoder
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Preview Window Render Encoder"),
        });

        // Blit the copied texture to the window surface
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Preview Window Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.02,
                            g: 0.02,
                            b: 0.1,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            
            // If we have a texture to blit, draw it
            if let Some(bind_group) = &self.blit_bind_group {
                render_pass.set_pipeline(&self.blit_pipeline);
                render_pass.set_bind_group(0, bind_group, &[]);
                render_pass.draw(0..3, 0..1); // Draw fullscreen triangle
            }
        }

        self.queue.submit(Some(encoder.finish()));
        output.present();

        Ok(())
    }
    
    /// Handle keyboard input for fullscreen toggle
    pub fn handle_input(&self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    physical_key: PhysicalKey::Code(KeyCode::KeyF),
                    state: winit::event::ElementState::Pressed,
                    ..
                },
                ..
            } => {
                // Toggle fullscreen
                if self.window.fullscreen().is_some() {
                    self.window.set_fullscreen(None);
                    log::info!("Preview window: Exited fullscreen");
                } else {
                    let monitor = self.window.current_monitor();
                    self.window.set_fullscreen(Some(Fullscreen::Borderless(monitor)));
                    log::info!("Preview window: Entered fullscreen");
                }
                true
            }
            _ => false,
        }
    }

    /// Handle window resize
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }
}
