use egui::Ui;
use vibevj_common::TimeInfo;
use vibevj_engine::texture;
use crate::scene_editor::SceneEditor;

/// Content types for the center panel
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PanelContent {
    Preview,
    SceneEditor,
    TimelineEditor,
}

/// Left panel - Main render preview and controls
pub struct LeftPanel {
    fps: f32,
    show_stats: bool,
    render_texture: Option<egui::TextureId>,
}

impl LeftPanel {
    pub fn new() -> Self {
        Self {
            fps: 0.0,
            show_stats: true,
            render_texture: None,
        }
    }

    pub fn update(&mut self, time: &TimeInfo) {
        if time.delta > 0.0 {
            self.fps = 1.0 / time.delta;
        }
    }
    
    /// Set the render texture to display in preview
    pub fn set_render_texture(&mut self, texture_id: Option<egui::TextureId>) {
        self.render_texture = texture_id;
    }

    pub fn render_preview(&self, ui: &mut Ui, texture_id: egui::TextureId) {
        ui.heading("Render Preview");
        ui.separator();
        // Preview area
        egui::ScrollArea::both().show(ui, |ui| {
            if let Some(texture_id) = self.render_texture {
                // Display the 3D render texture
                let available_size = ui.available_size();
                let aspect_ratio = 1280.0 / 720.0; // Match render target aspect
                let height = available_size.x / aspect_ratio;
                let size = egui::vec2(available_size.x, height.min(available_size.y));
                
                ui.image(egui::load::SizedTexture::new(texture_id, size));
            } else {
                // Fallback if no texture
                ui.group(|ui| {
                    ui.set_min_height(300.0);
                    ui.centered_and_justified(|ui| {
                        ui.label("🎬 Waiting for render...");
                    });
                });
            }
        });
    }

    pub fn ui(&mut self, ui: &mut Ui) {

        if let Some(texture_id) = self.render_texture {
            self.render_preview(ui, texture_id);
        }
        

        ui.separator();

        // Controls
        ui.heading("Controls");
        
        ui.horizontal(|ui| {
            if ui.button("▶ Play").clicked() {
                log::info!("Play clicked");
            }
            if ui.button("⏸ Pause").clicked() {
                log::info!("Pause clicked");
            }
            if ui.button("⏹ Stop").clicked() {
                log::info!("Stop clicked");
            }
        });

        ui.separator();

        // Stats
        ui.checkbox(&mut self.show_stats, "Show Stats");
        
        if self.show_stats {
            ui.group(|ui| {
                ui.label(format!("FPS: {:.1}", self.fps));
                ui.label(format!("Frame: {}", 0)); // TODO: Get from time
            });
        }

        ui.separator();

        // Scene settings
        ui.collapsing("Scene Settings", |ui| {
            ui.label("Background Color");
            // Color picker would go here
            ui.label("Resolution");
            ui.label("Quality");
        });
    }
}

impl Default for LeftPanel {
    fn default() -> Self {
        Self::new()
    }
}

/// Center panel - Main view (preview, scene editor, or timeline)
pub struct CenterPanel {
    current_content: PanelContent,
    scene_editor: SceneEditor,
    render_texture: Option<egui::TextureId>,
}

impl CenterPanel {
    pub fn new() -> Self {
        Self {
            current_content: PanelContent::Preview,
            scene_editor: SceneEditor::new(),
            render_texture: None,
        }
    }

    pub fn update(&mut self, _time: &TimeInfo) {
        // Update based on current content
    }
    
    /// Set the render texture to display in preview
    pub fn set_render_texture(&mut self, texture_id: Option<egui::TextureId>) {
        self.render_texture = texture_id;
    }
    
    /// Get the current content type
    pub fn current_content(&self) -> PanelContent {
        self.current_content
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        // Content selector tabs
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.current_content, PanelContent::Preview, "📺 Preview");
            ui.selectable_value(&mut self.current_content, PanelContent::SceneEditor, "🎨 Scene Editor");
            ui.selectable_value(&mut self.current_content, PanelContent::TimelineEditor, "⏱ Timeline");
        });

        ui.separator();

        // Render current content
        match self.current_content {
            PanelContent::Preview => self.render_preview(ui),
            PanelContent::SceneEditor => self.render_scene_editor(ui),
            PanelContent::TimelineEditor => self.render_timeline(ui),
        }
    }

    fn render_preview(&self, ui: &mut Ui) {
        ui.heading("Main Preview");
        
        egui::ScrollArea::both().show(ui, |ui| {
            if let Some(texture_id) = self.render_texture {
                // Display the 3D render texture
                let available_size = ui.available_size();
                let aspect_ratio = 1280.0 / 720.0; // Match render target aspect
                let height = available_size.x / aspect_ratio;
                let size = egui::vec2(available_size.x, height.min(available_size.y));
                
                ui.image(egui::load::SizedTexture::new(texture_id, size));
            } else {
                // Fallback if no texture
                ui.group(|ui| {
                    ui.set_min_size(egui::vec2(ui.available_width(), 400.0));
                    ui.centered_and_justified(|ui| {
                        ui.label("🖼 Waiting for render...");
                    });
                });
            }
        });
    }

    fn render_scene_editor(&mut self, ui: &mut Ui) {
        self.scene_editor.ui(ui);
    }

    fn render_timeline(&self, ui: &mut Ui) {
        ui.heading("Timeline Editor");
        
        ui.group(|ui| {
            ui.set_min_height(100.0);
            ui.label("⏱ Timeline Track 1");
        });
        
        ui.group(|ui| {
            ui.set_min_height(100.0);
            ui.label("⏱ Timeline Track 2");
        });
        
        ui.group(|ui| {
            ui.set_min_height(100.0);
            ui.label("⏱ Timeline Track 3");
        });
    }
}

impl Default for CenterPanel {
    fn default() -> Self {
        Self::new()
    }
}

/// Right panel - Prefabs and resources browser
pub struct RightPanel {
    search_query: String,
}

impl RightPanel {
    pub fn new() -> Self {
        Self {
            search_query: String::new(),
        }
    }

    pub fn update(&mut self, _time: &TimeInfo) {
        // Update resource lists
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.heading("Resources");
        ui.separator();

        // Search bar
        ui.horizontal(|ui| {
            ui.label("🔍");
            ui.text_edit_singleline(&mut self.search_query);
        });

        ui.separator();

        // Resource categories
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.collapsing("📦 Prefabs", |ui| {
                ui.label("  • Cube");
                ui.label("  • Sphere");
                ui.label("  • Plane");
                ui.label("  • Custom Mesh");
            });

            ui.collapsing("🎨 Shaders", |ui| {
                ui.label("  • Basic Shader");
                ui.label("  • Phong Shader");
                ui.label("  • PBR Shader");
                ui.label("  • Custom Shader");
            });

            ui.collapsing("🖼 Textures", |ui| {
                ui.label("  • Texture 1");
                ui.label("  • Texture 2");
                ui.label("  • Normal Map");
            });

            ui.collapsing("🎵 Audio", |ui| {
                ui.label("  • Audio Input");
                ui.label("  • Audio File");
                ui.label("  • Frequency Bands");
            });

            ui.collapsing("📹 Videos", |ui| {
                ui.label("  • Video 1");
                ui.label("  • Video 2");
            });

            ui.collapsing("🖼 Images", |ui| {
                ui.label("  • Image 1");
                ui.label("  • GIF 1");
            });
        });

        ui.separator();

        // Quick actions
        if ui.button("➕ Import Resource").clicked() {
            log::info!("Import resource clicked");
        }
    }
}

impl Default for RightPanel {
    fn default() -> Self {
        Self::new()
    }
}
