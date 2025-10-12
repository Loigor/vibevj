use egui::Ui;

/// Scene editor component for node-based visual programming
pub struct SceneEditor {
    // Future fields for node graph state
}

impl SceneEditor {
    pub fn new() -> Self {
        Self {}
    }

    /// Render the scene editor UI
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.heading("Scene Editor");
        
        egui::ScrollArea::both().show(ui, |ui| {
            ui.group(|ui| {
                ui.set_min_size(egui::vec2(ui.available_width(), 400.0));
                ui.label("🔷 Node-based Visual Programming");
                ui.label("(Node graph editor will be implemented here)");
                
                // Placeholder for node editor
                ui.separator();
                ui.label("Nodes:");
                ui.label("  • Shader Node");
                ui.label("  • Audio Analyzer Node");
                ui.label("  • Transform Node");
                ui.label("  • Output Node");
            });
        });
    }
}

impl Default for SceneEditor {
    fn default() -> Self {
        Self::new()
    }
}
