use shred::{Read, World};
use voxelviewer::view_actions::ViewActions;
use world::components::*;
use std::time::Duration;
use super::Control;

pub fn render_fps_system(
    world: &mut Control,
    actions: &mut ViewActions,
    dt: Duration,
) {
    world.total_time += dt;
    if world.total_time.as_secs_f32() > 0.10{
        world.total_time = Duration::new(0, 0);
        let fps_text = format!("FPS: {}", 1./dt.as_secs_f32());
        let fps_id = world.texts_ids.get("fps").unwrap();
        actions.update_text(*fps_id, fps_text);

        let looking_dir = actions.state.camera.get_looking_dir();
        let looking_text = format!("Looking XZ: ({}, {})", looking_dir[0], looking_dir[1]);
        let looking_id = world.texts_ids.get("looking").unwrap();
        actions.update_text(*looking_id, looking_text);

        let pos = actions.state.camera.get_position();
        let text = format!("Position XYZ: ({:.0}, {:.0}, {:.0})", pos[0], pos[1], pos[2]);
        let id = world.texts_ids.get("position").unwrap();
        actions.update_text(*id, text);

        let count = actions.get_vertex_count();
        let text = format!("Vextex count: {}", count);
        let id = world.texts_ids.get("vertices").unwrap();
        actions.update_text(*id, text);
    }
}


pub fn render_system(
    components: &mut World,
    view: &mut ViewActions
) {
    let system_data: (
        Read<ComponentMap<RenderComponent>>,
    ) = components.system_data();

    let renders = system_data;
    
    let data_iter = renders.0.data().into_iter();
    for render in data_iter {
        match render {
            Some(render) =>{
                view.update_cube(&render.value.obj);
            },
            _ => {}
        }
    }
}