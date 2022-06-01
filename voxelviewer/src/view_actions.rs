use super::scene::*;
use super::grid;
use super::scene_entity;
use super::cube;
use super::screen_text;
use nalgebra::{Point3, Vector3};
pub struct ViewActions{
  pub state: State
}

pub struct ViewObjectInfo{
    pub position: Point3<f32>,
    pub color: [f32; 3],
    pub size: [f32; 3],
    pub id: u32
}

impl ViewActions{

    pub fn create_grid(&mut self, position: [f32; 3], cube_size: f32, grid_mat: grid::GridMatrix) -> ViewObjectInfo{
      let mut grid = grid::Grid::create_from(
          cube_size,
          grid_mat
      );
      grid.move_origin(Point3::from(position));
        // Create outline for mesh
        {
            let ent = scene_entity::SceneEntity::new(
                &self.state.device,
                grid._build_outline()
            );
            self.state.entities_outlines.insert(ent.id, ent);
        }

      let new_ent = scene_entity::SceneEntity::new(
          &self.state.device,
          grid.build()
      );

      let id = new_ent.id;
      self.state.entities.insert(id, new_ent);

      return ViewObjectInfo{
          position: grid.origin,
          color: [0., 0., 0.],
          size: [0., 0., 0.],
          id
      };
  }

  pub fn create_cube(&mut self, obj: ViewObjectInfo)-> ViewObjectInfo{
      let mut mesh = cube::Cuboid::new(
          Vector3::from(obj.size), 
          obj.color
      );
      mesh.move_origin_to(obj.position);

      let new_ent = scene_entity::SceneEntity::new(
          &self.state.device, 
          mesh.build()
      );

      let id = new_ent.id;
      let mut obj = obj;
      self.state.entities.insert(id, new_ent);

      obj.id = id;
      return obj;
  }

  pub fn update_cube(&mut self, obj: &ViewObjectInfo){
      let entity = self.state.entities.get_mut(&obj.id).unwrap();
      entity.update_origin(&self.state.queue, obj.position);
  }

  pub fn set_camera_pos(&mut self, pos: nalgebra::Point3<f32>){
      self.state.camera.position = pos;
  }

  pub fn set_camera_config(&mut self, speed: Option<f32>, sens: Option<f32>){
      if let Some(value) = speed {
          self.state.camera_controller.speed = value;
      }
      if let Some(value) = sens {
          self.state.camera_controller.sensitivity = value;
      }
  }

  pub fn update_text(&mut self, id: usize, text: String){
      self.state.screen_texts[id as usize].text = text;
  }

  pub fn create_text(&mut self)-> usize{
      let new_text = screen_text::ScreenText::new(
          String::from(""), 
          10., 
          20. * self.state.screen_texts.len() as f32, 
      [0., 0., 0., 1.]);
      self.state.screen_texts.push(new_text);
      return self.state.screen_texts.len()-1;
  }

  pub fn get_vertex_count(&self) -> u32{
      let mut count = 0;
      for (_, entity) in self.state.entities.iter(){
          count += entity.mesh.vertices.len() as u32;
      }
      return count;
  }
}
