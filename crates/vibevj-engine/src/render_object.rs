use glam::Mat4;
use serde::{Deserialize, Serialize};
use crate::{Mesh, Material};

/// A renderable 3D object combining mesh, material, and transform
#[derive(Debug)]
pub struct RenderObject {
    pub mesh: Mesh,
    pub material: Material,
    pub transform: Mat4,
    pub vertex_buffer: Option<wgpu::Buffer>,
    pub index_buffer: Option<wgpu::Buffer>,
    pub material_buffer: Option<wgpu::Buffer>,
    pub model_buffer: Option<wgpu::Buffer>,
    pub material_bind_group: Option<wgpu::BindGroup>,
    pub model_bind_group: Option<wgpu::BindGroup>,
}

impl RenderObject {
    /// Create a new render object
    pub fn new(mesh: Mesh, material: Material, transform: Mat4) -> Self {
        Self {
            mesh,
            material,
            transform,
            vertex_buffer: None,
            index_buffer: None,
            material_buffer: None,
            model_buffer: None,
            material_bind_group: None,
            model_bind_group: None,
        }
    }
    
    /// Upload mesh and material data to GPU
    pub fn upload(&mut self, device: &wgpu::Device, material_layout: &wgpu::BindGroupLayout, model_layout: &wgpu::BindGroupLayout) {
        use wgpu::util::DeviceExt;
        
        // Upload mesh data
        self.vertex_buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&self.mesh.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        }));
        
        self.index_buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&self.mesh.indices),
            usage: wgpu::BufferUsages::INDEX,
        }));
        
        // Create material uniform
        let material_uniform: crate::MaterialUniform = (&self.material).into();
        self.material_buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Material Buffer"),
            contents: bytemuck::cast_slice(&[material_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        }));
        
        // Create model uniform
        let model_uniform = ModelUniform {
            model: self.transform.to_cols_array_2d(),
        };
        self.model_buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Model Buffer"),
            contents: bytemuck::cast_slice(&[model_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        }));
        
        // Create bind groups
        if let Some(ref material_buffer) = self.material_buffer {
            self.material_bind_group = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Material Bind Group"),
                layout: material_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: material_buffer.as_entire_binding(),
                }],
            }));
        }
        
        if let Some(ref model_buffer) = self.model_buffer {
            self.model_bind_group = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Model Bind Group"),
                layout: model_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: model_buffer.as_entire_binding(),
                }],
            }));
        }
    }
    
    /// Update the transform matrix
    pub fn update_transform(&mut self, queue: &wgpu::Queue, transform: Mat4) {
        self.transform = transform;
        
        if let Some(ref model_buffer) = self.model_buffer {
            let model_uniform = ModelUniform {
                model: self.transform.to_cols_array_2d(),
            };
            queue.write_buffer(model_buffer, 0, bytemuck::cast_slice(&[model_uniform]));
        }
    }
}

/// Model uniform data for GPU
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelUniform {
    pub model: [[f32; 4]; 4],
}

/// Descriptor for creating a render object from serialized data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderObjectDescriptor {
    pub mesh_type: MeshType,
    pub material: Material,
    pub position: [f32; 3],
    pub rotation: [f32; 3], // Euler angles in radians
    pub scale: [f32; 3],
}

/// Types of procedural meshes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MeshType {
    Cube { size: u32 }, // size in fixed point (divide by 100)
    Sphere { radius: u32, segments: u32, rings: u32 },
    Plane { width: u32, height: u32, subdivisions_x: u32, subdivisions_y: u32 },
    Cylinder { radius: u32, height: u32, segments: u32 },
}

impl RenderObjectDescriptor {
    /// Create a render object from this descriptor
    pub fn create_object(&self) -> RenderObject {
        use crate::mesh_gen::*;
        
        let mesh = match self.mesh_type {
            MeshType::Cube { size } => create_cube(size as f32 / 100.0),
            MeshType::Sphere { radius, segments, rings } => {
                create_sphere(radius as f32 / 100.0, segments, rings)
            }
            MeshType::Plane { width, height, subdivisions_x, subdivisions_y } => {
                create_plane(width as f32 / 100.0, height as f32 / 100.0, subdivisions_x, subdivisions_y)
            }
            MeshType::Cylinder { radius, height, segments } => {
                create_cylinder(radius as f32 / 100.0, height as f32 / 100.0, segments)
            }
        };
        
        // Build transform matrix
        let translation = Mat4::from_translation(self.position.into());
        let rotation = Mat4::from_euler(
            glam::EulerRot::XYZ,
            self.rotation[0],
            self.rotation[1],
            self.rotation[2],
        );
        let scale = Mat4::from_scale(self.scale.into());
        let transform = translation * rotation * scale;
        
        RenderObject::new(mesh, self.material.clone(), transform)
    }
}
