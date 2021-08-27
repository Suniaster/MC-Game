#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_export]
macro_rules! add_getter{
  ($f_name:ident, $var_name:ident, $_type: ty) => {
      fn $f_name(&self) -> &$_type {
        return &self.$var_name;
      }
  };
}

pub mod physics;
pub mod world;

#[derive(Debug)]
pub struct Vec2D<T> {
    pub x: T, pub y: T
}


use entity_macros::Physics;
use world::WorldInfo;

#[derive(Debug)]
#[derive(Physics)]
pub struct Person{
    world: world::WorldInfo
}

impl Person {
    pub fn new() -> Person {
        Person{ 
            world: world::WorldInfo::new()
        }
    }
}
