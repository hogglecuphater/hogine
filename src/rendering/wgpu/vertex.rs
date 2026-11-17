#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
pub struct GpuVertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl GpuVertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<GpuVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

// Normalized Device Coordinates (NDC): (0,0) is center, (-1,-1) is bottom-left, (1,1) is top-right
pub const VERTICES: &[GpuVertex] = &[
    GpuVertex { position: [ 0.0,  0.5, 0.0], color: [1.0, 0.0, 0.0] }, // Top (Red)
    GpuVertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] }, // Bottom Left (Green)
    GpuVertex { position: [ 0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] }, // Bottom Right (Blue)
];