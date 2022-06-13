use std::{collections::HashMap, time::Duration};

use specs::prelude::*;
use voxelviewer::screen_text::ScreenText;

pub struct RenderTextInfoSystem{
    pub texts_ids: HashMap<&'static str, usize>,
    pub time_counter: std::time::Duration,
}

impl RenderTextInfoSystem {
    pub fn new() -> Self {
        RenderTextInfoSystem{
            texts_ids: HashMap::new(),
            time_counter: Duration::new(0, 0),
        }
    }
}

impl<'a> System<'a> for RenderTextInfoSystem {
    type SystemData = (
        Write<'a, Vec<ScreenText>>,
        Read<'a, WorldDt>,
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
        self.texts_ids.insert("fps", screen_texts.len() - 1);
        world.insert::<Vec<ScreenText>>(screen_texts);
    }

    fn run(&mut self, (mut texts, dt): Self::SystemData){

        self.time_counter += dt.0;
        if self.time_counter.as_secs_f32() > 0.1 {
            self.time_counter = Duration::new(0, 0);


            let fps_ent = self.texts_ids.get("fps").unwrap();
            texts[*fps_ent].text = format!("FPS: {:.1}", 1.0 / dt.0.as_secs_f32());

        }
        //     let mut screen = scren_mutex.lock().unwrap();
            
        //     let fps = 1./ dt.0.as_secs_f32();
        //     let text = format!("Vextex count: {}", fps);
        //     let id = self.texts_ids.get("fps").unwrap();
        //     screen.actions.update_text(*id, text);

        //     let looking_dir = screen.actions.state.camera.get_looking_dir();
        //     let looking_text = format!("Looking XZ: ({}, {})", looking_dir[0], looking_dir[1]);
        //     let id = self.texts_ids.get("looking").unwrap();
        //     screen.actions.update_text(*id, looking_text);

        //     let position = screen.actions.state.camera.get_position();
        //     let position_text = format!(
        //         "Position: ({:.0}, {:.0}, {:.0})", 
        //         position.x, position.y, position.z
        //     );
        //     let id = self.texts_ids.get("position").unwrap();
        //     screen.actions.update_text(*id, position_text);

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