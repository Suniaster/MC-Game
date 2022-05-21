use world::components::*;
use world::scene::GameScene;
use ncollide2d::math::Vector;
use voxelviewer::ViewObjectInfo;
pub struct Cube;
impl Cube {
    pub fn create(scene: &mut GameScene, view_actions: &mut voxelviewer::ViewActions) {
        let cube_idx = scene.entity_allocator.allocate();

        scene.add_component(&cube_idx, PositionComponent(Vector::new(10., 10.)));

        scene.add_component(&cube_idx, PhysicsComponent::new_random());

        scene.add_component(&cube_idx, SizeComponent(0.2, 0.2));


        scene.add_component(&cube_idx, RenderComponent{
            cube_idx: view_actions.create_cube(
                &ViewObjectInfo{
                    position: [10., 10., 2.], 
                    color: [0.1, 0.1, 0.5],
                    size: [0.2, 0.2, 0.2],
                    id: 0
                }
            )
        });

        scene.cubes.push(cube_idx);
    }
}
