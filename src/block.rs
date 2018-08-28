use piston_window::{Context, G2d, rectangle, types::Color};

pub const BLOCK_SIZE: i32 = 10;

pub struct Block {
    pub x: i32,
    pub y: i32,
}

impl Block {
    pub fn draw(&self, ctx: &Context, g: &mut G2d, color: Color) {
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