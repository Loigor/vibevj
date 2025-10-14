use serde::{Deserialize, Serialize};
use vibevj_common::Color;

/// Material properties for rendering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    /// Base color (albedo)
    pub color: Color,
    
    /// Metallic factor (0.0 = dielectric, 1.0 = metallic)
    pub metallic: f32,
    
    /// Roughness factor (0.0 = smooth, 1.0 = rough)
    pub roughness: f32,
    
    /// Emissive color
    pub emissive: Color,
    
    /// Shader type to use
    pub shader_type: ShaderType,
}

/// Types of shaders available
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShaderType {
    /// Basic unlit shader
    Unlit,
    
    /// Simple lit shader with basic lighting
    BasicLit,
    
    /// Physically-based rendering
    PBR,
    
    /// Custom shader (references shader by name)
    Custom,
}

impl Material {
    /// Create a new material with default values
    pub fn new() -> Self {
        Self {
            color: Color::WHITE,
            metallic: 0.0,
            roughness: 0.5,
            emissive: Color::BLACK,
            shader_type: ShaderType::BasicLit,
        }
    }
    
    /// Create an unlit material with a specific color
    pub fn unlit(color: Color) -> Self {
        Self {
            color,
            metallic: 0.0,
            roughness: 1.0,
            emissive: Color::BLACK,
            shader_type: ShaderType::Unlit,
        }
    }
    
    /// Create an emissive material
    pub fn emissive(color: Color, intensity: f32) -> Self {
        Self {
            color: Color::BLACK,
            metallic: 0.0,
            roughness: 1.0,
            emissive: Color {
                r: color.r * intensity,
                g: color.g * intensity,
                b: color.b * intensity,
                a: color.a,
            },
            shader_type: ShaderType::Unlit,
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::new()
    }
}

/// Material uniform data for GPU
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MaterialUniform {
    pub color: [f32; 4],
    pub emissive: [f32; 4],
    pub metallic: f32,
    pub roughness: f32,
    pub _padding: [f32; 2],
}

impl From<&Material> for MaterialUniform {
    fn from(material: &Material) -> Self {
        Self {
            color: [material.color.r, material.color.g, material.color.b, material.color.a],
            emissive: [material.emissive.r, material.emissive.g, material.emissive.b, material.emissive.a],
            metallic: material.metallic,
            roughness: material.roughness,
            _padding: [0.0; 2],
        }
    }
}
