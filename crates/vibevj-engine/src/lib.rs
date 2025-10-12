/// 3D rendering engine module for VibeVJ
/// 
/// This module provides the core rendering capabilities including:
/// - WGPU-based renderer
/// - Pipeline management
/// - Shader compilation and management
/// - Render passes
/// - Texture and buffer management

pub mod renderer;
pub mod pipeline;
pub mod shader;
pub mod camera;
pub mod mesh;
pub mod texture;

pub use renderer::Renderer;
pub use pipeline::{Pipeline, PipelineBuilder};
pub use shader::{Shader, ShaderManager};
pub use camera::Camera;
pub use mesh::Mesh;
pub use texture::Texture;
