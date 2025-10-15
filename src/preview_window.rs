use anyhow::Result;
use std::sync::Arc;
use winit::window::{Window, Fullscreen};
use winit::event::{WindowEvent, KeyEvent};
use winit::keyboard::{KeyCode, PhysicalKey};
use vibevj_engine::{RenderTarget, RenderObject, ModelUniform};
use vibevj_scene::SceneRenderer;

/// Manages a separate preview window for displaying the rendered scene
pub struct PreviewWindow {
    pub window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    pub enabled: bool,
    is_ready: bool, // Track if window is ready to render
    
    // Scene rendering on preview device
    scene_renderer: SceneRenderer,
    render_target: RenderTarget,
    render_objects: Vec<RenderObject>,
    pending_transforms: Vec<glam::Mat4>,
    
    // Blit pipeline to copy render target to surface
    blit_pipeline: wgpu::RenderPipeline,
    blit_bind_group_layout: wgpu::BindGroupLayout,
    blit_bind_group: wgpu::BindGroup,
    sampler: wgpu::Sampler,
}

impl PreviewWindow {
    /// Create a new preview window using the shared device
    pub async fn new(
        window: Arc<Window>,
        device: &wgpu::Device,
        instance: &wgpu::Instance,
    ) -> Result<Self> {
        
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
            .map_err(|e| anyhow::anyhow!("Failed to find suitable adapter for preview window: {}", e))?;
        
        // Create device for preview window
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("Preview Window Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::default(),
                experimental_features: wgpu::ExperimentalFeatures::default(),
                trace: wgpu::Trace::Off,
            })
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
        
        // Create scene renderer for preview window
        let camera = vibevj_engine::Camera::new(
            glam::Vec3::new(3.0, 2.0, 5.0),
            glam::Vec3::ZERO,
            size.width as f32 / size.height as f32,
        );
        let scene_renderer = SceneRenderer::new(&device, surface_format, camera);
        
        // Create render target for 3D scene
        let render_target = RenderTarget::new(
            &device,
            size.width.max(1),
            size.height.max(1),
            surface_format,
            Some("Preview Window Render Target"),
        );
        
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
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &blit_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            cache: None,
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

        // Create initial bind group for blitting
        let blit_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Preview Blit Bind Group"),
            layout: &blit_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&render_target.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        Ok(Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            enabled: true,
            is_ready: false, // Will be set to true after scene objects are initialized
            scene_renderer,
            render_target,
            render_objects: Vec::new(),
            pending_transforms: Vec::new(),
            blit_pipeline,
            blit_bind_group_layout,
            blit_bind_group,
            sampler,
        })
    }

    /// Initialize scene objects on the preview window's device
    /// This creates new render objects from the same mesh/material data
    /// and uploads them to the preview device's GPU
    pub fn init_scene_objects(&mut self, mesh_material_data: Vec<(vibevj_engine::Mesh, vibevj_engine::Material, glam::Mat4)>) {
        self.render_objects = mesh_material_data.into_iter().map(|(mesh, material, transform)| {
            let mut obj = RenderObject::new(mesh, material, transform);
            obj.upload(
                &self.device,
                self.scene_renderer.material_bind_group_layout(),
                self.scene_renderer.model_bind_group_layout(),
            );
            obj
        }).collect();
        self.is_ready = true; // Mark as ready after scene objects are initialized
    }

    /// Update transforms of render objects to match the main scene
    /// Stores transforms to be applied during next render call
    pub fn update_scene(&mut self, transforms: Vec<glam::Mat4>) {
        self.pending_transforms = transforms;
    }
    
    /// Render the 3D scene and blit to window surface in a single pass
    /// This eliminates CPU copying by rendering the scene independently
    pub fn render(&mut self) -> Result<()> {
        // Skip rendering if window is not ready yet
        if !self.is_ready {
            return Ok(());
        }
        
        // Skip if no render objects
        if self.render_objects.is_empty() {
            return Ok(());
        }
        
        // Device polling is no longer needed in wgpu 27
        
        // Apply pending transform updates (only if we have pending transforms)
        if !self.pending_transforms.is_empty() {
            for (i, transform) in self.pending_transforms.iter().enumerate() {
                if i < self.render_objects.len() {
                    self.render_objects[i].transform = *transform;
                    
                    // Update GPU buffer directly
                    if let Some(ref model_buffer) = self.render_objects[i].model_buffer {
                        let model_uniform = ModelUniform {
                            model: transform.to_cols_array_2d(),
                        };
                        self.queue.write_buffer(model_buffer, 0, bytemuck::cast_slice(&[model_uniform]));
                    }
                }
            }
        }
        
        // Create command encoder for both scene and blit
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Preview Window Render Encoder"),
        });
        
        // Update camera
        self.scene_renderer.update_camera(&self.queue);
        
        // Render 3D scene to render target
        let object_refs: Vec<&RenderObject> = self.render_objects.iter().collect();
        self.scene_renderer.render(
            &mut encoder,
            &self.render_target.view,
            &self.render_target.depth_view,
            &object_refs,
            wgpu::Color {
                r: 0.1,
                g: 0.1,
                b: 0.1,
                a: 1.0,
            },
        );
        
        // Get the window's surface texture, handling surface changes
        let output = match self.surface.get_current_texture() {
            Ok(texture) => texture,
            Err(e) => {
                // Surface has changed, need to reconfigure
                let err_str = e.to_string();
                if err_str.contains("surface has changed") || 
                   err_str.contains("swap chain must be updated") ||
                   err_str.contains("Timeout") ||
                   err_str.contains("outdated") {
                    log::info!("Preview window surface changed, reconfiguring...");
                    let size = self.window.inner_size();
                    self.resize(size);
                    // Try again after reconfiguration
                    self.surface
                        .get_current_texture()
                        .map_err(|e| anyhow::anyhow!("Failed to acquire surface texture after reconfigure: {}", e))?
                } else {
                    return Err(anyhow::anyhow!("Failed to acquire surface texture: {}", e));
                }
            }
        };
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Blit the render target to the window surface
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
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            
            // Blit the render target to the surface
            render_pass.set_pipeline(&self.blit_pipeline);
            render_pass.set_bind_group(0, &self.blit_bind_group, &[]);
            render_pass.draw(0..3, 0..1); // Draw fullscreen triangle
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
