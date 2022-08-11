use common::VelocityComponent;
use nalgebra::{Vector3, UnitComplex};
use specs::prelude::*;
use voxelviewer::{view_system::{camera_system::CameraResource, components::LookingDirectionComponent, resources::DeviceEventBuffer}, camera::CameraController};
use winit::event::{DeviceEvent, KeyboardInput, VirtualKeyCode, ElementState};

pub struct IoSystem{
    camera_controller: CameraController,
}


impl<'a> System<'a> for IoSystem{
    type SystemData = (
        Read<'a, DeviceEventBuffer>,
        ReadExpect<'a, CameraResource>,

        WriteStorage<'a, VelocityComponent>,
        WriteStorage<'a, LookingDirectionComponent>,
    );
 
    
    fn run(&mut self, (evnts, camera, mut vel_s, mut ldc_strg): Self::SystemData){
        let p = vel_s.get_mut(camera.entity).unwrap();
        let ldc = ldc_strg.get_mut(camera.entity).unwrap();

        for evnt in &evnts.events {
            self.process_event(evnt);
        }

        self.update_camera_dir_vel(p, ldc);
        if self.camera_controller.mouse_pressed {
            self.update_camera_rotation(ldc);
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
            },
            DeviceEvent::Button {
                button: 1, // Left Mouse Button
                state,
            } => {
                self.camera_controller.mouse_pressed = *state == ElementState::Pressed;
            }
            _ => {}
        }
    }

    pub fn update_camera_dir_vel(&mut self, p: &mut VelocityComponent, ldc: &mut LookingDirectionComponent){
        let mut direction = Vector3::new(0., 0., 0.);
        let look_vec = Vector3::new(
            ldc.yaw.cos_angle(),
            0.,
            ldc.yaw.sin_angle()
        );
        let mut vel = 5.;
        if self.camera_controller.pressed_keys.contains(&VirtualKeyCode::D){
            let look_x = look_vec.x;
            direction.x += -look_vec.z;
            direction.z += look_x;
        }
        if self.camera_controller.pressed_keys.contains(&VirtualKeyCode::A){
            let look_x = look_vec.x;
            direction.x += look_vec.z;
            direction.z += -look_x;
        }
        if self.camera_controller.pressed_keys.contains(&VirtualKeyCode::W){
            direction += look_vec;
        }
        if self.camera_controller.pressed_keys.contains(&VirtualKeyCode::S){
            direction -= look_vec;
        }

        if self.camera_controller.pressed_keys.contains(&VirtualKeyCode::LShift){
            // direction.y -= 1.;
            vel *= 2.;
        }
        if self.camera_controller.pressed_keys.contains(&VirtualKeyCode::Space){
            p.0.y = vel;
        }
        let result = direction * vel;
        p.0.x = result.x;
        p.0.z = result.z;
    }

    fn update_camera_rotation(&mut self, ldc: &mut LookingDirectionComponent){
        ldc.yaw = UnitComplex::new(ldc.yaw.angle() + (self.camera_controller.rotate_horizontal) * 0.004);
        self.camera_controller.rotate_horizontal = 0.;

        ldc.pitch = UnitComplex::new(ldc.pitch.angle() - (self.camera_controller.rotate_vertical) * 0.004);
        self.camera_controller.rotate_vertical = 0.;

        if ldc.pitch.angle() < UnitComplex::new(-std::f32::consts::FRAC_PI_2).angle() {
            ldc.pitch = UnitComplex::new(-std::f32::consts::FRAC_PI_2);
        }
        if ldc.pitch.angle() > UnitComplex::new(std::f32::consts::FRAC_PI_2).angle() {
            ldc.pitch = UnitComplex::new(std::f32::consts::FRAC_PI_2);
        }
    }
}
