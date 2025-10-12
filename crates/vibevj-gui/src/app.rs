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
        }
    }

    /// Get the egui context
    pub fn context(&self) -> &Context {
        &self.context
    }

    /// Update the GUI
    pub fn update(&mut self, time: &TimeInfo) {
        // Update internal state
        self.left_panel.update(time);
        self.center_panel.update(time);
        self.right_panel.update(time);
    }

    /// Render the GUI layout
    pub fn render(&mut self, ctx: &Context) {
        // Calculate panel widths based on available space
        let available_width = ctx.screen_rect().width();
        let left_width = available_width * 0.25;
        let center_width = available_width * 0.60;
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
