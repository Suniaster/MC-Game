use cgmath::prelude::*;

pub trait Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct DefaultVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3]
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
                },
            ],
        }
    }
}

impl DefaultVertex{
    pub fn new(position: [f32; 3], normal: [f32; 3]) -> DefaultVertex{
        DefaultVertex{
            position, normal,
        }
    }
}

pub struct DefaultQuad{
    position: cgmath::Vector3<f32>,
    size: f32
}

impl DefaultQuad{
    pub fn new() -> DefaultQuad {
        DefaultQuad{
            position: cgmath::Vector3::new(0., 0., 0.),
            size: 0.5
        }
    }

    pub fn get_vertex_list(&self) -> Vec<DefaultVertex> {
        vec![
            DefaultVertex::new([self.position.x,               self.position.y,                self.position.z], [0.1, 0.9, 0.3]),
            DefaultVertex::new([self.position.x,               self.position.y + self.size,    self.position.z], [0.1, 0.9, 0.3]),
            DefaultVertex::new([self.position.x + self.size,   self.position.y + self.size,    self.position.z], [0.1, 0.9, 0.3]),
            DefaultVertex::new([self.position.x + self.size,   self.position.y,                self.position.z], [0.1, 0.9, 0.3]),
            
            DefaultVertex::new([self.position.x,               self.position.y,                self.position.z + self.size], [0.1, 0.9, 0.3]),
            DefaultVertex::new([self.position.x,               self.position.y + self.size,    self.position.z + self.size], [0.1, 0.9, 0.3]),
            DefaultVertex::new([self.position.x + self.size,   self.position.y + self.size,    self.position.z + self.size], [0.1, 0.9, 0.3]),
            DefaultVertex::new([self.position.x + self.size,   self.position.y,                self.position.z + self.size], [0.1, 0.9, 0.3])
        ]
    }

    pub fn get_complete_vertexes(&self) -> Vec<DefaultVertex>{
        let mut ret_vec = vec![];
        let vextexes = self.get_vertex_list();
        let indexes = self.get_indexes();
        
        for triangle_i in 0..(indexes.len()/3){
            let mut v1:DefaultVertex = vextexes[indexes[triangle_i*3 + 0] as usize];
            let mut v2:DefaultVertex = vextexes[indexes[triangle_i*3 + 1] as usize];
            let mut v3:DefaultVertex = vextexes[indexes[triangle_i*3 + 2] as usize];

            let c = cgmath::Vector3::from(v1.position);
            let b = cgmath::Vector3::from(v2.position);
            let a = cgmath::Vector3::from(v3.position);
            
            let dir = (b - a).cross(c - a);
            v1.normal = dir.normalize().into();
            v2.normal = dir.normalize().into();
            v3.normal = dir.normalize().into();

            ret_vec.push(v3);ret_vec.push(v2);ret_vec.push(v1);
        }
        return ret_vec;
    }

    pub fn get_indexes(&self) -> Vec<u16>{
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

static mut INSTANCE_ID: u32 = 0;

unsafe fn gen_instance_id() -> u32{
    INSTANCE_ID += 1;
    INSTANCE_ID
}
pub struct Instance {
    pub color: [f32; 3],
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
    pub idx: u32,
}

impl Instance{
    pub fn new(position: cgmath::Vector3<f32>, color: [f32; 3]) -> Instance{
        let idx: u32;
        unsafe{
            idx = gen_instance_id();
        }
        Instance{
            position, 
            color, 
            idx,
            rotation:  cgmath::Quaternion::from_axis_angle((cgmath::Vector3{x: 0., y: 1., z: 0.}).normalize(), cgmath::Deg(0.)),
        }
    }
}

// NEW!
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    color: [f32; 3],
    model: [[f32; 4]; 4],
    normal: [[f32; 3]; 3]
}
 
// NEW!
impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        let model =
            cgmath::Matrix4::from_translation(self.position) * cgmath::Matrix4::from(self.rotation);
        InstanceRaw {
            model: model.into(),
            color: self.color,
            normal: cgmath::Matrix3::from(self.rotation).into(),
        }
    }
}

impl Vertex for InstanceRaw {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
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
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 7]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 11]>() as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 15]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 19]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 22]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 25]>() as wgpu::BufferAddress,
                    shader_location: 9,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}
 