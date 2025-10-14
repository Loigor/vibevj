use anyhow::Result;
use std::sync::Arc;
use std::time::Instant;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use vibevj_common::TimeInfo;
use vibevj_engine::{Renderer, RenderObject, Material, mesh_gen, Camera, RenderTarget};
use vibevj_gui::GuiApp;
use vibevj_audio::{AudioInput, AudioAnalyzer, FrequencyBands};
use vibevj_scene::{Scene, SceneRenderer};
use vibevj_scripting::ScriptEngine;
use glam::{Mat4, Vec3};
use crate::preview_window::PreviewWindow;
use crate::scene_state::SceneState;

/// Main VibeVJ application
pub struct VibeVJApp {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    gui: Option<GuiApp>,
    egui_state: Option<egui_winit::State>,
    wgpu_instance: wgpu::Instance,
    
    // 3D rendering
    scene_renderer: Option<SceneRenderer>,
    scene_state: SceneState,
    render_target: Option<RenderTarget>,
    
    // Preview window
    preview_window: Option<PreviewWindow>,
    show_preview_window: bool,
    
    // Application state
    scene: Scene,
    audio_input: AudioInput,
    audio_analyzer: AudioAnalyzer,
    script_engine: ScriptEngine,
    selected_audio_device: Option<String>,
    
    // Time tracking
    start_time: Instant,
    last_frame_time: Instant,
    frame_count: u64,
    
    // Audio data
    frequency_bands: FrequencyBands,
}

impl VibeVJApp {
    /// Create a new VibeVJ application
    pub fn new() -> Result<Self> {
        // Create WGPU instance once for the entire application
        let wgpu_instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        
        Ok(Self {
            window: None,
            renderer: None,
            gui: None,
            egui_state: None,
            wgpu_instance,
            
            scene_renderer: None,
            scene_state: SceneState::new(),
            render_target: None,
            
            preview_window: None,
            show_preview_window: false,
            
            scene: Scene::new("Main Scene".to_string()),
            audio_input: AudioInput::default(),
            audio_analyzer: AudioAnalyzer::default(),
            script_engine: ScriptEngine::new(),
            selected_audio_device: None,
            
            start_time: Instant::now(),
            last_frame_time: Instant::now(),
            frame_count: 0,
            
            frequency_bands: FrequencyBands::default(),
        })
    }
    
    /// Get list of available audio devices
    pub fn list_audio_devices(&self) -> Vec<vibevj_audio::AudioDeviceInfo> {
        AudioInput::list_devices().unwrap_or_default()
    }
    
    /// Get currently selected audio device
    pub fn selected_audio_device(&self) -> Option<&str> {
        self.selected_audio_device.as_deref()
    }
    
    /// Select and start audio device
    pub fn select_audio_device(&mut self, device_name: Option<String>) -> Result<()> {
        self.audio_input.stop();
        self.selected_audio_device = device_name.clone();
        
        if let Err(e) = self.audio_input.start_with_device(device_name.as_deref()) {
            log::warn!("Failed to start audio input: {}", e);
        }
        
        Ok(())
    }

    /// Initialize the application after window creation
    async fn initialize(&mut self, window: Arc<Window>) -> Result<()> {
        // Create renderer
        let renderer = Renderer::new(window.clone()).await?;
        let surface_format = renderer.surface_format();

        // Create GUI
        let mut gui = GuiApp::new(&renderer.device, &renderer.queue, surface_format);

        // Create egui state
        let mut egui_state = egui_winit::State::new(
            gui.context().clone(),
            egui::ViewportId::ROOT,
            &window,
            Some(window.scale_factor() as f32),
            None,
        );

        // Run a first frame to initialize font textures
        let raw_input = egui_state.take_egui_input(&window);
        let full_output = gui.context().run(raw_input, |_ctx| {
            // Empty first frame just to initialize textures
        });
        
        // Upload initial textures (especially fonts)
        for (id, image_delta) in &full_output.textures_delta.set {
            gui.renderer_mut().update_texture(
                &renderer.device,
                &renderer.queue,
                *id,
                image_delta,
            );
        }

        // Create 3D scene renderer
        let camera = Camera::new(
            Vec3::new(3.0, 2.0, 5.0),
            Vec3::ZERO,
            renderer.aspect_ratio(),
        );
        let scene_renderer = SceneRenderer::new(&renderer.device, surface_format, camera);
        
        // Create render target for the 3D scene
        let render_target = RenderTarget::new(
            &renderer.device,
            1280,  // Default render size
            720,
            surface_format,
            Some("Scene Render Target"),
        );
        
        // Create some test objects
        let mut cube = RenderObject::new(
            mesh_gen::create_cube(1.0),
            Material::unlit(vibevj_common::Color::new(1.0, 0.5, 0.2, 1.0)),
            Mat4::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        );
        cube.upload(
            &renderer.device,
            scene_renderer.material_bind_group_layout(),
            scene_renderer.model_bind_group_layout(),
        );
        
        let mut sphere = RenderObject::new(
            mesh_gen::create_sphere(0.8, 32, 16),
            Material::unlit(vibevj_common::Color::new(0.2, 0.5, 1.0, 1.0)),
            Mat4::from_translation(Vec3::new(-2.5, 0.0, 0.0)),
        );
        sphere.upload(
            &renderer.device,
            scene_renderer.material_bind_group_layout(),
            scene_renderer.model_bind_group_layout(),
        );
        
        // Initialize scene state with render objects
        self.scene_state.render_objects = vec![cube, sphere];
        self.scene_renderer = Some(scene_renderer);
        
        // Register render target texture with egui
        let texture_id = gui.register_render_texture(
            &renderer.device,
            &renderer.queue,
            &render_target.texture,
            [render_target.width, render_target.height],
        );
        log::info!("Registered render texture with ID: {:?}", texture_id);
        
        self.render_target = Some(render_target);

        self.renderer = Some(renderer);
        self.gui = Some(gui);
        self.egui_state = Some(egui_state);
        self.window = Some(window);

        // Start audio input
        if let Err(e) = self.audio_input.start() {
            log::warn!("Failed to start audio input: {}", e);
        }

        log::info!("VibeVJ initialized successfully");
        Ok(())
    }

    /// Update application state
    fn update(&mut self) {
        let now = Instant::now();
        let delta = (now - self.last_frame_time).as_secs_f32();
        let elapsed = (now - self.start_time).as_secs_f64();
        
        let time_info = TimeInfo {
            elapsed,
            delta,
            frame: self.frame_count,
        };

        // Update audio analysis
        let samples = self.audio_input.get_samples();
        if !samples.is_empty() {
            if let Ok(bands) = self.audio_analyzer.analyze_bands(&samples, self.audio_input.sample_rate()) {
                self.frequency_bands = bands;
            }
        }

        // Update GUI
        let mut audio_device_to_select: Option<String> = None;
        
        // Check if we need to populate audio devices
        let needs_audio_devices = self.gui.as_ref().map(|g| !g.has_audio_devices()).unwrap_or(false);
        let audio_device_list = if needs_audio_devices {
            let devices = self.list_audio_devices();
            let device_names: Vec<String> = devices.iter().map(|d| {
                if d.is_default {
                    format!("{} (Default)", d.name)
                } else {
                    d.name.clone()
                }
            }).collect();
            Some((device_names, self.audio_input.current_device_name().map(|s| s.to_string())))
        } else {
            None
        };
        
        if let Some(gui) = &mut self.gui {
            // Update audio device list in GUI if not populated
            if let Some((device_names, selected)) = audio_device_list {
                gui.set_audio_devices(device_names, selected.as_deref());
            }
            
            // Check for audio device changes
            if let Some(device_name) = gui.take_audio_device_change() {
                // Extract actual device name (remove " (Default)" suffix if present)
                audio_device_to_select = Some(device_name.replace(" (Default)", ""));
            }
            
            gui.update(&time_info);
            
            // Check if preview window toggle state has changed
            let should_show = gui.should_show_preview_window();
            if should_show != self.show_preview_window {
                self.show_preview_window = should_show;
                if !should_show {
                    // Close preview window
                    self.preview_window = None;
                    log::info!("Preview window closed");
                }
                // Note: Preview window creation is handled in the event loop
                // because we need access to the EventLoopWindowTarget
            }
        }
        
        // Handle audio device selection outside of GUI borrow
        if let Some(device_name) = audio_device_to_select {
            log::info!("Switching audio device to: {}", device_name);
            if let Err(e) = self.select_audio_device(Some(device_name)) {
                log::error!("Failed to switch audio device: {}", e);
            }
        }
        
        // Preview window texture will be updated in render() method
        
        // Update scene state
        self.scene_state.update(elapsed as f32);
        
        // Update 3D objects - rotate them
        if let Some(renderer) = &self.renderer {
            let rotation_speed = 1.0;
            let angle = elapsed as f32 * rotation_speed;
            
            // Rotate cube around Y axis
            if !self.scene_state.render_objects.is_empty() {
                let transform = Mat4::from_translation(Vec3::new(0.0, 0.0, 0.0))
                    * Mat4::from_rotation_y(angle)
                    * Mat4::from_rotation_x(angle * 0.5);
                self.scene_state.render_objects[0].update_transform(&renderer.queue, transform);
            }
            
            // Rotate sphere around its own axis
            if self.scene_state.render_objects.len() > 1 {
                let transform = Mat4::from_translation(Vec3::new(-2.5, 0.0, 0.0))
                    * Mat4::from_rotation_z(angle * 1.5);
                self.scene_state.render_objects[1].update_transform(&renderer.queue, transform);
            }
        }

        self.last_frame_time = now;
        self.frame_count += 1;
    }

    /// Render a frame
    fn render(&mut self) -> Result<()> {
        let renderer = self.renderer.as_mut().unwrap();
        let gui = self.gui.as_mut().unwrap();
        let egui_state = self.egui_state.as_mut().unwrap();
        let window = self.window.as_ref().unwrap();

        // Get surface texture, handling surface changes
        let output = match renderer.get_current_texture() {
            Ok(texture) => texture,
            Err(e) => {
                // Surface has changed, need to reconfigure
                if e.to_string().contains("surface has changed") || 
                   e.to_string().contains("swap chain must be updated") ||
                   e.to_string().contains("Timeout") {
                    log::info!("Surface changed, reconfiguring...");
                    let size = window.inner_size();
                    renderer.resize(size);
                    // Try again after reconfiguration
                    renderer.get_current_texture()?
                } else {
                    return Err(e.into());
                }
            }
        };
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Begin egui frame
        let raw_input = egui_state.take_egui_input(window);
        let ctx = gui.context().clone();
        let full_output = ctx.run(raw_input, |ctx| {
            gui.render(ctx);
        });

        // Handle egui output
        egui_state.handle_platform_output(window, full_output.platform_output);

        // Update egui textures BEFORE tessellation
        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [renderer.size.width, renderer.size.height],
            pixels_per_point: window.scale_factor() as f32,
        };
        
        for (id, image_delta) in &full_output.textures_delta.set {
            gui.renderer_mut().update_texture(
                &renderer.device,
                &renderer.queue,
                *id,
                image_delta,
            );
        }

        // Tessellate the shapes into triangles AFTER texture updates
        let clipped_primitives = ctx.tessellate(full_output.shapes, full_output.pixels_per_point);

        // Create command encoder
        let mut encoder = renderer.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        
        // Update buffers before render pass
        gui.renderer_mut().update_buffers(
            &renderer.device,
            &renderer.queue,
            &mut encoder,
            &clipped_primitives,
            &screen_descriptor,
        );

        // Render 3D scene to main render target
        if let (Some(scene_renderer), Some(render_target)) = (&mut self.scene_renderer, &self.render_target) {
            // Update camera
            scene_renderer.update_camera(&renderer.queue);
            
            // Render 3D objects to render target
            let object_refs: Vec<&RenderObject> = self.scene_state.render_objects.iter().collect();
            scene_renderer.render(
                &mut encoder,
                &render_target.view,
                &render_target.depth_view,
                &object_refs,
                wgpu::Color {
                    r: 0.1,
                    g: 0.1,
                    b: 0.1,
                    a: 1.0,
                },
            );
        }
        
        // Update preview window's scene transforms to match main scene
        // The actual rendering will happen in RedrawRequested event
        if let Some(preview_window) = &mut self.preview_window {
            let transforms: Vec<_> = self.scene_state.render_objects.iter().map(|obj| obj.transform).collect();
            preview_window.update_scene(transforms);
        }

        // Render GUI to window
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("GUI Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.1,
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

        // Render egui
        gui.renderer_mut().render(
            &mut render_pass,
            &clipped_primitives,
            &screen_descriptor,
        );
        
        // Explicitly drop render_pass to release the borrow on encoder
        drop(render_pass);

        // Free egui textures
        for id in &full_output.textures_delta.free {
            gui.renderer_mut().free_texture(id);
        }

        // Submit commands
        renderer.queue.submit(Some(encoder.finish()));
        output.present();

        Ok(())
    }

    /// Run the application event loop
    pub fn run(mut self, event_loop: EventLoop<()>) -> Result<()> {
        event_loop.run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::Poll);

            match event {
                Event::Resumed => {
                    if self.window.is_none() {
                        let window = Arc::new(
                            winit::window::WindowBuilder::new()
                                .with_title("VibeVJ - Visual Jockey")
                                .with_inner_size(winit::dpi::LogicalSize::new(1600, 900))
                                .build(elwt)
                                .expect("Failed to create window")
                        );
                        
                        pollster::block_on(async {
                            if let Err(e) = self.initialize(window).await {
                                log::error!("Failed to initialize application: {}", e);
                                elwt.exit();
                            }
                        });
                    }
                }
                Event::WindowEvent { event, window_id } => {
                    // Check if this is the preview window
                    let is_preview_window = self.preview_window.as_ref()
                        .map(|pw| pw.window.id() == window_id)
                        .unwrap_or(false);
                    
                    if is_preview_window {
                        // Handle preview window events
                        // First, handle close request which will set preview_window to None
                        if matches!(event, WindowEvent::CloseRequested) {
                            self.preview_window = None;
                            self.show_preview_window = false;
                            if let Some(gui) = &mut self.gui {
                                gui.set_show_preview_window(false);
                            }
                            log::info!("Preview window closed by user");
                            return;
                        }
                        
                        // For all other events, only process if preview window still exists
                        if let Some(preview_window) = &mut self.preview_window {
                            // Handle input (fullscreen toggle)
                            preview_window.handle_input(&event);
                            
                            match event {
                                WindowEvent::Resized(physical_size) => {
                                    preview_window.resize(physical_size);
                                }
                                WindowEvent::RedrawRequested => {
                                    if let Err(e) = preview_window.render() {
                                        log::error!("Preview window render error: {}", e);
                                    }
                                }
                                _ => {}
                            }
                        }
                        return;
                    }
                    
                    // Handle main window events
                    // Handle egui events
                    if let Some(egui_state) = &mut self.egui_state {
                        if let Some(window) = &self.window {
                            if window.id() == window_id {
                                let response = egui_state.on_window_event(window, &event);
                                if response.consumed {
                                    return;
                                }
                            }
                        }
                    }

                    // Handle window events
                    match event {
                        WindowEvent::CloseRequested => {
                            log::info!("Close requested, exiting...");
                            elwt.exit();
                        }
                        WindowEvent::Resized(physical_size) => {
                            if let Some(renderer) = &mut self.renderer {
                                renderer.resize(physical_size);
                            }
                        }
                        WindowEvent::RedrawRequested => {
                            self.update();
                            
                            if let Err(e) = self.render() {
                                log::error!("Render error: {}", e);
                            }

                            if let Some(window) = &self.window {
                                window.request_redraw();
                            }
                        }
                        _ => {}
                    }
                }
                Event::AboutToWait => {
                    // Create preview window if needed
                    if self.show_preview_window && self.preview_window.is_none() && self.renderer.is_some() {
                        let renderer = self.renderer.as_ref().unwrap();
                        let instance = &self.wgpu_instance;
                        pollster::block_on(async {
                            match PreviewWindow::new(elwt, &renderer.device, instance).await {
                                Ok(mut pw) => {
                                    // Initialize preview window with scene objects
                                    let mesh_material_data: Vec<_> = self.scene_state.render_objects.iter().map(|obj| {
                                        (obj.mesh.clone(), obj.material.clone(), obj.transform)
                                    }).collect();
                                    pw.init_scene_objects(mesh_material_data);
                                    
                                    log::info!("Preview window created successfully");
                                    self.preview_window = Some(pw);
                                }
                                Err(e) => {
                                    log::error!("Failed to create preview window: {}", e);
                                    self.show_preview_window = false;
                                    if let Some(gui) = &mut self.gui {
                                        gui.set_show_preview_window(false);
                                    }
                                }
                            }
                        });
                    }
                    
                    // Request redraws
                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                    if let Some(preview_window) = &self.preview_window {
                        preview_window.window.request_redraw();
                    }
                }
                _ => {}
            }
        }).map_err(|e| anyhow::anyhow!("Event loop error: {}", e))
    }
}
