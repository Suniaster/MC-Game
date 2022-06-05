use std::sync::{Arc, Mutex};

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

struct RenderTextInfoSystem;
impl<'a> System<'a> for RenderTextInfoSystem {
    type SystemData = ReadExpect<'a, Arc<Mutex<ScreenView>>>;

    fn run(&mut self, scren_mutex: Self::SystemData){
        let text = format!("Vextex count: 12");
        let mut screen = scren_mutex.lock().unwrap();
        screen.actions.update_text(1, text);
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        let screen_mutex = world.read_resource::<Arc<Mutex<ScreenView>>>();
        let mut screen = screen_mutex.lock().unwrap();

        screen.actions.create_text();
        screen.actions.create_text();
        screen.actions.create_text();
        screen.actions.create_text();
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
        .with_thread_local(
            RenderTextInfoSystem
        )
        .build();

    world.insert(arc_screen.clone());

    dispatcher.setup(&mut world);
    voxelviewer::start(world, dispatcher, arc_screen, evloop);
}
