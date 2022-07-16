use std::sync::{Arc, Mutex};
use std::time::Duration;

use nalgebra::{Point3, Vector3};
use rapier3d::prelude::{ RigidBodySet, ColliderSet};
use specs::prelude::*;

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
//      - Reescrevendo shaders do front para padronizar uso de grid 
//      - Parei em reescrevendo "grid_instance"
//      - Proximos passos são, reescrever grid instance, refotarar criação de grid para não incluir vertex
//      - Mudar render system para usar novo padrão
//      - Adicionar no novo modelo a criação de BB
//      - Adicionar no novo modelo a criação de Wireframe
//  - Adicionar interface com front parar mesh
//  - Adicionar interface com front parar BoundingBox
//  - Adicionar interface com front parar Wireframe
// - Ajustar geração de terreno
// - Fazer render ser independete do resto, de modo a poder mexer a camera mesmo com as coisas carregando
// - Adicionar algum tipo de terminal, console ou algo que ajude na visualização dos dados
// - Adicionar luz no céu

// - Ajustar cores (não tão sendo efetivamente usados)
// - Descobrir como fazer um tipo de animação de entrada
// - Descobrir como rotacionar cubos e grids
// - Tentar fazer algum modelo no MagicaVoxel
// - Tentar importar modelo pelo back e passar para o front renderiza-lo
// - Adicionar menu para mudanças de propriedades da camera

use specs::DispatcherBuilder;
use systems::RenderTextInfoSystem;
use systems::physics::{PhysicsSystem, VelocityComponent, RigidBodyComponent, AddRigidBodyCubeFlag, PhysicsWorldResource, PhysicsWorldManagerSystem};
use voxelviewer::ScreenView;
use voxelviewer::view_system::camera_system::CameraResource;
use voxelviewer::view_system::components::{PositionComponent, LookingDirectionComponent, MeshRendererComponent};

pub type MultiThread<T> = Arc<Mutex<T>>;
pub type ScreenMutex = MultiThread<ScreenView>;


fn main() {
    let mut world = World::new();

    let (screen, evloop) = voxelviewer::create_screen();
    let arc_screen = Arc::new(Mutex::new(screen));

    // Components
    world.register::<LookingDirectionComponent>();
    world.register::<PositionComponent>();
    world.register::<MeshRendererComponent>();
    world.register::<RigidBodyComponent>();
    world.register::<VelocityComponent>();
    world.register::<AddRigidBodyCubeFlag>();

    // Resources
    world.insert(terrain::LoadedChunks::new());
    world.insert(systems::WorldDt(Duration::new(0, 0)));
    world.insert(voxelviewer::view_system::resources::DeviceEventBuffer::default());
    world.insert(arc_screen.clone());
    world.insert(PhysicsWorldResource::new());
    world.insert(RigidBodySet::new());
    world.insert(ColliderSet::new());

    let mut dispatcher = 
        DispatcherBuilder::new()
        .with(systems::UpdateDtSystem{
            last_time: std::time::Instant::now()
        }, "update_dt_system", &[]
        ).with(
            terrain::TerrainSystem
        , "terrain_system", &[]
        ).with(
            RenderTextInfoSystem::new()
        , "render_text_info_system", &["update_dt_system"]
        ).with(
            voxelviewer::view_system::camera_system::CameraSystem::new()
        , "camera_system", &["update_dt_system"]
        ).with(
            PhysicsWorldManagerSystem,
            "physics_manager_system", &["update_dt_system"]
        ).with(
            PhysicsSystem::new(),
            "physics_system", &["update_dt_system", "physics_manager_system"]
        ).with(
            systems::io::IoSystem::new(),
            "io_system", &["update_dt_system"]
        ).
        with_thread_local(
            voxelviewer::view_system::UpdateViewMeshesSystem::new(arc_screen.clone())
        ).with_thread_local(
            voxelviewer::view_system::ViewSystem::new(arc_screen.clone())
        ).build();
    dispatcher.setup(&mut world);

    // Create camera
    let camera = world
        .create_entity()
        .with(PositionComponent::new(Point3::new(0.0, 10.0, 0.0)))
        .with(LookingDirectionComponent::new(0.,0.))
        .with(VelocityComponent(Vector3::new(0.0, 0.0, 0.0)))
        .with(AddRigidBodyCubeFlag(1.))
        .build() 
    ;
    world.insert(CameraResource::new(camera));

    voxelviewer::start(world, dispatcher, arc_screen, evloop);
}
