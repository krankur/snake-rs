use piston_window::{Context, G2d, rectangle};

use block::BLOCK_SIZE;
use canvas::{BLACK_COLOR, CANVAS_WIDTH, CANVAS_HEIGHT};

pub struct Boundary {
    pub top_margin: i32,
    pub left_margin: i32,
    pub bottom_margin: i32,
    pub right_margin: i32,
}

impl Boundary {
    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        rectangle(
            BLACK_COLOR,
            [
                (self.left_margin * BLOCK_SIZE) as f64,
                (self.top_margin * BLOCK_SIZE) as f64,
                ((CANVAS_WIDTH - self.left_margin - self.right_margin) * BLOCK_SIZE) as f64,
                BLOCK_SIZE as f64,
            ],
            ctx.transform,
            g,
        );
        rectangle(
            BLACK_COLOR,
            [
                ((CANVAS_WIDTH - self.right_margin - 1) * BLOCK_SIZE) as f64,
                ((self.top_margin + 1) * BLOCK_SIZE) as f64,
                BLOCK_SIZE as f64,
                ((CANVAS_HEIGHT - self.top_margin - self.bottom_margin - 2) * BLOCK_SIZE) as f64,
            ],
            ctx.transform,
            g,
        );
        rectangle(
            BLACK_COLOR,
            [
                (self.left_margin * BLOCK_SIZE) as f64,
                ((CANVAS_HEIGHT - self.bottom_margin  - 1) * BLOCK_SIZE) as f64,
                ((CANVAS_WIDTH - self.left_margin - self.right_margin) * BLOCK_SIZE) as f64,
                BLOCK_SIZE as f64,
            ],
            ctx.transform,
            g,
        );
        rectangle(
            BLACK_COLOR,
            [
                (self.left_margin * BLOCK_SIZE) as f64,
                ((self.top_margin + 1) * BLOCK_SIZE) as f64,
                BLOCK_SIZE as f64,
                ((CANVAS_HEIGHT - self.top_margin - self.bottom_margin - 2) * BLOCK_SIZE) as f64,
            ],
            ctx.transform,
            g,
        );
    }

    pub fn is_overstepped(&self, x: i32, y: i32) -> bool {
        x == self.left_margin ||
        x == CANVAS_WIDTH - self.right_margin - 1 ||
        y == self.top_margin ||
        y == CANVAS_HEIGHT - self.bottom_margin - 1
    }
}
