use std::collections::HashMap;
use vibevj_common::{Result, VibeVJError};

/// Shader wrapper
pub struct Shader {
    pub module: wgpu::ShaderModule,
    pub source: String,
}

impl Shader {
    /// Create a shader from WGSL source
    pub fn from_wgsl(device: &wgpu::Device, source: &str, label: Option<&str>) -> Self {
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label,
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });

        Self {
            module,
            source: source.to_string(),
        }
    }
}

/// Shader manager for loading and caching shaders
pub struct ShaderManager {
    shaders: HashMap<String, Shader>,
}

impl ShaderManager {
    pub fn new() -> Self {
        Self {
            shaders: HashMap::new(),
        }
    }

    /// Load a shader from WGSL source
    pub fn load_shader(
        &mut self,
        device: &wgpu::Device,
        name: String,
        source: &str,
    ) -> Result<&Shader> {
        if !self.shaders.contains_key(&name) {
            let shader = Shader::from_wgsl(device, source, Some(&name));
            self.shaders.insert(name.clone(), shader);
        }
        
        self.shaders
            .get(&name)
            .ok_or_else(|| VibeVJError::RenderError(format!("Shader '{}' not found", name)))
    }

    /// Get a shader by name
    pub fn get_shader(&self, name: &str) -> Option<&Shader> {
        self.shaders.get(name)
    }

    /// Remove a shader
    pub fn remove_shader(&mut self, name: &str) -> Option<Shader> {
        self.shaders.remove(name)
    }
}

impl Default for ShaderManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Default shaders
pub mod default_shaders {
    pub const BASIC_VERTEX: &str = r#"
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

struct Uniforms {
    view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = uniforms.view_proj * uniforms.model * vec4<f32>(in.position, 1.0);
    out.color = in.color;
    return out;
}
"#;

    pub const BASIC_FRAGMENT: &str = r#"
struct FragmentInput {
    @location(0) color: vec3<f32>,
};

@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
"#;

    pub const SHADER_TOY: &str = r#"
struct Uniforms {
    time: f32,
    resolution: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    let uv = pos.xy / uniforms.resolution;
    let col = 0.5 + 0.5 * cos(uniforms.time + uv.xyx + vec3<f32>(0.0, 2.0, 4.0));
    return vec4<f32>(col, 1.0);
}
"#;
}
