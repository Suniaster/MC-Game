mod components;
mod entities;
mod scene;
mod systems;

use winit::event::{VirtualKeyCode};

use voxelviewer;

// Proximos Objetivos
// - Adicionar Colisao
// - Adicionar Input
// - Ajustar estrutura do loop principal
// - Adicionar fixed dt
// - Decidir como estruturar melhor sistemas em arquivos separados
// - Fazer alguma parada massa
use rand::prelude::*;

fn main() {
    let mut controller = voxelviewer::ViewController::new();
    
    controller.on_update = |_, dt|{
        print!("\r FPS: {}", 1./dt.as_secs_f32());
    };
    controller.on_keybord_input = |action, key, _ |{
        let mut rng = rand::thread_rng();
        match key{
            VirtualKeyCode::Z =>{
                action.create_cube(
                    [
                        rng.gen::<f32>() * 100. - 50., 
                        rng.gen::<f32>() * 100. - 50.,
                        rng.gen::<f32>() * 100. - 50.,
                    ],
                    [0.2, 0.1, 0.1]
                );
            }
            _ => {}
        }
    };
    voxelviewer::main(controller);
    
    // let mut world = scene::GameScene::new((800., 600.));

    // world.setup_components();
    // world.setup_assets(&texture_creator);

    

    // systems::physics_system(&mut world.components, delta_t, world.time_scale);
    // systems::circular_world_system(&mut world.components, &world.scene_size);

    // systems::render_system(&world.components, &world.assets, &mut canvas);
}