use glam::Vec3;
use bytemuck::{Pod, Zeroable};

/// Vertex representation for 3D meshes
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
    pub color: [f32; 3],
}

impl Vertex {
    pub fn new(position: [f32; 3], normal: [f32; 3], uv: [f32; 2], color: [f32; 3]) -> Self {
        Self {
            position,
            normal,
            uv,
            color,
        }
    }

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // Normal
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // UV
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // Color
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

/// 3D Mesh with vertices and indices
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub vertex_buffer: Option<wgpu::Buffer>,
    pub index_buffer: Option<wgpu::Buffer>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        Self {
            vertices,
            indices,
            vertex_buffer: None,
            index_buffer: None,
        }
    }

    /// Upload mesh data to GPU
    pub fn upload(&mut self, device: &wgpu::Device) {
        use wgpu::util::DeviceExt;

        self.vertex_buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&self.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        }));

        self.index_buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&self.indices),
            usage: wgpu::BufferUsages::INDEX,
        }));
    }

    /// Create a cube mesh
    pub fn cube() -> Self {
        let vertices = vec![
            // Front face
            Vertex::new([-0.5, -0.5, 0.5], [0.0, 0.0, 1.0], [0.0, 0.0], [1.0, 0.0, 0.0]),
            Vertex::new([0.5, -0.5, 0.5], [0.0, 0.0, 1.0], [1.0, 0.0], [0.0, 1.0, 0.0]),
            Vertex::new([0.5, 0.5, 0.5], [0.0, 0.0, 1.0], [1.0, 1.0], [0.0, 0.0, 1.0]),
            Vertex::new([-0.5, 0.5, 0.5], [0.0, 0.0, 1.0], [0.0, 1.0], [1.0, 1.0, 0.0]),
            // Add more faces as needed...
        ];

        let indices = vec![
            0, 1, 2, 2, 3, 0, // Front face
        ];

        Self::new(vertices, indices)
    }

    /// Create a quad mesh
    pub fn quad() -> Self {
        let vertices = vec![
            Vertex::new([-1.0, -1.0, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0], [1.0, 1.0, 1.0]),
            Vertex::new([1.0, -1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0], [1.0, 1.0, 1.0]),
            Vertex::new([1.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 1.0], [1.0, 1.0, 1.0]),
            Vertex::new([-1.0, 1.0, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0], [1.0, 1.0, 1.0]),
        ];

        let indices = vec![0, 1, 2, 2, 3, 0];

        Self::new(vertices, indices)
    }
}
