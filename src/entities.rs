
use super::scene::GameScene;
use super::components::*;
use ncollide2d::math::Vector;


pub struct Blob;
impl Blob{
  pub fn create(scene: &mut GameScene){
    let blob_idx = scene.entity_allocator.allocate();

    scene.components.get_mut::<ComponentMap::<PositionComponent>>().unwrap().set(
      &blob_idx, PositionComponent(Vector::new(100., 100.))
    );

    scene.components.get_mut::<ComponentMap::<PhysicsComponent>>().unwrap().set(
      &blob_idx, PhysicsComponent::new_random()
    );

    scene.components.get_mut::<ComponentMap::<SizeComponent>>().unwrap().set(
      &blob_idx, SizeComponent(40., 40.)
    );

    scene.components.get_mut::<ComponentMap::<TextureId>>().unwrap().set(
      &blob_idx, TextureId(String::from("test"))
    );

    scene.blobs.push(blob_idx);
  }
}