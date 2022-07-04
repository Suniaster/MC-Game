use nalgebra::{Vector3, Rotation3};
use specs::prelude::*;
use voxelviewer::{view_system::{camera_system::CameraResource, components::LookingDirectionComponent, resources::DeviceEventBuffer}, camera::CameraController};
use winit::event::{DeviceEvent, KeyboardInput};

use crate::components::PhysicsComponent;



pub struct IoSystem{
    camera_controller: CameraController,
}


impl<'a> System<'a> for IoSystem{
    type SystemData = (
        Read<'a, DeviceEventBuffer>,
        ReadExpect<'a, CameraResource>,

        WriteStorage<'a, PhysicsComponent>,
        WriteStorage<'a, LookingDirectionComponent>,
    );
 
    
    fn run(&mut self, (evnts, camera, mut phys_strg, mut ldc_strg): Self::SystemData){
        let p = phys_strg.get_mut(camera.entity).unwrap();
        let ldc = ldc_strg.get_mut(camera.entity).unwrap();

        for evnt in &evnts.events {
            self.process_event(evnt);
        }
    }
}

impl IoSystem{
    pub fn new() -> Self{
        IoSystem{
            camera_controller: CameraController::new(1., 1.),
        }
    }

    pub fn process_event(&mut self, evnt: &winit::event::DeviceEvent){
        match evnt {
            DeviceEvent::Key(
                KeyboardInput {
                    virtual_keycode: Some(key),
                    state,
                    ..
                }
            ) => {
                self.camera_controller.process_keyboard(*key, *state);
            },
            DeviceEvent::MouseMotion { delta } => {
                self.camera_controller.process_mouse(delta.0, delta.1);
            },
            DeviceEvent::MouseWheel { delta, .. } => {
                self.camera_controller.process_scroll(delta);
            }
            _ => {}
        }
    }

    pub fn update_camera(&mut self, p: &mut PhysicsComponent, ldc: &mut LookingDirectionComponent){
        let mut move_dir = Vector3::new(
            ldc.yaw.cos_angle(),
            ldc.pitch.sin_angle(),
            ldc.yaw.sin_angle()
        );

        move_dir = move_dir.normalize();

        let axis = Vector3::y_axis();
        let mut rot = Rotation3::from_axis_angle(&axis, 0);
        
        let mut final_vel = Vector3::new(0.,0.,0.);
        if self.camera_controller.amount_left == 1. {
            
        }
        if self.camera_controller.amount_right == 1. {
            final_vel += Vector3::new(1., 0., 0.);
        }
        if self.camera_controller.amount_forward == 1.{
            final_vel += Vector3::new(0., 0., -1.);
        }
        if self.camera_controller.amount_backward == 1.{
            final_vel += Vector3::new(0., 0., 1.);
        }
        if self.camera_controller.amount_up == 1.{
            final_vel += Vector3::new(0., 1., 0.);
        }
        if self.camera_controller.amount_down == 1.{
            final_vel += Vector3::new(0., -1., 0.);
        }

        
    }
}