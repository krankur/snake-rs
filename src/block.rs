use piston_window::{Context, rectangle, types::Color};
use opengl_graphics::*;

pub const BLOCK_SIZE: i32 = 10;

pub struct Block {
    pub x: i32,
    pub y: i32,
}

impl Block {
    pub fn draw(&self, ctx: &Context, g: &mut GlGraphics, color: Color) {
        let x = (self.x * BLOCK_SIZE) as f64;
        let y = (self.y * BLOCK_SIZE)  as f64;
        rectangle(
            color,
            [x, y, BLOCK_SIZE as f64, BLOCK_SIZE as f64],
            ctx.transform,
            g,
        );
    }
}