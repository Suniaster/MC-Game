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

fn main() {
    let world_scene = world::scene::GameScene::new((40., 20.));
    let mut controller = voxelviewer::ViewController::new();
    
    controller.on_update = |actions,world, dt|{
        print!(
            "\r FPS: {} \t\t\tCount: {}", 
            1./dt.as_secs_f32(), 
            world.entity_allocator.size()
        );

        systems::physics_system(&mut world.components, dt.as_secs_f32(), 1.);
        systems::circular_world_system(&mut world.components, &world.scene_size);
        systems::render_system(&world.components, actions);
    };
    controller.on_keybord_input = |actions, world, key, state |{
        match key{
            VirtualKeyCode::Z =>{
                if state == ElementState::Pressed {
                    entities::Cube::create(world, actions);
                }
            }
            _ => {}
        }
    };

    voxelviewer::main(controller, world_scene);
}