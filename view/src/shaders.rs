

// main.rs
pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    // NEW!
    render_pipeline: wgpu::RenderPipeline,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

pub const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.5, 0.5, 0.0], color: [0.5, 0.0, 0.5] }, // A
    Vertex { position: [-0.5, -0.5, 0.0], color: [0.5, 0.0, 0.5] }, // B
    Vertex { position: [0.5, -0.5, 0.0], color: [0.5, 0.0, 0.5] }, // C
    Vertex { position: [0.5, 0.5, 0.0], color: [0.5, 0.0, 0.5] }, // D
]
;
pub const INDICES: &[u16] = &[
    0, 1, 3,
    1, 2, 3
];

impl Vertex {
    const _ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::_ATTRIBS,
        }
    }
}