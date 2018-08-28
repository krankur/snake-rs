use piston_window::{Context, G2d, types::Color};

use block::Block;

pub const APPLE_COLOR: Color = [0.84, 0.25, 0.25, 1.0];

pub struct Apple {
    pub body: Block,
    pub is_eaten: bool,
}

impl Apple {
    pub fn new(x: i32, y: i32) -> Self {
        Apple {
            body: Block {
                x,
                y,
            },
            is_eaten: true,
        }
    }
    
    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        self.body.draw(ctx, g, APPLE_COLOR);
    }

    pub fn get_position(&self) -> (i32, i32) {
        (self.body.x, self.body.y)
    }
}
