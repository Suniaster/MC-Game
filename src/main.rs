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

fn main() {

    let mut controller = voxelviewer::ViewController::new();
    controller.on_update = |_, dt|{
        print!("\r FPS: {}", 1./dt.as_secs_f32());
    };
    controller.on_keybord_input = |_, key, _ |{
        match key{
            VirtualKeyCode::Z =>{
                println!("Z Pressed");
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