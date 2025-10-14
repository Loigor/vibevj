/// Scene management module for VibeVJ
/// 
/// Provides scene graph functionality including:
/// - Node-based scene hierarchy
/// - Component system
/// - Scene serialization/deserialization
/// - Node graph for visual programming

pub mod node;
pub mod scene;
pub mod component;
pub mod graph;
pub mod renderer;

pub use node::{SceneNode, NodeId};
pub use scene::Scene;
pub use component::{Component, ComponentType};
pub use graph::{NodeGraph, GraphNode};
pub use renderer::SceneRenderer;
