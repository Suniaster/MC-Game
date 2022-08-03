use std::collections::HashMap;
use plugins::{Plugin, App};

pub mod system;

pub struct LoadedChunks {
    chunks: HashMap<isize, ()>,
}
impl LoadedChunks { pub fn new() -> Self { LoadedChunks { chunks: HashMap::new() } } }

pub const GRID_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 64;
pub const CUBE_SIZE: f32 = 1.;
pub const CHUNK_SIZE: f32 = GRID_SIZE as f32 * CUBE_SIZE;


pub struct TerrainPlugin;
impl Plugin for TerrainPlugin {
    fn build(&mut self, app: &mut App) {
        app.add_resource(LoadedChunks::new());
        
        app.add_system(system::TerrainSystem::new());
    }
}