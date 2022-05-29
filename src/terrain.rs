

use super::Control;
use super::voxelviewer::ViewActions;
use shred::{Read};
use world::components::*;
use super::entities;
use cgmath::Vector3;

pub fn terrain_system(control: &mut Control, actions: &mut ViewActions){
    let camera_pos = actions.state.camera.position;
    let mut chunk_already_loaded = false;
    
    {
        let system_data: (
            Read<ComponentMap<PositionComponent>>,
        ) = control.world.components.system_data();

        let positons = system_data;
    
        let data_iter = positons.0.data().into_iter();
        for pos in data_iter {
            if let Some(chunk_pos) = pos {
                let x = camera_pos.x;
                let z = camera_pos.z;
                let x1 = chunk_pos.value.0.x - entities::CHUNK_SIZE/2.;
                let x2 = chunk_pos.value.0.x + entities::CHUNK_SIZE/2.;
                let z1 = chunk_pos.value.0.z - entities::CHUNK_SIZE/2.;
                let z2 = chunk_pos.value.0.z + entities::CHUNK_SIZE/2.;
                if x > x1 && x < x2 && z > z1 && z < z2 {
                    chunk_already_loaded = true;
                }
            }
        }
    }

    if !chunk_already_loaded{
        let chunk_i = (camera_pos.x + entities::CHUNK_SIZE/2.) / (entities::CHUNK_SIZE);
        let chunk_j = (camera_pos.z + entities::CHUNK_SIZE/2.) / (entities::CHUNK_SIZE);
        
        let chunk_x = chunk_i.floor() * entities::CHUNK_SIZE;
        let chunk_z = chunk_j.floor() * entities::CHUNK_SIZE;
        let chunk_pos = Vector3::new(chunk_x, 0., chunk_z);
        let grid = entities::Chunk::create_chunk_mat_at(chunk_pos);
        entities::Chunk::create(&mut control.world, actions, chunk_pos, grid);
    }

}