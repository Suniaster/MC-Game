use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

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
use voxelviewer::ScreenView;

pub type ScreenMutex = Arc<Mutex<ScreenView>>;

fn main() {
    let mut world = World::new();

    world.register::<components::RenderComponent>();
    world.register::<components::PositionComponent>();
    world.register::<components::GridDescriptorComponent>();

    let (screen, evloop) = voxelviewer::create_screen();
    let arc_screen = Arc::new(Mutex::new(screen));

    let mut dispatcher = 
        DispatcherBuilder::new()
        .with(systems::UpdateDtSystem{
            last_time: std::time::Instant::now()
        }, "update_dt_system", &[])
        .with(
            terrain::TerrainSystem, "terrain", &[]
        )
        .with_thread_local(systems::RenderTextInfoSystem{
            texts_ids: HashMap::new(),
            time_counter: std::time::Duration::new(0, 0),
        })
        .build();

    world.insert(arc_screen.clone());
    world.insert(terrain::LoadedChunks::new());
    world.insert(systems::WorldDt(Duration::new(0, 0)));

    dispatcher.setup(&mut world);
    voxelviewer::start(world, dispatcher, arc_screen, evloop);
}
