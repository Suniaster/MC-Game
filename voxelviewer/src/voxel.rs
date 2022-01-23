pub trait Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct DefaultVertex {
    position: [f32; 3],
    color: [f32; 3]
}

impl Vertex for DefaultVertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<DefaultVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ],
        }
    }
}


pub struct DefaultQuad{
    position: cgmath::Vector3<f32>,
    color: cgmath::Vector3<f32>,
    size: f32
}

impl DefaultQuad{
    pub fn new() -> DefaultQuad {
        DefaultQuad{
            position: cgmath::Vector3::new(0., 0., 0.),
            color: cgmath::Vector3::new(0., 2., 3.),
            size: 0.5
        }
    }

    pub fn getVertexList(&self) -> Vec<DefaultVertex> {
        vec![
            DefaultVertex{position: [self.position.x, self.position.y, self.position.z], color: [0.1, 0.9, 0.3]},
            DefaultVertex{position: [self.position.x, self.position.y + self.size, self.position.z], color: [0.1, 0.9, 0.3]},
            DefaultVertex{position: [self.position.x + self.size, self.position.y + self.size, self.position.z], color: [0.1, 0.9, 0.3]},
            DefaultVertex{position: [self.position.x + self.size, self.position.y, self.position.z], color: [0.1, 0.9, 0.3]}
        ]
    }

    pub fn getIndexList(&self) -> Vec<u16>{
        vec![
            2, 1, 0,
            2, 0, 3
        ]
    }
}
