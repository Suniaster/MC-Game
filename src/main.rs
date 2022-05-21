use winit::event::{VirtualKeyCode, ElementState};

use voxelviewer;
use world;
mod systems;
mod entities;

// Proximos Objetivos
// - Adicionar Colisao
// - Adicionar Input
// - Ajustar estrutura do loop principal
// - Adicionar fixed dt
// - Decidir como estruturar melhor sistemas em arquivos separados
// - Fazer alguma parada massa
pub struct Control{
    world: world::scene::GameScene
}
impl voxelviewer::ViewController for Control{
    fn on_update(&mut self, actions: &mut voxelviewer::ViewActions, dt: std::time::Duration){
        print!(
            "\r FPS: {}", 
            1./dt.as_secs_f32(),
        );
    }

    fn on_keybord_input(&mut self, actions: &mut voxelviewer::ViewActions, b: VirtualKeyCode, c: ElementState){

    }
}



fn main() {
    let world = world::scene::GameScene::new((40., 20.));
    // let mut controller = voxelviewer::ViewController::new();
    let mut CONTROLLER = Control{world};
    voxelviewer::main(Box::new(CONTROLLER));
}