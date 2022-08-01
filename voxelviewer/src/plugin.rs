use std::sync::{Mutex, Arc};
use window::WindowResizeBuffer;

use plugins::{Plugin, PluginSytem, App};
use specs::{System, WorldExt, Read, WriteExpect};
use winit::{window::Window};
use crate::scene::State;

struct VoxelViewerSystem;
impl<'a> System<'a> for VoxelViewerSystem {
    type SystemData = (
        WriteExpect<'a, Mutex<State>>,
        Read<'a, WindowResizeBuffer>,
    );

    fn run(&mut self, (mutex_state, win_evs): Self::SystemData) {
        let mut state = mutex_state.lock().unwrap();
        for ev in win_evs.events.iter() {
            state.resize(*ev);
        }
        drop(state);
    }

}

impl PluginSytem<'_> for VoxelViewerSystem {
    fn name(&self) -> &'static str {
        "voxel_viewer_system"
    }
}

pub struct VoxelPlugin;
impl Plugin for VoxelPlugin {
    fn build(&mut self, app: &mut App) {
        app.add_system(VoxelViewerSystem);
        let win = app.world.read_resource::<Arc<Window>>();
        
        let state =  pollster::block_on(State::new(&win));
        drop(win);

        let m = Mutex::new(state);
        app.add_resource(m);
    }
}


