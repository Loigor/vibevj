//! Node graph system for visual programming in the scene editor
//! 
//! This module provides the core node system including:
//! - Node rendering with inputs/outputs
//! - Node connections (edges)
//! - Dragging and interaction
//! - Visual styling

use egui::{Color32, Pos2, Rect, Response, Sense, Shape, Stroke, Ui, Vec2};
use std::collections::HashMap;

/// Unique identifier for a node
pub type NodeId = u64;

/// Unique identifier for a node socket (input or output)
pub type SocketId = u64;

/// Represents a connection between two node sockets
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Connection {
    pub from_node: NodeId,
    pub from_socket: SocketId,
    pub to_node: NodeId,
    pub to_socket: SocketId,
}

/// Type of socket (input or output)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketType {
    Input,
    Output,
}

/// Socket on a node (input or output connection point)
#[derive(Debug, Clone)]
pub struct Socket {
    pub id: SocketId,
    pub socket_type: SocketType,
    pub name: String,
    pub position: Pos2, // World space position
}

/// Visual node in the graph
#[derive(Debug, Clone)]
pub struct Node {
    pub id: NodeId,
    pub title: String,
    pub position: Pos2,
    pub size: Vec2,
    pub inputs: Vec<Socket>,
    pub outputs: Vec<Socket>,
    pub color: Color32,
}

impl Node {
    /// Create a new node
    pub fn new(id: NodeId, title: impl Into<String>, position: Pos2) -> Self {
        Self {
            id,
            title: title.into(),
            position,
            size: Vec2::new(150.0, 100.0),
            inputs: Vec::new(),
            outputs: Vec::new(),
            color: Color32::from_rgb(60, 60, 80),
        }
    }

    /// Add an input socket
    pub fn add_input(&mut self, id: SocketId, name: impl Into<String>) {
        self.inputs.push(Socket {
            id,
            socket_type: SocketType::Input,
            name: name.into(),
            position: Pos2::ZERO,
        });
    }

    /// Add an output socket
    pub fn add_output(&mut self, id: SocketId, name: impl Into<String>) {
        self.outputs.push(Socket {
            id,
            socket_type: SocketType::Output,
            name: name.into(),
            position: Pos2::ZERO,
        });
    }

    /// Get the rectangle bounds of this node
    pub fn rect(&self) -> Rect {
        Rect::from_min_size(self.position, self.size)
    }

    /// Calculate socket positions based on node position and size
    fn update_socket_positions(&mut self) {
        let socket_radius = 6.0;
        let vertical_spacing = 25.0;
        let start_y = self.position.y + 35.0;

        // Update input socket positions (left side)
        for (i, input) in self.inputs.iter_mut().enumerate() {
            input.position = Pos2::new(
                self.position.x - socket_radius,
                start_y + i as f32 * vertical_spacing,
            );
        }

        // Update output socket positions (right side)
        for (i, output) in self.outputs.iter_mut().enumerate() {
            output.position = Pos2::new(
                self.position.x + self.size.x + socket_radius,
                start_y + i as f32 * vertical_spacing,
            );
        }
    }

    /// Draw the node in the UI
    pub fn draw(&mut self, ui: &mut Ui, is_selected: bool) -> Response {
        self.update_socket_positions();

        let rect = self.rect();
        let response = ui.allocate_rect(rect, Sense::click_and_drag());

        // Draw node background
        let bg_color = if is_selected {
            Color32::from_rgb(80, 80, 120)
        } else if response.hovered() {
            Color32::from_rgb(70, 70, 90)
        } else {
            self.color
        };

        ui.painter().rect_filled(
            rect,
            4.0, // rounding
            bg_color,
        );

        // Draw node border
        let border_color = if is_selected {
            Color32::from_rgb(150, 150, 200)
        } else {
            Color32::from_rgb(100, 100, 120)
        };
        ui.painter().rect_stroke(
            rect,
            4.0,
            Stroke::new(2.0, border_color),
        );

        // Draw title bar
        let title_rect = Rect::from_min_size(
            self.position,
            Vec2::new(self.size.x, 25.0),
        );
        ui.painter().rect_filled(
            title_rect,
            egui::Rounding {
                nw: 4.0,
                ne: 4.0,
                sw: 0.0,
                se: 0.0,
            },
            Color32::from_rgb(40, 40, 60),
        );

        // Draw title text
        ui.painter().text(
            title_rect.center(),
            egui::Align2::CENTER_CENTER,
            &self.title,
            egui::FontId::proportional(14.0),
            Color32::WHITE,
        );

        // Draw input sockets
        for input in &self.inputs {
            self.draw_socket(ui, input);
            
            // Draw socket label
            ui.painter().text(
                input.position + Vec2::new(15.0, 0.0),
                egui::Align2::LEFT_CENTER,
                &input.name,
                egui::FontId::proportional(11.0),
                Color32::from_gray(200),
            );
        }

        // Draw output sockets
        for output in &self.outputs {
            self.draw_socket(ui, output);
            
            // Draw socket label
            ui.painter().text(
                output.position + Vec2::new(-15.0, 0.0),
                egui::Align2::RIGHT_CENTER,
                &output.name,
                egui::FontId::proportional(11.0),
                Color32::from_gray(200),
            );
        }

        response
    }

    /// Draw node with canvas transformation applied
    pub fn draw_transformed(&mut self, ui: &mut Ui, is_selected: bool, canvas_offset: Vec2, canvas_scale: f32, canvas_rect: Rect) -> Response {
        self.update_socket_positions();

        // Transform position to screen space
        let screen_pos = Pos2::new(
            self.position.x * canvas_scale + canvas_offset.x + canvas_rect.min.x,
            self.position.y * canvas_scale + canvas_offset.y + canvas_rect.min.y,
        );
        let screen_size = self.size * canvas_scale;
        let rect = Rect::from_min_size(screen_pos, screen_size);
        let response = ui.allocate_rect(rect, Sense::click_and_drag());

        // Draw node background
        let bg_color = if is_selected {
            Color32::from_rgb(80, 80, 120)
        } else if response.hovered() {
            Color32::from_rgb(70, 70, 90)
        } else {
            self.color
        };

        ui.painter().rect_filled(
            rect,
            4.0 * canvas_scale, // scale rounding
            bg_color,
        );

        // Draw node border
        let border_color = if is_selected {
            Color32::from_rgb(150, 150, 200)
        } else {
            Color32::from_rgb(100, 100, 120)
        };
        ui.painter().rect_stroke(
            rect,
            4.0 * canvas_scale,
            Stroke::new(2.0 * canvas_scale, border_color),
        );

        // Draw title bar
        let title_rect = Rect::from_min_size(
            screen_pos,
            Vec2::new(screen_size.x, 25.0 * canvas_scale),
        );
        ui.painter().rect_filled(
            title_rect,
            egui::Rounding {
                nw: 4.0 * canvas_scale,
                ne: 4.0 * canvas_scale,
                sw: 0.0,
                se: 0.0,
            },
            Color32::from_rgb(40, 40, 60),
        );

        // Draw title text
        ui.painter().text(
            title_rect.center(),
            egui::Align2::CENTER_CENTER,
            &self.title,
            egui::FontId::proportional(14.0 * canvas_scale),
            Color32::WHITE,
        );

        // Draw input sockets (transform positions to screen space)
        for input in &self.inputs {
            let socket_screen_pos = Pos2::new(
                input.position.x * canvas_scale + canvas_offset.x + canvas_rect.min.x,
                input.position.y * canvas_scale + canvas_offset.y + canvas_rect.min.y,
            );
            self.draw_socket_transformed(ui, input, socket_screen_pos, canvas_scale);
            
            // Draw socket label
            ui.painter().text(
                socket_screen_pos + Vec2::new(15.0 * canvas_scale, 0.0),
                egui::Align2::LEFT_CENTER,
                &input.name,
                egui::FontId::proportional(11.0 * canvas_scale),
                Color32::from_gray(200),
            );
        }

        // Draw output sockets (transform positions to screen space)
        for output in &self.outputs {
            let socket_screen_pos = Pos2::new(
                output.position.x * canvas_scale + canvas_offset.x + canvas_rect.min.x,
                output.position.y * canvas_scale + canvas_offset.y + canvas_rect.min.y,
            );
            self.draw_socket_transformed(ui, output, socket_screen_pos, canvas_scale);
            
            // Draw socket label
            ui.painter().text(
                socket_screen_pos + Vec2::new(-15.0 * canvas_scale, 0.0),
                egui::Align2::RIGHT_CENTER,
                &output.name,
                egui::FontId::proportional(11.0 * canvas_scale),
                Color32::from_gray(200),
            );
        }

        response
    }

    /// Draw a socket (connection point)
    fn draw_socket(&self, ui: &mut Ui, socket: &Socket) {
        let radius = 6.0;
        let color = match socket.socket_type {
            SocketType::Input => Color32::from_rgb(100, 150, 255),
            SocketType::Output => Color32::from_rgb(255, 150, 100),
        };

        ui.painter().circle(
            socket.position,
            radius,
            color,
            Stroke::new(2.0, Color32::from_gray(50)),
        );
    }

    /// Draw a socket with transformation applied
    fn draw_socket_transformed(&self, ui: &mut Ui, socket: &Socket, screen_pos: Pos2, canvas_scale: f32) {
        let radius = 6.0 * canvas_scale;
        let color = match socket.socket_type {
            SocketType::Input => Color32::from_rgb(100, 150, 255),
            SocketType::Output => Color32::from_rgb(255, 150, 100),
        };

        ui.painter().circle(
            screen_pos,
            radius,
            color,
            Stroke::new(2.0 * canvas_scale, Color32::from_gray(50)),
        );
    }

    /// Get socket by ID
    pub fn get_socket(&self, socket_id: SocketId) -> Option<&Socket> {
        self.inputs
            .iter()
            .chain(self.outputs.iter())
            .find(|s| s.id == socket_id)
    }

    /// Check if a point is over a socket (pos is in canvas space)
    pub fn socket_at_pos(&self, pos: Pos2, canvas_scale: f32) -> Option<&Socket> {
        let socket_radius = 8.0 / canvas_scale; // Scale-adjusted radius for easier clicking
        
        self.inputs
            .iter()
            .chain(self.outputs.iter())
            .find(|socket| {
                socket.position.distance(pos) < socket_radius
            })
    }
}

/// Manager for the node graph
pub struct NodeGraph {
    pub nodes: HashMap<NodeId, Node>,
    pub connections: Vec<Connection>,
    next_node_id: NodeId,
    next_socket_id: SocketId,
    pub selected_node: Option<NodeId>,
    drag_start_pos: Option<Pos2>,
    
    // Connection being created
    pub active_connection: Option<(NodeId, SocketId, Pos2)>,
}

impl NodeGraph {
    /// Create a new node graph
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            connections: Vec::new(),
            next_node_id: 1,
            next_socket_id: 1,
            selected_node: None,
            drag_start_pos: None,
            active_connection: None,
        }
    }

    /// Generate a new unique node ID
    pub fn new_node_id(&mut self) -> NodeId {
        let id = self.next_node_id;
        self.next_node_id += 1;
        id
    }

    /// Generate a new unique socket ID
    pub fn new_socket_id(&mut self) -> SocketId {
        let id = self.next_socket_id;
        self.next_socket_id += 1;
        id
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, mut node: Node) {
        // Ensure all sockets have unique IDs
        for input in &mut node.inputs {
            if input.id == 0 {
                input.id = self.new_socket_id();
            }
        }
        for output in &mut node.outputs {
            if output.id == 0 {
                output.id = self.new_socket_id();
            }
        }
        
        self.nodes.insert(node.id, node);
    }

    /// Remove a node from the graph
    pub fn remove_node(&mut self, node_id: NodeId) {
        self.nodes.remove(&node_id);
        
        // Remove all connections to/from this node
        self.connections.retain(|conn| {
            conn.from_node != node_id && conn.to_node != node_id
        });
        
        if self.selected_node == Some(node_id) {
            self.selected_node = None;
        }
    }

    /// Add a connection between two sockets
    pub fn add_connection(&mut self, connection: Connection) -> bool {
        // Validate connection
        if let (Some(from_node), Some(to_node)) = (
            self.nodes.get(&connection.from_node),
            self.nodes.get(&connection.to_node),
        ) {
            if let (Some(from_socket), Some(to_socket)) = (
                from_node.get_socket(connection.from_socket),
                to_node.get_socket(connection.to_socket),
            ) {
                // Ensure we're connecting output to input
                if from_socket.socket_type == SocketType::Output
                    && to_socket.socket_type == SocketType::Input
                {
                    // Remove existing connection to the same input
                    self.connections.retain(|conn| {
                        !(conn.to_node == connection.to_node
                            && conn.to_socket == connection.to_socket)
                    });
                    
                    self.connections.push(connection);
                    return true;
                }
            }
        }
        false
    }

    /// Remove a connection
    pub fn remove_connection(&mut self, connection: &Connection) {
        self.connections.retain(|c| c != connection);
    }

    /// Draw all connections
    pub fn draw_connections(&self, ui: &mut Ui, canvas_offset: Vec2, canvas_scale: f32, canvas_rect: Rect) {
        for connection in &self.connections {
            if let (Some(from_node), Some(to_node)) = (
                self.nodes.get(&connection.from_node),
                self.nodes.get(&connection.to_node),
            ) {
                if let (Some(from_socket), Some(to_socket)) = (
                    from_node.get_socket(connection.from_socket),
                    to_node.get_socket(connection.to_socket),
                ) {
                    let start = Self::canvas_to_screen(from_socket.position, canvas_offset, canvas_scale, canvas_rect);
                    let end = Self::canvas_to_screen(to_socket.position, canvas_offset, canvas_scale, canvas_rect);
                    self.draw_connection(ui, start, end);
                }
            }
        }

        // Draw active connection being created
        if let Some((node_id, socket_id, end_pos)) = self.active_connection {
            if let Some(node) = self.nodes.get(&node_id) {
                if let Some(socket) = node.get_socket(socket_id) {
                    let start = Self::canvas_to_screen(socket.position, canvas_offset, canvas_scale, canvas_rect);
                    self.draw_connection(ui, start, end_pos);
                }
            }
        }
    }
    
    /// Convert canvas coordinates to screen coordinates
    fn canvas_to_screen(canvas_pos: Pos2, canvas_offset: Vec2, canvas_scale: f32, canvas_rect: Rect) -> Pos2 {
        Pos2::new(
            canvas_pos.x * canvas_scale + canvas_offset.x + canvas_rect.min.x,
            canvas_pos.y * canvas_scale + canvas_offset.y + canvas_rect.min.y,
        )
    }

    /// Draw a bezier curve connection between two points
    fn draw_connection(&self, ui: &mut Ui, start: Pos2, end: Pos2) {
        let control_offset = ((end.x - start.x).abs() * 0.5).max(30.0);
        
        let control1 = Pos2::new(start.x + control_offset, start.y);
        let control2 = Pos2::new(end.x - control_offset, end.y);

        // Draw bezier curve
        let points = self.bezier_points(start, control1, control2, end, 20);
        
        ui.painter().add(Shape::line(
            points,
            Stroke::new(3.0, Color32::from_rgb(150, 150, 180)),
        ));
    }

    /// Generate points along a cubic bezier curve
    fn bezier_points(&self, p0: Pos2, p1: Pos2, p2: Pos2, p3: Pos2, segments: usize) -> Vec<Pos2> {
        (0..=segments)
            .map(|i| {
                let t = i as f32 / segments as f32;
                let t2 = t * t;
                let t3 = t2 * t;
                let mt = 1.0 - t;
                let mt2 = mt * mt;
                let mt3 = mt2 * mt;

                Pos2::new(
                    mt3 * p0.x + 3.0 * mt2 * t * p1.x + 3.0 * mt * t2 * p2.x + t3 * p3.x,
                    mt3 * p0.y + 3.0 * mt2 * t * p1.y + 3.0 * mt * t2 * p2.y + t3 * p3.y,
                )
            })
            .collect()
    }

    /// Handle node interaction
    pub fn handle_interaction(&mut self, ui: &mut Ui, cursor_pos: Option<Pos2>, canvas_offset: Vec2, canvas_scale: f32, canvas_rect: Rect) {
        // Draw nodes (in reverse order so first node is on top when dragging)
        let node_ids: Vec<NodeId> = self.nodes.keys().copied().collect();
        
        for node_id in node_ids {
            if let Some(node) = self.nodes.get_mut(&node_id) {
                let is_selected = self.selected_node == Some(node_id);
                let response = node.draw_transformed(ui, is_selected, canvas_offset, canvas_scale, canvas_rect);

                // Handle node dragging
                if response.dragged() {
                    if self.drag_start_pos.is_none() {
                        self.drag_start_pos = Some(node.position);
                        self.selected_node = Some(node_id);
                    }
                    // Scale drag delta by inverse of canvas scale
                    node.position += response.drag_delta() / canvas_scale;
                } else if response.drag_stopped() {
                    self.drag_start_pos = None;
                }

                // Handle node selection
                if response.clicked() {
                    self.selected_node = Some(node_id);
                }
            }
        }

        // Handle socket interaction for connections (cursor_pos is in canvas space)
        if let Some(cursor_pos) = cursor_pos {
            if ui.input(|i| i.pointer.primary_down()) {
                // Check if we're starting a connection
                if self.active_connection.is_none() {
                    for node in self.nodes.values() {
                        if let Some(socket) = node.socket_at_pos(cursor_pos, canvas_scale) {
                            if socket.socket_type == SocketType::Output {
                                // Store end_pos in screen space for drawing
                                let screen_pos = Self::canvas_to_screen(cursor_pos, canvas_offset, canvas_scale, canvas_rect);
                                self.active_connection = Some((node.id, socket.id, screen_pos));
                                break;
                            }
                        }
                    }
                } else {
                    // Update active connection end position (convert to screen space for drawing)
                    if let Some((_, _, ref mut end_pos)) = self.active_connection {
                        *end_pos = Self::canvas_to_screen(cursor_pos, canvas_offset, canvas_scale, canvas_rect);
                    }
                }
            } else if ui.input(|i| i.pointer.primary_released()) {
                // Try to complete the connection
                if let Some((from_node, from_socket, _)) = self.active_connection {
                    for node in self.nodes.values() {
                        if let Some(socket) = node.socket_at_pos(cursor_pos, canvas_scale) {
                            if socket.socket_type == SocketType::Input {
                                let connection = Connection {
                                    from_node,
                                    from_socket,
                                    to_node: node.id,
                                    to_socket: socket.id,
                                };
                                self.add_connection(connection);
                                break;
                            }
                        }
                    }
                }
                self.active_connection = None;
            }
        }
    }

    /// Delete selected node
    pub fn delete_selected(&mut self) {
        if let Some(node_id) = self.selected_node {
            self.remove_node(node_id);
        }
    }
}

impl Default for NodeGraph {
    fn default() -> Self {
        Self::new()
    }
}
