use std::time::Duration;

use winit::event::{VirtualKeyCode, ElementState};

use voxelviewer;
use world;
mod systems;
mod entities;

// Proximos Objetivos
// x - Adicionar bordas (linhas) nos cubos - LineStrip = 2,
// X - Descobrir como botar texto na tela - https://github.com/hecrj/wgpu_glyph
// X - Escrever na tela fps 
// - e direção olhando
// - Otmizar o uso de vertices no grid (remover repetidos)
// - Ajustar cores (não tão sendo efetivamente usados)
// - Adicionar geração de terreno
// - Descobrir como fazer um tipo de animação de entrada
// - Descobrir como rotacionar cubos e grids
// - Tentar fazer algum modelo no MagicaVoxel
// - Tentar importar modelo pelo back e passar para o front renderiza-lo
// - 
pub struct Control{
    world: world::scene::GameScene,
    pub fps_text_id: usize,
    pub total_time: Duration
}

impl voxelviewer::ViewController for Control{
    fn on_update(&mut self, actions: &mut voxelviewer::ViewActions, dt: std::time::Duration){
        systems::render_fps_system(self, actions, dt);
        systems::render_system(&mut self.world.components, actions);
    }

    fn on_keybord_input(&mut self, actions: &mut voxelviewer::ViewActions, b: VirtualKeyCode, _c: ElementState){
        if b == VirtualKeyCode::C && _c == ElementState::Pressed {
            entities::Cube::create(&mut self.world, actions);
        }
    }

    fn before_start(&mut self, a:&mut voxelviewer::ViewActions) -> () {
        self.fps_text_id = a.create_text(
            String::from("FPS: XX"),
             50., 50., 
             [0., 0., 0., 1.]
        );
    }
}

fn main() {
    let world = world::scene::GameScene::new((40., 20.));
    let controller = Control{
        world, 
        fps_text_id: 0,
        total_time: Duration::new(0, 0)
    };
    voxelviewer::main(Box::new(controller));
}