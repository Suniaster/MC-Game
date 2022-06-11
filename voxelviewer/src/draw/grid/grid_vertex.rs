#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GridVertex {
    pub grid_position: [u8; 4],
    pub normal: [f32; 3],
    pub diffuse_color: [f32; 3],
}

impl GridVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<GridVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // position
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Uint8x4,
                    offset: 0,
                    shader_location: 0,
                },
                // normal
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: mem::size_of::<[u8; 4]>() as wgpu::BufferAddress,
                    shader_location: 1,
                },
                // diffuse color
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: (mem::size_of::<[u8; 4]>() + mem::size_of::<[f32; 3]>()) as wgpu::BufferAddress,
                    shader_location: 2,
                },
            ],
        }
    }
}
