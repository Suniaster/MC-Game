use winit::event::{VirtualKeyCode};

use voxelviewer;
use world;

// Proximos Objetivos
// - Adicionar Colisao
// - Adicionar Input
// - Ajustar estrutura do loop principal
// - Adicionar fixed dt
// - Decidir como estruturar melhor sistemas em arquivos separados
// - Fazer alguma parada massa

fn main() {
    let world_scene = world::scene::GameScene::new((800., 600.));
    let mut controller = voxelviewer::ViewController::new();
    // world.setup_assets(&texture_creator);
    
    controller.on_update = |_,_, dt|{
        print!("\r FPS: {}", 1./dt.as_secs_f32());
    };
    controller.on_keybord_input = |action, world, key, _ |{
        let mut rng = rand::thread_rng();
        match key{
            VirtualKeyCode::Z =>{
                // action.world;
                // entities::Cube::create(&mut world, view_actions: &mut voxelviewer::ViewActions)
            }
            _ => {}
        }
    };

    voxelviewer::main(controller, world_scene);
    

    // world.setup_components();

    

    // systems::physics_system(&mut world.components, delta_t, world.time_scale);
    // systems::circular_world_system(&mut world.components, &world.scene_size);

    // systems::render_system(&world.components, &world.assets, &mut canvas);
}