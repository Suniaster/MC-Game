use voxelviewer::view_actions::{ViewActions, ViewObjectInfo};
use world::components::*;
use world::scene::GameScene;

use nalgebra::Point3;
pub struct Cube;

impl Cube {
    pub fn create(scene: &mut GameScene, view_actions: &mut ViewActions) {
        let cube_idx = scene.entity_allocator.allocate();

        scene.add_component(&cube_idx, PhysicsComponent::new_random());

        scene.add_component(&cube_idx, SizeComponent(0.2, 0.2));

        scene.add_component(
            &cube_idx,
            RenderComponent {
                obj: view_actions.create_cube(ViewObjectInfo {
                    position: Point3::origin(),
                    color: [0.1, 1., 0.1],
                    size: [0.2, 0.2, 0.2],
                    id: 0,
                }),
            },
        );

        scene.cubes.push(cube_idx);
    }
}

pub struct Chunk;
type Mat3 = Vec<Vec<Vec<bool>>>;
pub const GRID_SIZE: usize = 16;
pub const CUBE_SIZE: f32 = 1.;
pub const CHUNK_SIZE: f32 = GRID_SIZE as f32 * CUBE_SIZE;
impl Chunk {
    pub fn create(
        scene: &mut GameScene,
        view: &mut ViewActions,
        position: Point3<f32>,
        grid: Mat3,
    ) {
        let chunk_idx = scene.entity_allocator.allocate();
        let new_chunk = view.create_grid([position.x, position.y, position.z], CUBE_SIZE, grid);
        scene.add_component(&chunk_idx, RenderComponent { obj: new_chunk });
        // scene.add_component(&chunk_idx, PositionComponent(position));
        scene.terrain_chunk.push(chunk_idx);
    }
}
