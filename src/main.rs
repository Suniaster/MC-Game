use winit::event::{VirtualKeyCode, ElementState};

use voxelviewer;
use world;
mod systems;
mod entities;

// Proximos Objetivos
// - Adicionar bordas (linhas) nos cubos - LineStrip = 2,
// - Descobrir como botar texto na tela - https://github.com/hecrj/wgpu_glyph
// - Escrever na tela fps e direção olhando
// - Otmizar o uso de vertices no grid (remover repetidos)
// - Ajustar cores (não tão sendo efetivamente usados)
// - Adicionar geração de terreno
// - Descobrir como fazer um tipo de animação de entrada
// - Descobrir como rotacionar cubos e grids
// - Tentar fazer algum modelo no MagicaVoxel
// - Tentar importar modelo pelo back e passar para o front renderiza-lo
// - 
pub struct Control{
    world: world::scene::GameScene
}

impl voxelviewer::ViewController for Control{
    fn on_update(&mut self, actions: &mut voxelviewer::ViewActions, dt: std::time::Duration){
        print!(
            "\r FPS: {}", 
            actions.state.entities.len()
        );

        systems::render_system(&mut self.world.components, actions);
        // systems::physics_system(&mut self.world.components, dt.as_secs_f32(), 1.);
        // systems::circular_world_system(&mut self.world.components, &self.world.scene_size);
    }

    fn on_keybord_input(&mut self, actions: &mut voxelviewer::ViewActions, b: VirtualKeyCode, _c: ElementState){
        if b == VirtualKeyCode::C && _c == ElementState::Pressed {
            entities::Cube::create(&mut self.world, actions);
        }
    }
}



fn main() {
    let world = world::scene::GameScene::new((40., 20.));
    let controller = Control{world};
    voxelviewer::main(Box::new(controller));
}