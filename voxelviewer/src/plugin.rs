use std::sync::{Mutex, Arc};
use common::{PositionComponent, VelocityComponent, AddRigidBodyCubeFlag};
use nalgebra::{Point3, Vector3};
use window::WindowResizeBuffer;

use plugins::{Plugin, PluginSytem, App};
use specs::{System, WorldExt, Read, WriteExpect, Builder};
use winit::{window::Window};
use crate::{scene::State, view_system::{ViewSystem, components::{MeshRendererComponent, LookingDirectionComponent}, UpdateViewMeshesSystem, camera_system::{CameraSystem, CameraResource}}, screen_text::ScreenText};

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

impl PluginSytem<'_> for CameraSystem {
    fn name(&self) -> &'static str {
        "camera_sync_system"
    }
}

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&mut self, app: &mut App) {
        app.add_component_storage::<LookingDirectionComponent>();

        let camera = app.world
            .create_entity()
            .with(PositionComponent::new(Point3::new(0.0, 10.0, 0.0)))
            .with(LookingDirectionComponent::new(0.,0.))
            .with(VelocityComponent(Vector3::new(0.0, 0.0, 0.0)))
            .with(AddRigidBodyCubeFlag(1.))
            .build() 
        ;
        app.world.insert(CameraResource::new(camera));
        
        app.add_system(CameraSystem::new());
    }
}

pub struct VoxelPlugin;
impl Plugin for VoxelPlugin {
    fn build(&mut self, app: &mut App) {
        app.add_system(UpdateViewMeshesSystem);
        app.add_system_thread_local(ResizeScreenSystem);
        app.add_system_thread_local(ViewSystem);
        app.with(CameraPlugin);

        app.add_component_storage::<MeshRendererComponent>();
        app.add_resource(Vec::<ScreenText>::new());

        let win = app.world.read_resource::<Arc<Window>>();
        
        let state =  pollster::block_on(State::new(&win));
        drop(win);

        let m = Mutex::new(state);
        app.add_resource(m);
    }
}


