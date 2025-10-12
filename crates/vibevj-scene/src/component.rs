use vibevj_common::{Color, Transform};
use serde::{Deserialize, Serialize};

/// Component types that can be attached to scene nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Component {
    /// 3D mesh renderer
    MeshRenderer {
        mesh: String,
        material: String,
    },
    /// Camera component
    Camera {
        fov: f32,
        near: f32,
        far: f32,
    },
    /// Light source
    Light {
        color: Color,
        intensity: f32,
        light_type: LightType,
    },
    /// Shader effect
    ShaderEffect {
        shader: String,
        parameters: Vec<(String, ShaderParameter)>,
    },
    /// Audio analyzer
    AudioAnalyzer {
        fft_size: usize,
        enabled: bool,
    },
    /// Script behavior
    Script {
        script_name: String,
        enabled: bool,
    },
    /// 2D sprite renderer
    SpriteRenderer {
        texture: String,
        color: Color,
    },
    /// Video player
    VideoPlayer {
        video_path: String,
        playing: bool,
        loop_enabled: bool,
    },
}

impl Component {
    /// Get the component type as a string
    pub fn component_type(&self) -> &str {
        match self {
            Component::MeshRenderer { .. } => "MeshRenderer",
            Component::Camera { .. } => "Camera",
            Component::Light { .. } => "Light",
            Component::ShaderEffect { .. } => "ShaderEffect",
            Component::AudioAnalyzer { .. } => "AudioAnalyzer",
            Component::Script { .. } => "Script",
            Component::SpriteRenderer { .. } => "SpriteRenderer",
            Component::VideoPlayer { .. } => "VideoPlayer",
        }
    }
}

/// Types of lights
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LightType {
    Directional,
    Point,
    Spot,
}

/// Shader parameter types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShaderParameter {
    Float(f32),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    Color(Color),
    Texture(String),
    Bool(bool),
    Int(i32),
}

/// Component type enum for querying
pub enum ComponentType {
    MeshRenderer,
    Camera,
    Light,
    ShaderEffect,
    AudioAnalyzer,
    Script,
    SpriteRenderer,
    VideoPlayer,
}
