/// Scripting module for VibeVJ using Rhai
/// 
/// Provides scripting capabilities for:
/// - Programmatic scene creation
/// - Animation scripting
/// - Custom behaviors
/// - Procedural generation
/// - Audio-reactive scripting

pub mod engine;
pub mod api;

pub use engine::ScriptEngine;
pub use api::register_api;
