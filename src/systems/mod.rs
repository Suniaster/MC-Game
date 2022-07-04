use std::{time::Duration};

pub mod physics;
pub mod io;

use specs::prelude::*;
use voxelviewer::{screen_text::ScreenText, view_system::{camera_system::CameraResource, components::{PositionComponent, LookingDirectionComponent}}};

pub struct RenderTextInfoSystem{
    pub time_counter: std::time::Duration,
}

impl RenderTextInfoSystem {
    pub fn new() -> Self {
        RenderTextInfoSystem{
            time_counter: Duration::new(0, 0),
        }
    }
}

impl<'a> System<'a> for RenderTextInfoSystem {
    type SystemData = (
        Write<'a, Vec<ScreenText>>,
        Read<'a, WorldDt>,
        ReadExpect<'a, CameraResource>,

        ReadStorage<'a, PositionComponent>,
        ReadStorage<'a, LookingDirectionComponent>,
    );

    fn setup(&mut self, world: &mut specs::World) {
        Self::SystemData::setup(world);
        let mut screen_texts :Vec<ScreenText> = vec![];
        screen_texts.push(ScreenText::new(
            String::from(""),
            0.0,
            0.0,
            [0.0, 0.0, 0.0, 1.0],
        ));

        screen_texts.push(ScreenText::new(
            String::from(""),
            0.0,
            20.0,
            [0.0, 0.0, 0.0, 1.0],
        ));

        screen_texts.push(ScreenText::new(
            String::from(""),
            0.0,
            40.0,
            [0.0, 0.0, 0.0, 1.0],
        ));

        world.insert::<Vec<ScreenText>>(screen_texts);
    }

    fn run(&mut self, (mut texts, dt, camera, pos_storage, ldc): Self::SystemData){

        self.time_counter += dt.0;
        if self.time_counter.as_secs_f32() > 0.1 {
            self.time_counter = Duration::new(0, 0);


            texts[0].text = format!("FPS: {:.1}", 1.0 / dt.0.as_secs_f32());

            let camera_pos = pos_storage.get(camera.entity);
            if let Some(camera_pos) = camera_pos {
                texts[1].text = format!("Position (X,Y,Z): ({:.1}, {:.1}, {:.1})", camera_pos.0.x, camera_pos.0.y, camera_pos.0.z);
            }

            let camera_dir = ldc.get(camera.entity);
            if let Some(camera_dir) = camera_dir {
                texts[2].text = format!("Looking direction (Yaw, Pitch): ({:.1}, {:.1})", camera_dir.yaw, camera_dir.pitch);
            }
        }

        //     let count = screen.actions.get_vertex_count();
        //     let count_text = format!("Vertex count: {}", count);
        //     let id = self.texts_ids.get("vertices").unwrap();
        //     screen.actions.update_text(*id, count_text);
        // }
    }

  
}



#[derive(Default)]
pub struct WorldDt(pub Duration);
pub struct UpdateDtSystem{
    pub last_time: std::time::Instant
}
impl<'a> System<'a> for UpdateDtSystem {
    type SystemData = Write<'a, WorldDt>;

    fn run(&mut self, mut data: Self::SystemData) {
        let now = std::time::Instant::now();
        let dt = now.duration_since(self.last_time);
        self.last_time = now;
        data.0 = dt;
    }
}