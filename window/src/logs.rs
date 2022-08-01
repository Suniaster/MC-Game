use plugins::{Plugin, PluginSytem};
use specs::{System, Read};

use crate::{DeltaTime, FrameCount};

struct FpsLogSystem;
impl<'a> System<'a> for FpsLogSystem {
    type SystemData = (
        Read<'a, DeltaTime>,
        Read<'a, FrameCount>
    );

    fn run(&mut self, (dt, fc): Self::SystemData) {
        if fc.0 % 30 == 0 {
            let fps = 1.0 / dt.0;
            println!("FPS: {}", fps);
        }
    }
}

impl PluginSytem<'_> for FpsLogSystem {
    fn name(&self) -> &'static str {
        "fps_log_system"
    }
}

pub struct FpsLoggerPlugin;
impl Plugin for FpsLoggerPlugin {
    fn build(&mut self, _app: &mut plugins::App){
        _app.add_system(FpsLogSystem);
    }
}