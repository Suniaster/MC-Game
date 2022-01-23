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
            DefaultVertex{position: [self.position.x,               self.position.y,                self.position.z], color: [0.1, 0.9, 0.3]},
            DefaultVertex{position: [self.position.x,               self.position.y + self.size,    self.position.z], color: [0.1, 0.9, 0.3]},
            DefaultVertex{position: [self.position.x + self.size,   self.position.y + self.size,    self.position.z], color: [0.1, 0.9, 0.3]},
            DefaultVertex{position: [self.position.x + self.size,   self.position.y,                self.position.z], color: [0.1, 0.9, 0.3]},
            
            DefaultVertex{position: [self.position.x,               self.position.y,                self.position.z + self.size], color: [0.1, 0.9, 0.3]},
            DefaultVertex{position: [self.position.x,               self.position.y + self.size,    self.position.z + self.size], color: [0.1, 0.9, 0.3]},
            DefaultVertex{position: [self.position.x + self.size,   self.position.y + self.size,    self.position.z + self.size], color: [0.1, 0.9, 0.3]},
            DefaultVertex{position: [self.position.x + self.size,   self.position.y,                self.position.z + self.size], color: [0.1, 0.9, 0.3]}
        ]
    }

    pub fn getIndexList(&self) -> Vec<u16>{
        vec![
            2, 1, 0,
            2, 0, 3,

            6, 2, 3,
            6, 3, 7,

            6, 5, 1,
            6, 1, 2,

            3, 0, 4,
            3, 4, 7,

            1, 5, 4,
            1, 4, 0,

            5, 6, 7,
            5, 7, 4,
        ]
    }
}

pub struct Instance {
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
}

// NEW!
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    model: [[f32; 4]; 4],
}
 
// NEW!
impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            model: (cgmath::Matrix4::from_translation(self.position) * cgmath::Matrix4::from(self.rotation)).into(),
        }
    }
}

impl InstanceRaw {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            // We need to switch from using a step mode of Vertex to Instance
            // This means that our shaders will only change to use the next
            // instance when the shader starts processing a new instance
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    // While our vertex shader only uses locations 0, and 1 now, in later tutorials we'll
                    // be using 2, 3, and 4, for Vertex. We'll start at slot 5 not conflict with them later
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // A mat4 takes up 4 vertex slots as it is technically 4 vec4s. We need to define a slot
                // for each vec4. We'll have to reassemble the mat4 in
                // the shader.
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}
