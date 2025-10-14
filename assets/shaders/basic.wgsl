// Vertex shader for basic 3D rendering

struct CameraUniform {
    view_proj: mat4x4<f32>,
};

struct ModelUniform {
    model: mat4x4<f32>,
};

struct MaterialUniform {
    color: vec4<f32>,
    emissive: vec4<f32>,
    metallic: f32,
    roughness: f32,
    _padding: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(1) @binding(0)
var<uniform> model: ModelUniform;

@group(2) @binding(0)
var<uniform> material: MaterialUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) color: vec3<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    let world_position = model.model * vec4<f32>(in.position, 1.0);
    out.world_position = world_position.xyz;
    out.clip_position = camera.view_proj * world_position;
    
    // Transform normal to world space (assuming uniform scaling)
    out.world_normal = normalize((model.model * vec4<f32>(in.normal, 0.0)).xyz);
    
    out.uv = in.uv;
    out.color = in.color;
    
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Simple directional light
    let light_dir = normalize(vec3<f32>(0.5, 1.0, 0.3));
    let ambient = 0.3;
    
    let diffuse = max(dot(in.world_normal, light_dir), 0.0);
    let lighting = ambient + diffuse * 0.7;
    
    // Combine material color with vertex color
    let base_color = material.color.rgb * in.color;
    let final_color = base_color * lighting + material.emissive.rgb;
    
    return vec4<f32>(final_color, material.color.a);
}
