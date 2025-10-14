/// Scene editor module for node-based visual programming
/// 
/// This module provides the scene editor interface where users can create
/// and manipulate scenes using a node-based visual programming approach.

mod scene_editor;
pub mod nodes;

pub use scene_editor::SceneEditor;
