use vibevj_common::{Transform, Color};
use serde::{Deserialize, Serialize};
use crate::component::Component;

/// Unique identifier for scene nodes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub u64);

impl NodeId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

/// Scene node in the hierarchy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneNode {
    pub id: NodeId,
    pub name: String,
    pub transform: Transform,
    pub visible: bool,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub components: Vec<Component>,
}

impl SceneNode {
    /// Create a new scene node
    pub fn new(id: NodeId, name: String) -> Self {
        Self {
            id,
            name,
            transform: Transform::default(),
            visible: true,
            parent: None,
            children: Vec::new(),
            components: Vec::new(),
        }
    }

    /// Add a child node
    pub fn add_child(&mut self, child_id: NodeId) {
        if !self.children.contains(&child_id) {
            self.children.push(child_id);
        }
    }

    /// Remove a child node
    pub fn remove_child(&mut self, child_id: NodeId) {
        self.children.retain(|&id| id != child_id);
    }

    /// Add a component
    pub fn add_component(&mut self, component: Component) {
        self.components.push(component);
    }

    /// Get a component by type
    pub fn get_component(&self, component_type: &str) -> Option<&Component> {
        self.components
            .iter()
            .find(|c| c.component_type() == component_type)
    }

    /// Get a mutable component by type
    pub fn get_component_mut(&mut self, component_type: &str) -> Option<&mut Component> {
        self.components
            .iter_mut()
            .find(|c| c.component_type() == component_type)
    }
}
