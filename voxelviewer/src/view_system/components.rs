use nalgebra::{UnitComplex, Point3};
use specs::{Component, VecStorage};

use crate::{draw::{renderer::SceneEntityRenderer, mesh::StaticVertexMesh, geometry::grid::build_grid_mesh_from_desc}, geometry::grid::{GridMatrix, self}};

pub struct LookingDirectionComponent {
    pub yaw: UnitComplex<f32>,
    pub pitch: UnitComplex<f32>,
}
impl Component for LookingDirectionComponent {type Storage = specs::VecStorage<Self>;}
impl LookingDirectionComponent{
    pub fn new(yaw: f32, pitch: f32) -> Self {
        LookingDirectionComponent {
            yaw: UnitComplex::new(yaw),
            pitch: UnitComplex::new(pitch),
        }
    }
}

pub struct PositionComponent(pub Point3<f32>);
impl Component for PositionComponent {type Storage = VecStorage<Self>;}
impl PositionComponent {pub fn new(position: Point3<f32>) -> Self {Self(position)}}


pub struct MeshRendererComponent{
    pub renderer: Option<SceneEntityRenderer>,
    pub mesh: StaticVertexMesh
}

impl Component for MeshRendererComponent {
    type Storage = VecStorage<Self>;
}

impl MeshRendererComponent {
    pub fn new(device: &wgpu::Device, mesh: StaticVertexMesh) -> MeshRendererComponent {
        let mut mesh = mesh;
        let renderer = SceneEntityRenderer::new(device, &mut mesh);
        MeshRendererComponent{
            renderer: Some(renderer),
            mesh
        }
    }

    pub fn create_without_renderer(mesh: StaticVertexMesh) -> Self{
        MeshRendererComponent{
            renderer: None,
            mesh
        }
    }

    pub fn from_grid(cube_size: f32, desc: GridMatrix) -> Self {
        let grid_mesh = grid::Grid::create_with(
            cube_size
        );

        let mesh = build_grid_mesh_from_desc(&grid_mesh, &desc);

        MeshRendererComponent::create_without_renderer(
            mesh
        )
    }

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
