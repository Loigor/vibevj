use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Node in a visual programming graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub node_type: String,
    pub position: [f32; 2],
    pub inputs: Vec<GraphPort>,
    pub outputs: Vec<GraphPort>,
    pub parameters: HashMap<String, serde_json::Value>,
}

impl GraphNode {
    pub fn new(id: String, node_type: String, position: [f32; 2]) -> Self {
        Self {
            id,
            node_type,
            position,
            inputs: Vec::new(),
            outputs: Vec::new(),
            parameters: HashMap::new(),
        }
    }

    pub fn add_input(&mut self, name: String, port_type: PortType) {
        self.inputs.push(GraphPort::new(name, port_type));
    }

    pub fn add_output(&mut self, name: String, port_type: PortType) {
        self.outputs.push(GraphPort::new(name, port_type));
    }
}

/// Port on a graph node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphPort {
    pub name: String,
    pub port_type: PortType,
    pub connected_to: Option<String>, // "node_id:port_name"
}

impl GraphPort {
    pub fn new(name: String, port_type: PortType) -> Self {
        Self {
            name,
            port_type,
            connected_to: None,
        }
    }
}

/// Types of ports
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PortType {
    Float,
    Vec2,
    Vec3,
    Vec4,
    Color,
    Texture,
    Audio,
    Geometry,
    Shader,
    Any,
}

/// Connection between two ports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphConnection {
    pub from_node: String,
    pub from_port: String,
    pub to_node: String,
    pub to_port: String,
}

/// Visual programming node graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeGraph {
    pub name: String,
    pub nodes: HashMap<String, GraphNode>,
    pub connections: Vec<GraphConnection>,
}

impl NodeGraph {
    pub fn new(name: String) -> Self {
        Self {
            name,
            nodes: HashMap::new(),
            connections: Vec::new(),
        }
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, node: GraphNode) {
        self.nodes.insert(node.id.clone(), node);
    }

    /// Remove a node and all its connections
    pub fn remove_node(&mut self, node_id: &str) {
        self.nodes.remove(node_id);
        self.connections
            .retain(|c| c.from_node != node_id && c.to_node != node_id);
    }

    /// Connect two ports
    pub fn connect(&mut self, connection: GraphConnection) -> Result<(), String> {
        // Validate nodes exist
        if !self.nodes.contains_key(&connection.from_node) {
            return Err(format!("Source node '{}' not found", connection.from_node));
        }
        if !self.nodes.contains_key(&connection.to_node) {
            return Err(format!("Target node '{}' not found", connection.to_node));
        }

        // TODO: Validate port types match

        self.connections.push(connection);
        Ok(())
    }

    /// Disconnect two ports
    pub fn disconnect(&mut self, from_node: &str, from_port: &str, to_node: &str, to_port: &str) {
        self.connections.retain(|c| {
            !(c.from_node == from_node
                && c.from_port == from_port
                && c.to_node == to_node
                && c.to_port == to_port)
        });
    }

    /// Get all connections for a node
    pub fn get_node_connections(&self, node_id: &str) -> Vec<&GraphConnection> {
        self.connections
            .iter()
            .filter(|c| c.from_node == node_id || c.to_node == node_id)
            .collect()
    }

    /// Evaluate the graph (placeholder)
    pub fn evaluate(&self) -> Result<(), String> {
        // TODO: Implement graph evaluation
        log::info!("Evaluating node graph: {}", self.name);
        Ok(())
    }
}

impl Default for NodeGraph {
    fn default() -> Self {
        Self::new("Untitled Graph".to_string())
    }
}

/// Predefined node types
pub mod node_types {
    use super::*;

    pub fn create_shader_node(id: String, position: [f32; 2]) -> GraphNode {
        let mut node = GraphNode::new(id, "Shader".to_string(), position);
        node.add_input("Time".to_string(), PortType::Float);
        node.add_input("Audio".to_string(), PortType::Audio);
        node.add_output("Color".to_string(), PortType::Color);
        node
    }

    pub fn create_audio_node(id: String, position: [f32; 2]) -> GraphNode {
        let mut node = GraphNode::new(id, "AudioAnalyzer".to_string(), position);
        node.add_output("Bass".to_string(), PortType::Float);
        node.add_output("Mid".to_string(), PortType::Float);
        node.add_output("Treble".to_string(), PortType::Float);
        node
    }

    pub fn create_transform_node(id: String, position: [f32; 2]) -> GraphNode {
        let mut node = GraphNode::new(id, "Transform".to_string(), position);
        node.add_input("Position".to_string(), PortType::Vec3);
        node.add_input("Rotation".to_string(), PortType::Vec3);
        node.add_input("Scale".to_string(), PortType::Vec3);
        node.add_output("Transform".to_string(), PortType::Any);
        node
    }

    pub fn create_output_node(id: String, position: [f32; 2]) -> GraphNode {
        let mut node = GraphNode::new(id, "Output".to_string(), position);
        node.add_input("Color".to_string(), PortType::Color);
        node.add_input("Geometry".to_string(), PortType::Geometry);
        node
    }
}
