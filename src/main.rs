use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use voxelviewer::{ScreenView};
use specs::prelude::*;

mod entities;
mod systems;
mod terrain;
mod components;

// Proximos Objetivos
// x - Adicionar bordas (linhas) nos cubos - LineStrip = 2,
// X - Descobrir como botar texto na tela - https://github.com/hecrj/wgpu_glyph
// X - Escrever na tela fps
// X - e direção olhando
// X - Otmizar o uso de vertices no grid (remover repetidos)
// X - Adicionar geração de terreno
// X - Melhorar geração de terreno
// X - Descobrir porque as paradas que tão longe somem
// X - Concertar o treco de cima
// X - Mudar no front uso de cgmath para nalgebra
// - Melhorar interface com front
// - Adicionar luz no céu
// - Melhorar uso de entidades na GameScene
// - Deixar padronizado chunk size no back e no front
// - Melhorar iteração por componentes no GameScene
// - Melhorar interface de comunicação entre back e front

// - Ajustar cores (não tão sendo efetivamente usados)
// - Descobrir como fazer um tipo de animação de entrada
// - Descobrir como rotacionar cubos e grids
// - Tentar fazer algum modelo no MagicaVoxel
// - Tentar importar modelo pelo back e passar para o front renderiza-lo
// - Adicionar menu para mudanças de propriedades da camera

use specs::DispatcherBuilder;

struct RenderTextInfoSystem{
    texts_ids: HashMap<&'static str, usize>,
    time_counter: std::time::Duration,
}

impl<'a> System<'a> for RenderTextInfoSystem {
    type SystemData = (
        ReadExpect<'a, Arc<Mutex<ScreenView>>>,
        Read<'a, WorldDt>,
    );

    fn run(&mut self, (scren_mutex, dt): Self::SystemData){
        self.time_counter += dt.0;
        if self.time_counter.as_secs_f32() > 0.1 {
            self.time_counter = Duration::new(0, 0);
            let mut screen = scren_mutex.lock().unwrap();
            
            let fps = 1./ dt.0.as_secs_f32();
            let text = format!("Vextex count: {}", fps);
            let id = self.texts_ids.get("fps").unwrap();
            screen.actions.update_text(*id, text);

            let looking_dir = screen.actions.state.camera.get_looking_dir();
            let looking_text = format!("Looking XZ: ({}, {})", looking_dir[0], looking_dir[1]);
            let id = self.texts_ids.get("looking").unwrap();
            screen.actions.update_text(*id, looking_text);

            let position = screen.actions.state.camera.get_position();
            let position_text = format!(
                "Position: ({:.0}, {:.0}, {:.0})", 
                position.x, position.y, position.z
            );
            let id = self.texts_ids.get("position").unwrap();
            screen.actions.update_text(*id, position_text);

            let count = screen.actions.get_vertex_count();
            let count_text = format!("Vertex count: {}", count);
            let id = self.texts_ids.get("vertices").unwrap();
            screen.actions.update_text(*id, count_text);
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        let screen_mutex = world.read_resource::<Arc<Mutex<ScreenView>>>();
        let mut screen = screen_mutex.lock().unwrap();

        self.texts_ids.insert("fps", screen.actions.create_text());
        self.texts_ids.insert("looking", screen.actions.create_text());
        self.texts_ids.insert("position", screen.actions.create_text());
        self.texts_ids.insert("vertices", screen.actions.create_text());
    }
}

#[derive(Default)]
struct WorldDt(Duration);
struct UpdateDtSystem{
    last_time: std::time::Instant
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

fn main() {
    let mut world = World::new();

    world.register::<components::RenderComponent>();
    world.register::<components::PositionComponent>();
    world.register::<components::GridDescriptorComponent>();

    let (screen, evloop) = voxelviewer::create_screen();
    let arc_screen = Arc::new(Mutex::new(screen));

    let mut dispatcher = DispatcherBuilder::new()
        .with(UpdateDtSystem{
            last_time: std::time::Instant::now()
        }, "update_dt_system", &[])
        .with_thread_local(RenderTextInfoSystem{
            texts_ids: HashMap::new(),
            time_counter: std::time::Duration::new(0, 0),
        })
        .build();

    world.insert(arc_screen.clone());
    world.insert(WorldDt(Duration::new(0, 0)));

    dispatcher.setup(&mut world);
    voxelviewer::start(world, dispatcher, arc_screen, evloop);
}
