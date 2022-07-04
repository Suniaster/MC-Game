use winit::event::*;
use winit::dpi::PhysicalPosition;
use std::time::Duration;

use nalgebra::{Matrix4, Point3, Vector3, UnitComplex};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

#[derive(Debug)]
pub struct Camera {
    pub position: Point3<f32>,
    pub yaw: UnitComplex<f32>,
    pub pitch: UnitComplex<f32>,
}

impl Camera {
    pub fn new(
        position: Point3<f32>,
        yaw: UnitComplex<f32>,
        pitch: UnitComplex<f32>,
    ) -> Self {
        Self {
            position: position,
            yaw: yaw,
            pitch: pitch,
        }
    }

    pub fn calc_matrix(&self) -> Matrix4<f32> {
        let dir = Vector3::new(
            self.yaw.cos_angle(),
            self.pitch.sin_angle(),
            self.yaw.sin_angle()
        );
        let target = self.position + dir;
        Matrix4::look_at_rh(
            &self.position, 
            &target,
            &Vector3::y()
        )
    }

    pub fn get_looking_dir(&self) -> [f32;2]{
        let mut dir = [0.0, 0.0];
        dir[0] = self.yaw.cos_angle();
        dir[1] = self.yaw.sin_angle();
        dir
    }

    pub fn get_position(&self) -> Point3<f32>{
        self.position
    }

}

pub struct Projection {
  aspect: f32,
  fovy: UnitComplex<f32>,
  znear: f32,
  zfar: f32,
}

impl Projection {
  pub fn new(
      width: u32,
      height: u32,
      fovy: UnitComplex<f32>,
      znear: f32,
      zfar: f32,
  ) -> Self {
      Self {
          aspect: width as f32 / height as f32,
          fovy: fovy,
          znear,
          zfar,
      }
  }

  pub fn resize(&mut self, width: u32, height: u32) {
      self.aspect = width as f32 / height as f32;
  }

  pub fn calc_matrix(&self) -> Matrix4<f32> {
        let mat = nalgebra::Perspective3::new(
            self.aspect,
            self.fovy.angle(),
          self.znear,
          self.zfar,
      );
      OPENGL_TO_WGPU_MATRIX * mat.as_matrix()
  }
}

#[derive(Debug)]
pub struct CameraController {
    pub amount_left: f32,
    pub amount_right: f32,
    pub amount_forward: f32,
    pub amount_backward: f32,
    pub amount_up: f32,
    pub amount_down: f32,
    pub rotate_horizontal: f32,
    pub rotate_vertical: f32,
    pub scroll: f32,
    pub speed: f32,
    pub sensitivity: f32,
}

impl CameraController {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            amount_left: 0.0,
            amount_right: 0.0,
            amount_forward: 0.0,
            amount_backward: 0.0,
            amount_up: 0.0,
            amount_down: 0.0,
            rotate_horizontal: 0.0,
            rotate_vertical: 0.0,
            scroll: 0.0,
            speed,
            sensitivity,
        }
    }

    pub fn process_keyboard(&mut self, key: VirtualKeyCode, state: ElementState) -> bool{
        let amount = if state == ElementState::Pressed { 1.0 } else { 0.0 };
        match key {
            VirtualKeyCode::W | VirtualKeyCode::Up => {
                self.amount_forward = amount;
                true
            }
            VirtualKeyCode::S | VirtualKeyCode::Down => {
                self.amount_backward = amount;
                true
            }
            VirtualKeyCode::A | VirtualKeyCode::Left => {
                self.amount_left = amount;
                true
            }
            VirtualKeyCode::D | VirtualKeyCode::Right => {
                self.amount_right = amount;
                true
            }
            VirtualKeyCode::Space => {
                self.amount_up = amount;
                true
            }
            VirtualKeyCode::LShift => {
                self.amount_down = amount;
                true
            }
            _ => false,
        }
    }

    pub fn process_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self.rotate_horizontal = mouse_dx as f32 * 3.;
        self.rotate_vertical = mouse_dy as f32 * 3.;
    }

    pub fn process_scroll(&mut self, delta: &MouseScrollDelta) {
        self.scroll = -match delta {
            // I'm assuming a line is about 100 pixels
            MouseScrollDelta::LineDelta(_, scroll) => scroll * 100.0,
            MouseScrollDelta::PixelDelta(PhysicalPosition {
                y: scroll,
                ..
            }) => *scroll as f32,
        };
    }

    pub fn update_camera(&mut self, camera: &mut Camera, dt: Duration) {
        let dt = dt.as_secs_f32();

        // Move forward/backward and left/right
        let (yaw_sin, yaw_cos) = (camera.yaw.sin_angle(), camera.yaw.cos_angle());
        let forward = Vector3::new(yaw_cos, 0.0, yaw_sin).normalize();
        let right = Vector3::new(-yaw_sin, 0.0, yaw_cos).normalize();
        camera.position += forward * (self.amount_forward - self.amount_backward) * self.speed * dt;
        camera.position += right * (self.amount_right - self.amount_left) * self.speed * dt;

        // Move in/out (aka. "zoom")
        // Note: this isn't an actual zoom. The camera's position
        // changes when zooming. I've added this to make it easier
        // to get closer to an object you want to focus on.
        let (pitch_sin, pitch_cos) = (camera.pitch.sin_angle(), camera.pitch.cos_angle());
        let scrollward = Vector3::new(pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin).normalize();
        camera.position += scrollward * self.scroll * self.speed * self.sensitivity * dt;
        self.scroll = 0.0;

        // Move up/down. Since we don't use roll, we can just
        // modify the y coordinate directly.
        camera.position.y += (self.amount_up - self.amount_down) * self.speed * dt;

        // Rotate
        camera.yaw = UnitComplex::new(camera.yaw.angle() + (self.rotate_horizontal * self.sensitivity * dt));
        camera.pitch = UnitComplex::new(camera.pitch.angle() + (-self.rotate_vertical * self.sensitivity * dt));

        // If process_mouse isn't called every frame, these values
        // will not get set to zero, and the camera will rotate
        // when moving in a non cardinal direction.
        self.rotate_horizontal = 0.0;
        self.rotate_vertical = 0.0;

        // Keep the camera's angle from going too high/low.
        // if camera.pitch < -Rad(SAFE_FRAC_PI_2) {
        //     camera.pitch = -Rad(SAFE_FRAC_PI_2);
        // } else if camera.pitch > Rad(SAFE_FRAC_PI_2) {
        //     camera.pitch = Rad(SAFE_FRAC_PI_2);
        // }
    }
}
