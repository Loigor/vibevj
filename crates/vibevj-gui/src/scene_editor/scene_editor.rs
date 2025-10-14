use egui::{Color32, Pos2, Rect, Sense, Ui, Vec2};
use super::nodes::{Node, NodeGraph};

/// Scene editor component for node-based visual programming
pub struct SceneEditor {
    node_graph: NodeGraph,
    canvas_offset: Vec2,
    canvas_scale: f32,
    is_panning: bool,
    last_cursor_pos: Option<Pos2>,
}

impl SceneEditor {
    pub fn new() -> Self {
        let mut editor = Self {
            node_graph: NodeGraph::new(),
            canvas_offset: Vec2::ZERO,
            canvas_scale: 1.0,
            is_panning: false,
            last_cursor_pos: None,
        };
        
        // Add some example nodes
        editor.create_example_nodes();
        
        editor
    }

    /// Create some example nodes for demonstration
    fn create_example_nodes(&mut self) {
        // Create a shader node
        let shader_id = self.node_graph.new_node_id();
        let mut shader_node = Node::new(shader_id, "Shader", Pos2::new(100.0, 100.0));
        shader_node.add_input(self.node_graph.new_socket_id(), "UV");
        shader_node.add_input(self.node_graph.new_socket_id(), "Time");
        shader_node.add_output(self.node_graph.new_socket_id(), "Color");
        shader_node.color = Color32::from_rgb(80, 60, 100);
        self.node_graph.add_node(shader_node);

        // Create an audio node
        let audio_id = self.node_graph.new_node_id();
        let mut audio_node = Node::new(audio_id, "Audio Analyzer", Pos2::new(100.0, 250.0));
        audio_node.add_output(self.node_graph.new_socket_id(), "Bass");
        audio_node.add_output(self.node_graph.new_socket_id(), "Mid");
        audio_node.add_output(self.node_graph.new_socket_id(), "Treble");
        audio_node.color = Color32::from_rgb(60, 100, 80);
        self.node_graph.add_node(audio_node);

        // Create an output node
        let output_id = self.node_graph.new_node_id();
        let mut output_node = Node::new(output_id, "Scene Output", Pos2::new(400.0, 150.0));
        output_node.add_input(self.node_graph.new_socket_id(), "Color");
        output_node.add_input(self.node_graph.new_socket_id(), "Transform");
        output_node.color = Color32::from_rgb(100, 60, 60);
        self.node_graph.add_node(output_node);
    }

    /// Render the scene editor UI
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.heading("Scene Editor");
        
        // Toolbar
        ui.horizontal(|ui| {
            if ui.button("➕ Add Node").clicked() {
                // TODO: Show node menu
            }
            
            if ui.button("🗑 Delete Selected").clicked() {
                self.node_graph.delete_selected();
            }
            
            ui.separator();
            
            if ui.button("🔍 Reset View").clicked() {
                self.canvas_offset = Vec2::ZERO;
                self.canvas_scale = 1.0;
            }
            
            ui.label(format!("Zoom: {:.0}%", self.canvas_scale * 100.0));
        });
        
        ui.separator();
        
        // Node canvas
        let available_size = ui.available_size();
        let canvas_rect = Rect::from_min_size(ui.cursor().min, available_size);
        
        // Create canvas area with custom painting
        let response = ui.allocate_rect(canvas_rect, Sense::click_and_drag());
        
        // Handle panning with middle mouse or space + drag
        let is_panning_key = ui.input(|i| i.key_down(egui::Key::Space));
        if response.dragged_by(egui::PointerButton::Middle) || (response.dragged() && is_panning_key) {
            self.is_panning = true;
            self.canvas_offset += response.drag_delta();
        } else if response.drag_stopped() {
            self.is_panning = false;
        }
        
        // Handle zoom with mouse wheel
        let scroll_delta = ui.input(|i| i.smooth_scroll_delta.y);
        if scroll_delta.abs() > 0.0 {
            let zoom_factor = 1.0 + scroll_delta * 0.001;
            self.canvas_scale = (self.canvas_scale * zoom_factor).clamp(0.25, 2.0);
        }
        
        // Draw grid background
        self.draw_grid(ui, canvas_rect);
        
        // Set up clipping and transformation
        ui.set_clip_rect(canvas_rect);
        
        // Transform canvas coordinates
        let cursor_pos = ui.input(|i| i.pointer.hover_pos());
        let transformed_cursor = cursor_pos.map(|pos| {
            self.screen_to_canvas(pos, canvas_rect)
        });
        
        // Draw connections first (behind nodes)
        self.node_graph.draw_connections(ui, self.canvas_offset, self.canvas_scale, canvas_rect);
        
        // Handle node interaction and drawing
        self.node_graph.handle_interaction(ui, transformed_cursor, self.canvas_offset, self.canvas_scale, canvas_rect);
        
        // Show instructions
        ui.allocate_ui_at_rect(
            Rect::from_min_size(
                canvas_rect.min + Vec2::new(10.0, 10.0),
                Vec2::new(300.0, 100.0),
            ),
            |ui| {
                egui::Frame::none()
                    .fill(Color32::from_black_alpha(180))
                    .inner_margin(8.0)
                    .rounding(4.0)
                    .show(ui, |ui| {
                        ui.label("🖱 Click and drag to move nodes");
                        ui.label("🖱 Middle-click or Space + drag to pan");
                        ui.label("🖱 Scroll to zoom");
                        ui.label("🔗 Drag from output to input to connect");
                        ui.label(format!("📍 Offset: ({:.0}, {:.0})", self.canvas_offset.x, self.canvas_offset.y));
                    });
            },
        );
    }

    /// Draw grid background
    fn draw_grid(&self, ui: &mut Ui, rect: Rect) {
        let painter = ui.painter();
        
        // Fill background
        painter.rect_filled(rect, 0.0, Color32::from_rgb(30, 30, 35));
        
        // Draw grid lines
        let grid_spacing = 50.0 * self.canvas_scale;
        let grid_color = Color32::from_gray(40);
        
        // Vertical lines
        let start_x = (rect.min.x - self.canvas_offset.x) % grid_spacing;
        let mut x = rect.min.x + start_x;
        while x < rect.max.x {
            painter.line_segment(
                [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
                (1.0, grid_color),
            );
            x += grid_spacing;
        }
        
        // Horizontal lines
        let start_y = (rect.min.y - self.canvas_offset.y) % grid_spacing;
        let mut y = rect.min.y + start_y;
        while y < rect.max.y {
            painter.line_segment(
                [Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)],
                (1.0, grid_color),
            );
            y += grid_spacing;
        }
        
        // Draw origin lines (slightly brighter)
        let origin_x = rect.min.x + self.canvas_offset.x;
        let origin_y = rect.min.y + self.canvas_offset.y;
        
        if origin_x >= rect.min.x && origin_x <= rect.max.x {
            painter.line_segment(
                [Pos2::new(origin_x, rect.min.y), Pos2::new(origin_x, rect.max.y)],
                (2.0, Color32::from_gray(60)),
            );
        }
        
        if origin_y >= rect.min.y && origin_y <= rect.max.y {
            painter.line_segment(
                [Pos2::new(rect.min.x, origin_y), Pos2::new(rect.max.x, origin_y)],
                (2.0, Color32::from_gray(60)),
            );
        }
    }

    /// Convert screen coordinates to canvas coordinates
    fn screen_to_canvas(&self, screen_pos: Pos2, canvas_rect: Rect) -> Pos2 {
        Pos2::new(
            (screen_pos.x - canvas_rect.min.x - self.canvas_offset.x) / self.canvas_scale,
            (screen_pos.y - canvas_rect.min.y - self.canvas_offset.y) / self.canvas_scale,
        )
    }

    /// Convert canvas coordinates to screen coordinates
    pub fn canvas_to_screen(&self, canvas_pos: Pos2, canvas_rect: Rect) -> Pos2 {
        Pos2::new(
            canvas_pos.x * self.canvas_scale + self.canvas_offset.x + canvas_rect.min.x,
            canvas_pos.y * self.canvas_scale + self.canvas_offset.y + canvas_rect.min.y,
        )
    }
}

impl Default for SceneEditor {
    fn default() -> Self {
        Self::new()
    }
}
