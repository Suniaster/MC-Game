
use wgpu_glyph::{GlyphBrush, Section, Text};

pub struct ScreenText{
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub color: [f32; 4],
}

impl ScreenText{
    pub fn new(text: String, x: f32, y: f32, color: [f32; 4])-> ScreenText{
        ScreenText{
            text,
            x,
            y,
            color,
        }
    }

    pub fn draw<T>(&self, glyph_brush: &mut GlyphBrush<T>, width: f32, heigth: f32){
        glyph_brush.queue(Section {
            screen_position: (self.x, self.y),
            bounds: (width, heigth),
            text: vec![Text::new(self.text.as_str())
                .with_color([0.0, 0.0, 0.0, 1.0])
                .with_scale(40.0)],
            ..Section::default()
        });
    }
}