use vibevj_common::{Result, VibeVJError};
use crate::shader::Shader;

/// Render pipeline wrapper
pub struct Pipeline {
    pub render_pipeline: wgpu::RenderPipeline,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl Pipeline {
    /// Create a new pipeline builder
    pub fn builder() -> PipelineBuilder {
        PipelineBuilder::default()
    }
}

/// Builder for creating render pipelines
#[derive(Default)]
pub struct PipelineBuilder {
    vertex_shader: Option<wgpu::ShaderModule>,
    fragment_shader: Option<wgpu::ShaderModule>,
    vertex_buffers: Vec<wgpu::VertexBufferLayout<'static>>,
    bind_group_layouts: Vec<wgpu::BindGroupLayout>,
    topology: wgpu::PrimitiveTopology,
    cull_mode: Option<wgpu::Face>,
    depth_stencil: Option<wgpu::DepthStencilState>,
}

impl PipelineBuilder {
    pub fn new() -> Self {
        Self {
            topology: wgpu::PrimitiveTopology::TriangleList,
            cull_mode: Some(wgpu::Face::Back),
            ..Default::default()
        }
    }

    pub fn vertex_shader(mut self, shader: wgpu::ShaderModule) -> Self {
        self.vertex_shader = Some(shader);
        self
    }

    pub fn fragment_shader(mut self, shader: wgpu::ShaderModule) -> Self {
        self.fragment_shader = Some(shader);
        self
    }

    pub fn topology(mut self, topology: wgpu::PrimitiveTopology) -> Self {
        self.topology = topology;
        self
    }

    pub fn cull_mode(mut self, cull_mode: Option<wgpu::Face>) -> Self {
        self.cull_mode = cull_mode;
        self
    }

    pub fn depth_stencil(mut self, depth_stencil: wgpu::DepthStencilState) -> Self {
        self.depth_stencil = Some(depth_stencil);
        self
    }

    pub fn build(
        self,
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        label: Option<&str>,
    ) -> Result<Pipeline> {
        let vertex_shader = self.vertex_shader
            .ok_or_else(|| VibeVJError::RenderError("Vertex shader not set".to_string()))?;
        
        let fragment_shader = self.fragment_shader
            .ok_or_else(|| VibeVJError::RenderError("Fragment shader not set".to_string()))?;

        // Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Pipeline Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex_shader,
                entry_point: Some("vs_main"),
                buffers: &self.vertex_buffers,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: self.topology,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: self.cull_mode,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: self.depth_stencil,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        Ok(Pipeline {
            render_pipeline,
            bind_group_layout,
        })
    }
}
