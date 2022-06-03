use std::{collections::HashMap, time::Duration};

use winit::event::{ElementState, VirtualKeyCode};

use voxelviewer;
use voxelviewer::view_actions::*;
use world;
mod entities;
mod systems;
mod terrain;
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
pub struct Control {
    world: world::scene::GameScene,
    pub texts_ids: HashMap<String, usize>,
    pub total_time: Duration,
}

impl voxelviewer::ViewController for Control {
    fn on_update(&mut self, actions: &mut ViewActions, dt: std::time::Duration) {
        systems::render_fps_system(self, actions, dt);
        systems::render_system(&mut self.world.components, actions);
        terrain::terrain_system(self, actions);
    }

    fn on_keybord_input(&mut self, actions: &mut ViewActions, b: VirtualKeyCode, _c: ElementState) {
        if b == VirtualKeyCode::C && _c == ElementState::Pressed {
            entities::Cube::create(&mut self.world, actions);
        }
    }

    fn before_start(&mut self, a: &mut ViewActions) -> () {
        let fps_id = a.create_text();
        self.texts_ids.insert(String::from("fps"), fps_id);

        let looking_id = a.create_text();
        self.texts_ids.insert(String::from("looking"), looking_id);

        let position_id = a.create_text();
        self.texts_ids.insert(String::from("position"), position_id);

        let id = a.create_text();
        self.texts_ids.insert(String::from("vertices"), id);
    }
}

fn main() {
    let world = world::scene::GameScene::new();

    let mut test = world::World::new();
    test
        .add_component_storage::<i32>()
        .add_component_storage::<f64>()
    ;

    test.build_entity()
        .with_component(1)
        .with_component(3.14)
        .finish();
    
    test.build_entity()
        .with_component(2)
        .with_component(2.63)
        .finish();

    let iter1 = test.iter_comp::<i32>();
    let iter2 = test.iter_comp::<f64>();
    let f_iter = iter1.zip(iter2);
    for i in f_iter{
        println!("{:?}", i);
    }
    
    let controller = Control {
        world,
        texts_ids: HashMap::new(),
        total_time: Duration::new(0, 0),
    };
    // voxelviewer::main(Box::new(controller));
}
