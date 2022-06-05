// use std::collections::HashMap;

// use crate::components;

// use super::entities;
// use super::voxelviewer::view_actions::ViewActions;
// use nalgebra::Point3;
// use world::manager::WorldManager;

// pub struct LoadedChunks {
//     chunks: HashMap<isize, ()>,
// }
// impl LoadedChunks { pub fn new() -> Self { LoadedChunks { chunks: HashMap::new() } } }

// pub const GRID_SIZE: usize = 16;
// pub const CUBE_SIZE: f32 = 1.;
// pub const CHUNK_SIZE: f32 = GRID_SIZE as f32 * CUBE_SIZE;

// pub fn terrain_system(manager: &mut WorldManager, camera_pos: Point3<f32>) {
//     // let camera_pos = actions.state.camera.position;
//     let camera_chunk = position_to_chunk_idx(Point3::new(camera_pos.x, camera_pos.y, camera_pos.z));

//     // Olhar todos os id's de chunks ao redor da camera
//     let ids_surround = get_chunks_ids_surround_id(camera_chunk);

//     // Ver quais chunks não foram carregados
//     let unloaded_ids = get_unloaded_chunks(
//         manager.world.resource_ref::<LoadedChunks>().unwrap(), 
//         ids_surround
//     );

//     // Carregar chunks
//     for id in unloaded_ids {
//         load_chunk(manager, id);
//     }

//     // Descarregar chunks que não estão ao redor da camera
//     // TODO:
// }

// fn get_unloaded_chunks(loaded_chunks: &LoadedChunks, ids: Vec<[isize; 2]>) -> Vec<[isize; 2]> {
//     let mut unloaded_chunks = vec![];
//     for id in ids {
//         let key = id[0] * GRID_SIZE as isize + id[1];
//         let res = loaded_chunks.chunks.get(&key);
//         if res.is_none() {
//             unloaded_chunks.push(id);
//         }
//     }
//     unloaded_chunks
// }

// fn get_chunks_ids_surround_id(id: [isize; 2]) -> Vec<[isize; 2]> {
//     let mut chunks_ids = Vec::new();
//     const VIEW_RANGE: isize = 8;
//     for x in -VIEW_RANGE..VIEW_RANGE {
//         for y in -VIEW_RANGE..VIEW_RANGE {
//             chunks_ids.push([id[0] + x, id[1] + y]);
//         }
//     }
//     chunks_ids
// }

// fn position_to_chunk_idx(position: Point3<f32>) -> [isize; 2] {
//     let chunk_i = (position.x + CHUNK_SIZE / 2.) / (CHUNK_SIZE);
//     let chunk_j = (position.z + CHUNK_SIZE / 2.) / (CHUNK_SIZE);
//     [chunk_i.floor() as isize, chunk_j.floor() as isize]
// }

// fn load_chunk(manager: &mut world::manager::WorldManager, chunk_id: [isize; 2]) {
//     let chunk_x = chunk_id[0] as f32 * CHUNK_SIZE;
//     let chunk_z = chunk_id[1] as f32 * CHUNK_SIZE;
//     let chunk_pos = Point3::from([chunk_x, 0., chunk_z]);
//     let grid = create_chunk_mat_at(chunk_pos);

//     // Insert
//     let loaded_chunks = manager.world.resource_mut::<LoadedChunks>().unwrap();
//     loaded_chunks
//         .chunks
//         .insert(chunk_id[0] * GRID_SIZE as isize + chunk_id[1], ());

//     create_chunk(manager, chunk_pos, grid);
// }

// type Mat3 = Vec<Vec<Vec<bool>>>;
// use perlin2d::PerlinNoise2D;
// fn create_chunk_mat_at(postion: Point3<f32>) -> Mat3 {
//     let mut mat: Mat3 = vec![];
//     let perlin = PerlinNoise2D::new(1, 1., 1.0, 0.5, 2.0, (10.0, 10.0), 0., 1);

//     for i in 0..GRID_SIZE {
//         mat.push(vec![]);
//         for j in 0..GRID_SIZE {
//             mat[i].push(vec![]);
//             for k in 0..GRID_SIZE {
//                 let pos_x = postion.x + i as f32 * CUBE_SIZE - CHUNK_SIZE / 2.;
//                 let pos_y = postion.y + j as f32 * CUBE_SIZE - CHUNK_SIZE / 2.;
//                 let pos_z = postion.z + k as f32 * CUBE_SIZE - CHUNK_SIZE / 2.;

//                 let mut val = perlin.get_noise(pos_x as f64, pos_z as f64);

//                 val += 1.;
//                 val /= 2.;
//                 val *= 7.;

//                 mat[i][j].push(val > pos_y.into());
//             }
//         }
//     }
//     return mat;
// }

// fn create_chunk(
//     scene: &mut WorldManager,
//     position: Point3<f32>,
//     grid: Mat3,
// ) {
//     scene.build_entity()
//         .with(components::RenderComponent {
//             is_rendered: true,
//             obj_id: 0,
//         })
//         .with(components::PositionComponent(position))
//         .with(components::GridDescriptorComponent{
//             desc: grid,
//             cube_size: CUBE_SIZE,
//         })
//         .build();
// }