use super::Vec2D;

#[derive(Debug)]
pub struct WorldInfo {
    pub vel: Vec2D<f64>,
    pub pos: Vec2D<f64>,
    pub size: Vec2D<u32>
}

pub trait HasWorldInfo {
    fn get_world_info(&self) -> &WorldInfo;
    fn get_mut_world(&mut self) -> &mut WorldInfo;
}

impl WorldInfo {
    pub fn new() -> WorldInfo {
        return WorldInfo {
            pos: Vec2D {
                x: 0.0,
                y: 0.0
            },
            size:Vec2D {
                x: 100,
                y: 100
            },
            vel: Vec2D{
                x: 10.0,
                y: 10.0
            }
        }
    }
}
