// use crate::{
//   create_render_pipeline,
//   rendering::{render_utils, vertex_desc::VertexDesc},
//   texture,
//   voxel_tools::rendering::voxel_vertex::VoxelVertex,
// };
// use super::cube_vertex::VoxelVertex;
use super::mesh::vertex::StaticVertex;
use super::mesh::instance::MeshInstance;


fn create_render_pipeline(
  device: &wgpu::Device,
  layout: &wgpu::PipelineLayout,
  color_format: wgpu::TextureFormat,
  depth_format: Option<wgpu::TextureFormat>,
  vertex_layouts: &[wgpu::VertexBufferLayout],
  shader: wgpu::ShaderModuleDescriptor,
  topology: wgpu::PrimitiveTopology
) -> wgpu::RenderPipeline {
  let shader = device.create_shader_module(&shader);

  device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: Some(&format!("{:?}", shader)),
      layout: Some(layout),
      vertex: wgpu::VertexState {
          module: &shader,
          entry_point: "vs_main",
          buffers: vertex_layouts,
      },
      fragment: Some(wgpu::FragmentState {
          module: &shader,
          entry_point: "fs_main",
          targets: &[wgpu::ColorTargetState {
              format: color_format,
              blend: Some(wgpu::BlendState::REPLACE),
              write_mask: wgpu::ColorWrites::ALL,
          }],
      }),
      primitive: wgpu::PrimitiveState {
          topology,
          strip_index_format: None,
          front_face: wgpu::FrontFace::Ccw,
          cull_mode: Some(wgpu::Face::Back),
          // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
          polygon_mode: wgpu::PolygonMode::Fill,
          // Requires Features::DEPTH_CLIP_CONTROL
          unclipped_depth: false,
          // Requires Features::CONSERVATIVE_RASTERIZATION
          conservative: false,
      },
      // NE!!
      depth_stencil: depth_format.map(|format| wgpu::DepthStencilState {
          format,
          depth_write_enabled: true,
          depth_compare: wgpu::CompareFunction::Less,
          stencil: wgpu::StencilState::default(),
          bias: wgpu::DepthBiasState::default(),
      }),
      multisample: wgpu::MultisampleState {
          count: 1,
          mask: !0,
          alpha_to_coverage_enabled: false,
      },
      // If the pipeline will be used with a multiview render pass, this
      // indicates how many array layers the attachments will have.--
      multiview: None,
  })
}

pub fn create_cube_render_pipeline(
  device: &wgpu::Device,
  binds: &[&wgpu::BindGroupLayout],
  config: &wgpu::SurfaceConfiguration,
  topology: wgpu::PrimitiveTopology
)->wgpu::RenderPipeline{
    let shader = wgpu::ShaderModuleDescriptor {
        label: Some("Normal Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/static_vertex.wgsl").into()),
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
        &[StaticVertex::desc(), MeshInstance::desc()],
        shader,
        topology
    )
}