use egui::{Context, ViewportId};
use egui_wgpu::Renderer as EguiRenderer;
use vibevj_common::TimeInfo;
use crate::panels::{LeftPanel, CenterPanel, RightPanel};

/// Main GUI application
pub struct GuiApp {
    context: Context,
    renderer: EguiRenderer,
    left_panel: LeftPanel,
    center_panel: CenterPanel,
    right_panel: RightPanel,
    render_texture_id: Option<egui::TextureId>,
    show_preview_window: bool,
}

impl GuiApp {
    /// Create a new GUI application
    pub fn new(
        device: &wgpu::Device,
        _queue: &wgpu::Queue,
        surface_format: wgpu::TextureFormat,
    ) -> Self {
        let context = Context::default();
        
        let renderer = EguiRenderer::new(
            device,
            surface_format,
            None,
            1,
        );

        Self {
            context,
            renderer,
            left_panel: LeftPanel::new(),
            center_panel: CenterPanel::new(),
            right_panel: RightPanel::new(),
            render_texture_id: None,
            show_preview_window: false,
        }
    }
    
    /// Register a render target texture to display in preview
    pub fn register_render_texture(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture: &wgpu::Texture,
        size: [u32; 2],
    ) -> egui::TextureId {
        let texture_id = self.renderer.register_native_texture(
            device,
            &texture.create_view(&wgpu::TextureViewDescriptor::default()),
            wgpu::FilterMode::Linear,
        );
        self.render_texture_id = Some(texture_id);
        texture_id
    }
    
    /// Get the render texture ID
    pub fn render_texture_id(&self) -> Option<egui::TextureId> {
        self.render_texture_id
    }

    /// Get the egui context
    pub fn context(&self) -> &Context {
        &self.context
    }
    
    /// Check if the preview window should be shown
    pub fn should_show_preview_window(&self) -> bool {
        self.show_preview_window
    }
    
    /// Set whether the preview window should be shown
    pub fn set_show_preview_window(&mut self, show: bool) {
        self.show_preview_window = show;
    }

    /// Update the GUI
    pub fn update(&mut self, time: &TimeInfo) {
        // Update internal state
        self.left_panel.update(time);
        self.center_panel.update(time);
        self.right_panel.update(time);
        
        // Update render texture in center panel
        if let Some(texture_id) = self.render_texture_id {
            self.center_panel.set_render_texture(Some(texture_id));
        }
    }

    /// Render the GUI layout
    pub fn render(&mut self, ctx: &Context) {
        // Top menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // File menu
                ui.menu_button("File", |ui| {
                    if ui.button("New Project").clicked() {
                        // TODO: Implement new project
                        ui.close_menu();
                    }
                    if ui.button("Open Project...").clicked() {
                        // TODO: Implement open project
                        ui.close_menu();
                    }
                    if ui.button("Save Project").clicked() {
                        // TODO: Implement save project
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Exit").clicked() {
                        // TODO: Implement exit
                        ui.close_menu();
                    }
                });
                
                // Edit menu
                ui.menu_button("Edit", |ui| {
                    if ui.button("Undo").clicked() {
                        // TODO: Implement undo
                        ui.close_menu();
                    }
                    if ui.button("Redo").clicked() {
                        // TODO: Implement redo
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Preferences...").clicked() {
                        // TODO: Implement preferences
                        ui.close_menu();
                    }
                });
                
                // Window menu
                ui.menu_button("Window", |ui| {
                    if ui.checkbox(&mut self.show_preview_window, "Show Preview Window").changed() {
                        // State has changed, will be checked by main app
                    }
                });
                
                // Help menu
                ui.menu_button("Help", |ui| {
                    if ui.button("Documentation").clicked() {
                        // TODO: Open documentation
                        ui.close_menu();
                    }
                    if ui.button("About").clicked() {
                        // TODO: Show about dialog
                        ui.close_menu();
                    }
                });
            });
        });
        
        // Calculate panel widths based on available space
        let available_width = ctx.screen_rect().width();
        let left_width = available_width * 0.25;
        let right_width = available_width * 0.15;

        // Left Panel (25%)
        egui::SidePanel::left("left_panel")
            .exact_width(left_width)
            .resizable(true)
            .show(ctx, |ui| {
                self.left_panel.ui(ui);
            });

        // Right Panel (15%)
        egui::SidePanel::right("right_panel")
            .exact_width(right_width)
            .resizable(true)
            .show(ctx, |ui| {
                self.right_panel.ui(ui);
            });

        // Center Panel (60% - fills remaining space)
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                self.center_panel.ui(ui);
            });
    }

    /// Get the egui renderer
    pub fn renderer_mut(&mut self) -> &mut EguiRenderer {
        &mut self.renderer
    }
}
