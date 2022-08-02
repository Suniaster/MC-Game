use std::sync::{Mutex, Arc};
use window::WindowResizeBuffer;

use plugins::{Plugin, PluginSytem, App};
use specs::{System, WorldExt, Read, WriteExpect};
use winit::{window::Window};
use crate::{scene::State, view_system::{ViewSystem, components::MeshRendererComponent, UpdateViewMeshesSystem}, screen_text::ScreenText};

struct ResizeScreenSystem;
impl<'a> System<'a> for ResizeScreenSystem {
    type SystemData = (
        WriteExpect<'a, Mutex<State>>,
        Read<'a, WindowResizeBuffer>,
    );

    fn run(&mut self, (mutex_state, win_evs): Self::SystemData) {
        let mut state = mutex_state.lock().unwrap();
        for ev in win_evs.events.iter() {
            state.resize(*ev);
        }
    }

}

impl PluginSytem<'_> for ResizeScreenSystem {
    fn name(&self) -> &'static str {
        "voxel_viewer_system"
    }
}

pub struct VoxelPlugin;
impl Plugin for VoxelPlugin {
    fn build(&mut self, app: &mut App) {
        app.add_system(UpdateViewMeshesSystem);
        app.add_system_thread_local(ResizeScreenSystem);
        app.add_system_thread_local(ViewSystem);

        app.add_component_storage::<MeshRendererComponent>();
        app.add_resource(Vec::<ScreenText>::new());

        let win = app.world.read_resource::<Arc<Window>>();
        
        let state =  pollster::block_on(State::new(&win));
        drop(win);

        let m = Mutex::new(state);
        app.add_resource(m);
    }
}


