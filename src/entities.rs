use world::components::*;
use world::scene::GameScene;
use voxelviewer::ViewObjectInfo;

use cgmath::Vector3;
pub struct Cube;
use perlin2d::PerlinNoise2D;

impl Cube {
    pub fn create(scene: &mut GameScene, view_actions: &mut voxelviewer::ViewActions) {
        let cube_idx = scene.entity_allocator.allocate();

        scene.add_component(&cube_idx, PhysicsComponent::new_random());

        scene.add_component(&cube_idx, SizeComponent(0.2, 0.2));

        scene.add_component(&cube_idx, RenderComponent{
            obj: view_actions.create_cube(
                ViewObjectInfo{
                    position: cgmath::Vector3::from([0., 0., 0.]), 
                    color: [0.1, 1., 0.1],
                    size: [0.2, 0.2, 0.2],
                    id: 0
                }
            )
        });

        scene.cubes.push(cube_idx);
    }
}

pub struct Chunk;
type Mat3 = Vec<Vec<Vec<bool>>>;
pub const GRID_SIZE:usize = 16;
pub const CUBE_SIZE:f32 = 1.;
pub const CHUNK_SIZE:f32 = GRID_SIZE as f32 * CUBE_SIZE;
impl Chunk {
    pub fn create(scene: &mut GameScene, view: &mut voxelviewer::ViewActions, position: Vector3<f32>, grid: Mat3) {
        let chunk_idx = scene.entity_allocator.allocate();
        let new_chunk = view.create_grid(
            [position.x, position.y, position.z],
            CUBE_SIZE,
            grid
        );
        scene.add_component(&chunk_idx, RenderComponent{
            obj: new_chunk
        });
        scene.add_component(&chunk_idx, PositionComponent(position));
        scene.terrain_chunk.push(chunk_idx);
    }


    pub fn create_chunk_mat_at(postion: Vector3<f32>) -> Mat3{
        let mut mat:Mat3 = vec![];
        let perlin = PerlinNoise2D::new(
            8, 
            1.0, 
            1.0, 
            0.5, 
            2.0, 
            (1.0, 1.0), 
            0., 
            101
        );
      
        for i in 0..GRID_SIZE{
            mat.push(vec![]);
            for j in 0..GRID_SIZE{
                mat[i].push(vec![]);
                for k in 0..GRID_SIZE{

                    let pos_x = postion.x + i as f32 * CUBE_SIZE - CHUNK_SIZE/2.;
                    let pos_y = postion.y + j as f32 * CUBE_SIZE - CHUNK_SIZE/2.;
                    let pos_z = postion.z + k as f32 * CUBE_SIZE - CHUNK_SIZE/2.;

                    let mut val = perlin.get_noise(pos_x as f64, pos_z as f64);
                    val += 1.;
                    val /= 2.;
                    val *= 7.;
            
                    mat[i][j].push(val > pos_y.into());
                }
            }
        }

        return mat;
    }
}