use specs::prelude::*;

use common::PositionComponent;
use nalgebra::Point3;
use plugins::PluginSytem;

pub struct TerrainSystem;

impl PluginSytem<'_> for TerrainSystem {
    fn name(&self) -> &'static str {
        "TerrainSystem"
    }
}

impl <'a> System<'a> for TerrainSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, MeshRendererComponent>,
        WriteStorage<'a, PositionComponent>,
        WriteExpect<'a, LoadedChunks>
    );

    fn run(&mut self, (entities, mut mr, mut ps, mut loaded_chunks): Self::SystemData) {
        let camera_pos = Point3::origin();

        let camera_chunk = position_to_chunk_idx(camera_pos);

        let ids_surround = get_chunks_ids_surround_id(camera_chunk);
    
        let unloaded_ids = get_unloaded_chunks(
            &loaded_chunks, 
            ids_surround
        );

        for chunk_id in unloaded_ids {            
            let chunk_x = chunk_id[0] as f32 * CHUNK_SIZE;
            let chunk_z = chunk_id[1] as f32 * CHUNK_SIZE;
            let chunk_pos = Point3::from([chunk_x, 0., chunk_z]);
            let grid = create_chunk_mat_at(chunk_pos);
        
            // Insert
            loaded_chunks
                .chunks
                .insert(chunk_id[1] * GRID_SIZE as isize + chunk_id[0], ());
                
            let chunk = entities.create();
            let random_color:[f32;3] = [
                rand::random::<f32>(),
                rand::random::<f32>(),
                rand::random::<f32>(),
            ];

            let cube_tensor = CubeTensor::new(grid, CUBE_SIZE, [GRID_SIZE, CHUNK_HEIGHT, GRID_SIZE]);
            mr.insert(chunk, MeshRendererComponent::from_grid(random_color, &cube_tensor)).unwrap();
            ps.insert(chunk, PositionComponent::new(chunk_pos)).unwrap();
        }
    }
}

fn get_unloaded_chunks(loaded_chunks: &LoadedChunks, ids: Vec<[isize; 2]>) -> Vec<[isize; 2]> {
    let mut unloaded_chunks = vec![];
    for id in ids {
        let key = id[1] * GRID_SIZE as isize + id[0];
        let res = loaded_chunks.chunks.get(&key);
        if res.is_none() {
            unloaded_chunks.push(id);
        }
    }
    unloaded_chunks
}

fn get_chunks_ids_surround_id(id: [isize; 2]) -> Vec<[isize; 2]> {
    let mut chunks_ids = Vec::new();
    const VIEW_RANGE: isize = 8;
    for x in -VIEW_RANGE..VIEW_RANGE {
        for y in -VIEW_RANGE..VIEW_RANGE {
            chunks_ids.push([id[0] + x, id[1] + y]);
        }
    }
    chunks_ids
}

fn position_to_chunk_idx(position: Point3<f32>) -> [isize; 2] {
    let chunk_i = (position.x + CHUNK_SIZE / 2.) / (CHUNK_SIZE);
    let chunk_j = (position.z + CHUNK_SIZE / 2.) / (CHUNK_SIZE);
    [chunk_i.floor() as isize, chunk_j.floor() as isize]
}


type Mat3 = Vec<Vec<Vec<bool>>>;
use perlin2d::PerlinNoise2D;
use voxelviewer::{view_system::components::MeshRendererComponent, geometry::grid::CubeTensor};

use super::{LoadedChunks, CHUNK_SIZE, GRID_SIZE, CUBE_SIZE, CHUNK_HEIGHT};

fn create_chunk_mat_at(postion: Point3<f32>) -> Mat3 {
    let mut mat: Mat3 = vec![vec![vec![false; GRID_SIZE]; CHUNK_HEIGHT]; GRID_SIZE];
    let perlin_n = PerlinNoise2D::new(
        6, 
        10.0, 
        0.5, 
        1.0,
        2.0,
        (100. as f64, 100. as f64), 
        10.,
        101
    );

    let correct_x = postion.x - CHUNK_SIZE / 2. + CUBE_SIZE / 2.;
    let correct_z = postion.z - CHUNK_SIZE / 2. + CUBE_SIZE / 2.;
    let correct_y = postion.y - CHUNK_SIZE / 2. + CUBE_SIZE / 2.;

    for i in 0..GRID_SIZE {
        for k in 0..GRID_SIZE {
            let pos_x = correct_x + (i as f32 * CUBE_SIZE);
            let pos_z = correct_z + (k as f32 * CUBE_SIZE);
            let height = perlin_n.get_noise(pos_x as f64, pos_z as f64);

            for j in 0..CHUNK_HEIGHT {
                let pos_y = correct_y + (j as f32 * CUBE_SIZE);
                mat[i][j][k] = height > pos_y.into();
            }
        }
    }
    return mat;
}
