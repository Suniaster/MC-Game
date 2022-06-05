// use crate::components::{RenderComponent, PositionComponent, GridDescriptorComponent};

// use std::time::Duration;
// use voxelviewer::view_actions::ViewActions;
// use world::ComponentStorage;

// type CS<T> = ComponentStorage<T>;

// pub fn render_fps_system(world: &mut Control, actions: &mut ViewActions, dt: Duration) {
//     world.total_time += dt;
//     if world.total_time.as_secs_f32() > 0.10 {
//         world.total_time = Duration::new(0, 0);
//         let fps_text = format!("FPS: {}", 1. / dt.as_secs_f32());
//         let fps_id = world.texts_ids.get("fps").unwrap();
//         actions.update_text(*fps_id, fps_text);

//         let looking_dir = actions.state.camera.get_looking_dir();
//         let looking_text = format!("Looking XZ: ({}, {})", looking_dir[0], looking_dir[1]);
//         let looking_id = world.texts_ids.get("looking").unwrap();
//         actions.update_text(*looking_id, looking_text);

//         let pos = actions.state.camera.get_position();
//         let text = format!(
//             "Position XYZ: ({:.0}, {:.0}, {:.0})",
//             pos[0], pos[1], pos[2]
//         );
//         let id = world.texts_ids.get("position").unwrap();
//         actions.update_text(*id, text);

//         let count = actions.get_vertex_count();
//         let text = format!("Vextex count: {}", count);
//         let id = world.texts_ids.get("vertices").unwrap();
//         actions.update_text(*id, text);
//     }
// }

// pub fn render_terrain_system(
//     renders: &mut CS<RenderComponent>, 
//     positions: &CS<PositionComponent>, 
//     grids: &CS<GridDescriptorComponent>,
//     actions: &mut ViewActions,
// ){

//     for render in renders.iter(){
//         let pos = positions.get(render.0);
//         let grid = grids.get(render.0);

//         if let (Some(pos), Some(grid)) = (pos, grid) {
//             actions.create_grid(pos.0.into(), grid.cube_size, &grid.desc);
//         }
//     }
    
// }