use anyhow::Result;
use std::sync::Arc;
use std::time::Instant;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use vibevj_common::TimeInfo;
use vibevj_engine::Renderer;
use vibevj_gui::GuiApp;
use vibevj_audio::{AudioInput, AudioAnalyzer, FrequencyBands};
use vibevj_scene::Scene;
use vibevj_scripting::ScriptEngine;

/// Main VibeVJ application
pub struct VibeVJApp {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    gui: Option<GuiApp>,
    egui_state: Option<egui_winit::State>,
    
    // Application state
    scene: Scene,
    audio_input: AudioInput,
    audio_analyzer: AudioAnalyzer,
    script_engine: ScriptEngine,
    
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
        Ok(Self {
            window: None,
            renderer: None,
            gui: None,
            egui_state: None,
            
            scene: Scene::new("Main Scene".to_string()),
            audio_input: AudioInput::default(),
            audio_analyzer: AudioAnalyzer::default(),
            script_engine: ScriptEngine::new(),
            
            start_time: Instant::now(),
            last_frame_time: Instant::now(),
            frame_count: 0,
            
            frequency_bands: FrequencyBands::default(),
        })
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
        if let Some(gui) = &mut self.gui {
            gui.update(&time_info);
        }

        self.last_frame_time = now;
        self.frame_count += 1;
    }

    /// Render a frame
    fn render(&mut self) -> Result<()> {
        let renderer = self.renderer.as_ref().unwrap();
        let gui = self.gui.as_mut().unwrap();
        let egui_state = self.egui_state.as_mut().unwrap();
        let window = self.window.as_ref().unwrap();

        // Get surface texture
        let output = renderer.get_current_texture()?;
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

        // Render scene and egui
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main Render Pass"),
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
        }

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
                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                }
                _ => {}
            }
        }).map_err(|e| anyhow::anyhow!("Event loop error: {}", e))
    }
}
