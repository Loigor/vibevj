use crate::node::{SceneNode, NodeId};
use vibevj_common::{Result, VibeVJError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Scene containing a hierarchy of nodes
#[derive(Debug, Serialize, Deserialize)]
pub struct Scene {
    pub name: String,
    pub root: NodeId,
    nodes: HashMap<NodeId, SceneNode>,
    next_id: u64,
}

impl Scene {
    /// Create a new empty scene
    pub fn new(name: String) -> Self {
        let root_id = NodeId::new(0);
        let mut nodes = HashMap::new();
        
        let root_node = SceneNode::new(root_id, "Root".to_string());
        nodes.insert(root_id, root_node);

        Self {
            name,
            root: root_id,
            nodes,
            next_id: 1,
        }
    }

    /// Generate a new unique node ID
    fn generate_id(&mut self) -> NodeId {
        let id = NodeId::new(self.next_id);
        self.next_id += 1;
        id
    }

    /// Create a new node and add it to the scene
    pub fn create_node(&mut self, name: String, parent: Option<NodeId>) -> Result<NodeId> {
        let id = self.generate_id();
        let mut node = SceneNode::new(id, name);

        let parent_id = parent.unwrap_or(self.root);
        node.parent = Some(parent_id);

        // Add to parent's children
        if let Some(parent_node) = self.nodes.get_mut(&parent_id) {
            parent_node.add_child(id);
        } else {
            return Err(VibeVJError::SceneError(format!(
                "Parent node {:?} not found",
                parent_id
            )));
        }

        self.nodes.insert(id, node);
        Ok(id)
    }

    /// Get a node by ID
    pub fn get_node(&self, id: NodeId) -> Option<&SceneNode> {
        self.nodes.get(&id)
    }

    /// Get a mutable node by ID
    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut SceneNode> {
        self.nodes.get_mut(&id)
    }

    /// Remove a node and all its children
    pub fn remove_node(&mut self, id: NodeId) -> Result<()> {
        if id == self.root {
            return Err(VibeVJError::SceneError(
                "Cannot remove root node".to_string(),
            ));
        }

        // Get children to remove recursively
        let children: Vec<NodeId> = self
            .nodes
            .get(&id)
            .map(|node| node.children.clone())
            .unwrap_or_default();

        // Remove from parent's children list
        if let Some(parent_id) = self.nodes.get(&id).and_then(|n| n.parent) {
            if let Some(parent) = self.nodes.get_mut(&parent_id) {
                parent.remove_child(id);
            }
        }

        // Remove children recursively
        for child_id in children {
            self.remove_node(child_id)?;
        }

        // Remove the node itself
        self.nodes.remove(&id);
        Ok(())
    }

    /// Get all nodes in the scene
    pub fn nodes(&self) -> impl Iterator<Item = &SceneNode> {
        self.nodes.values()
    }

    /// Get all node IDs
    pub fn node_ids(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.nodes.keys().copied()
    }

    /// Clear the scene (except root)
    pub fn clear(&mut self) {
        let root = self.nodes.remove(&self.root).unwrap();
        self.nodes.clear();
        self.nodes.insert(self.root, root);
        self.next_id = 1;
    }

    /// Serialize the scene to JSON
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| VibeVJError::SerializationError(format!("JSON serialization error: {}", e)))
    }

    /// Deserialize a scene from JSON
    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json)
            .map_err(|e| VibeVJError::SerializationError(format!("JSON deserialization error: {}", e)))
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new("Untitled Scene".to_string())
    }
}
