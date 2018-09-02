use piston_window::{Context, types::Color};
use opengl_graphics::*;

use block::Block;

pub const APPLE_COLOR: Color = [0.84, 0.25, 0.25, 1.0];

pub struct Apple {
    pub body: Option<Block>,
}

impl Apple {
    pub fn new() -> Self {
        Apple {
            body: None,
        }
    }
    
    pub fn draw(&self, ctx: &Context, g: &mut GlGraphics) {
        self.body.as_ref().unwrap().draw(ctx, g, APPLE_COLOR);
    }

    pub fn get_position(&self) -> (i32, i32) {
        let body = self.body.as_ref().unwrap();
        (body.x, body.y)
    }
    
    pub fn update_state(&mut self, x: i32, y: i32) {
        self.body = Some(Block {
            x,
            y,
        });
    }
}
