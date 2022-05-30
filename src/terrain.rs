

use super::Control;
use super::voxelviewer::ViewActions;
use world::{components::*, scene::GameScene};
use super::entities;
use cgmath::{Vector3, Point3};

pub fn terrain_system(control: &mut Control, actions: &mut ViewActions){
    let camera_pos = actions.state.camera.position;
    let chunk_already_loaded = chunk_is_loaded(&mut control.world, camera_pos);
    
    if !chunk_already_loaded{
        let chunk_idx = position_to_chunk_idx(camera_pos);
        load_chunk(&mut control.world, actions, chunk_idx);
    }
}

fn position_to_chunk_idx(position: Point3<f32>) -> [isize;2]{
    let chunk_i = (position.x + entities::CHUNK_SIZE/2.) / (entities::CHUNK_SIZE);
    let chunk_j = (position.z + entities::CHUNK_SIZE/2.) / (entities::CHUNK_SIZE);
    [chunk_i.floor() as isize, chunk_j.floor() as isize]
}

fn load_chunk(world: &mut GameScene, actions: &mut ViewActions, chunk_id: [isize;2]){
    let chunk_x = chunk_id[0] as f32 * entities::CHUNK_SIZE;
    let chunk_z = chunk_id[1] as f32 * entities::CHUNK_SIZE;
    let chunk_pos = Vector3::from([chunk_x, 0., chunk_z]);
    let grid = create_chunk_mat_at(chunk_pos);
    entities::Chunk::create(world, actions, chunk_pos, grid);
}

fn chunk_is_loaded(world: &mut GameScene, position: Point3<f32>) -> bool{
    let mut chunk_already_loaded = false;

    let pos_iter = world.get_iter::<PositionComponent>();
    for pos in pos_iter{
        if let Some(chunk_pos) = pos {
            let x = position.x;
            let z = position.z;
            let x1 = chunk_pos.value.0.x - entities::CHUNK_SIZE/2.;
            let x2 = chunk_pos.value.0.x + entities::CHUNK_SIZE/2.;
            let z1 = chunk_pos.value.0.z - entities::CHUNK_SIZE/2.;
            let z2 = chunk_pos.value.0.z + entities::CHUNK_SIZE/2.;
            if x > x1 && x < x2 && z > z1 && z < z2 {
                chunk_already_loaded = true;
            }
        }
    }

    return chunk_already_loaded;
}

type Mat3 = Vec<Vec<Vec<bool>>>;
use super::entities::{CHUNK_SIZE, GRID_SIZE, CUBE_SIZE};
use perlin2d::PerlinNoise2D;
pub fn create_chunk_mat_at(postion: Vector3<f32>) -> Mat3{
    let mut mat:Mat3 = vec![];
    let perlin = PerlinNoise2D::new(
        1, 
        1., 
        1.0, 
        0.5, 
        2.0, 
        (10.0, 10.0), 
        0., 
        1
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