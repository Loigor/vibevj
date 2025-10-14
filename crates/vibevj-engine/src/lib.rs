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
pub mod mesh_gen;
pub mod material;
pub mod render_object;
pub mod render_target;
pub mod texture;

pub use renderer::Renderer;
pub use pipeline::{Pipeline, PipelineBuilder};
pub use shader::{Shader, ShaderManager};
pub use camera::{Camera, CameraUniform};
pub use mesh::{Mesh, Vertex};
pub use material::{Material, MaterialUniform, ShaderType};
pub use render_object::{RenderObject, RenderObjectDescriptor, MeshType, ModelUniform};
pub use render_target::RenderTarget;
pub use texture::Texture;
