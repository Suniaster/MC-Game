use super::create_render_pipeline;

mod grid_vertex;
mod grid_instance;


pub fn create_grid_render_pipeline(
  device: &wgpu::Device,
  binds: &[&wgpu::BindGroupLayout],
  config: &wgpu::SurfaceConfiguration,
  topology: wgpu::PrimitiveTopology
)->wgpu::RenderPipeline{
    let shader = wgpu::ShaderModuleDescriptor {
        label: Some("Normal Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("./grid_pipeline.wgsl").into()),
    };
    let render_pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: binds,
            push_constant_ranges: &[],
        });

    create_render_pipeline(
        &device,
        &render_pipeline_layout,
        config.format,
        Some( wgpu::TextureFormat::Depth32Float),
        &[
            grid_vertex::GridVertex::desc(), 
            grid_instance::GridInstance::desc()
        ],
        shader,
        topology
    )
}

pub mod grid_entity_mesh;
use wgpu::util::DeviceExt;
use grid_entity_mesh::GridEntityMesh;

pub struct GridRenderer{
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub instance_buffer: wgpu::Buffer,
    pub num_indices: u32,
    pub num_vertices: u32
}

impl GridRenderer{
    pub fn new(device: &wgpu::Device, grid_mesh: &GridEntityMesh)->GridRenderer{
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                usage: wgpu::BufferUsages::VERTEX,
                contents: grid_mesh.to_vertex_buffer()
            }
        );

        let indices = grid_mesh.get_indices_for_square_mesh();
        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                usage: wgpu::BufferUsages::INDEX,
                contents: bytemuck::cast_slice(&indices)
            }
        );

        let instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                contents: grid_mesh.to_instance_buffer()
            }
        );

        GridRenderer{
            index_buffer,
            num_indices: indices.len() as u32,
            num_vertices: grid_mesh.vertices.len() as u32,
            instance_buffer,
            vertex_buffer,
        }
    }
}
  
  
  