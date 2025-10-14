/// GUI module for VibeVJ using egui
/// 
/// Provides the graphical user interface with:
/// - Three-panel layout (left: 25%, center: 60%, right: 15%)
/// - Scene editor
/// - Scene sequencer
/// - Prefab and resource browser
/// - Node-based visual programming interface

pub mod app;
pub mod panels;
pub mod widgets;
pub mod scene_editor;

pub use app::GuiApp;
pub use panels::{LeftPanel, CenterPanel, RightPanel, PanelContent};
pub use scene_editor::SceneEditor;
