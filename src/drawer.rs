// use entities::HasWorldInfo;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::render::Texture;
use sdl2::rect;
use super::entities::TEntity;
// use super::entities::world::TWorldInfo;

use std::collections::HashMap;

pub trait AssetsConfig {
    fn asset_name(&self) -> &str;
    fn asset_path(&self) -> &str;
}

pub struct Renderer {
    pub sdl_context: sdl2::Sdl,
    texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    canvas: Canvas<Window>,
    textures: HashMap<String, Texture>,
}

use sdl2::image::{LoadTexture};

impl Renderer {
    pub fn new() -> Renderer {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        
        // let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    
        let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .expect("could not initialize video subsystem");
    
        let canvas = window.into_canvas().build()
            .expect("could not make a canvas");

        let texture_creator:sdl2::render::TextureCreator<sdl2::video::WindowContext> = canvas.texture_creator();
        
        return Renderer {
            sdl_context: sdl_context,
            texture_creator,
            canvas,
            textures: HashMap::new()
        }
    }

    pub fn add_texture_for<T: AssetsConfig>(&mut self, obj:T){
        let texture:Texture = self.texture_creator.load_texture(obj.asset_path()).expect("");
        self.textures.insert(obj.asset_name().to_string(), texture);
    }

    pub fn draw<T: AssetsConfig + TEntity>(&mut self, obj: &mut T){
        let texture = self.textures.get(obj.asset_name()).unwrap();
        let world = obj.get_world();
        let r = rect::Rect::new(world.pos.x as i32, world.pos.y as i32, world.size.x, world.size.y);
        self.canvas.copy(&texture, None, r).unwrap();
    }

    pub fn clear_screen(&mut self){
        self.canvas.clear();
    }

    pub fn draw_screen(&mut self){
        self.canvas.present();
    }
}



// test
use crate::entities::Person;

impl AssetsConfig for Person{
    fn asset_name(&self) -> &str { return &"buizel" }
    fn asset_path(&self) -> &str { return &"./assets/av.jpg"}
}

impl AssetsConfig for &Person{
    fn asset_name(&self) -> &str { return &"buizel" }
    fn asset_path(&self) -> &str { return &"./assets/av.jpg"}
}