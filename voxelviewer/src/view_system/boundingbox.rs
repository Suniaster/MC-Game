use nalgebra::Point3;
use specs::prelude::*;

use crate::{
  scene_entity::scene_entity_renderer::SceneEntityRenderer, 
  vertex::StaticVertexMesh, 
  geometry::grid::Grid
};

pub struct BoundBoxRenderer{
  pub renderer: Option<SceneEntityRenderer>,
  pub mesh: StaticVertexMesh
}

impl Component for BoundBoxRenderer {
  type Storage = VecStorage<Self>;
}

impl BoundBoxRenderer {
  pub fn new(device: &wgpu::Device, mesh: StaticVertexMesh) -> BoundBoxRenderer {
    let mut mesh = mesh;
    let renderer = SceneEntityRenderer::new(device, &mut mesh);
    BoundBoxRenderer{
      renderer: Some(renderer),
      mesh
    }
  }

  pub fn create_without_renderer(mesh: StaticVertexMesh) -> Self{
    BoundBoxRenderer{
      renderer: None,
      mesh
    }
  }

  // pub fn from_grid(cube_size: f32, grid: &Grid) -> Self {
  //   // let grid_mesh = Grid::create_with(
  //   //   cube_size
  //   // );

  //   // let mesh = grid_mesh.build_from(&grid);
  //   BoundBoxRenderer::create_without_renderer(
  //     mesh
  //   )
  // }

  pub fn update_origin(&mut self, queue: &wgpu::Queue, new_pos: Point3<f32>){
    if self.mesh.update_origin(new_pos.into()) {
      if let Some(renderer) = &self.renderer {
        queue.write_buffer(
          &renderer.instance_buffer, 
          0, 
          self.mesh.to_instance_buffer()
        );
      }
    }
  }

  pub fn update_renderer(&mut self, device: &wgpu::Device) {
    let renderer = SceneEntityRenderer::new(device, &mut self.mesh);
    self.renderer = Some(renderer);
  }

}