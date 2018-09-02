use piston_window::{Context, rectangle, types::Color};
use opengl_graphics::*;

use block::BLOCK_SIZE;
use canvas::{CANVAS_WIDTH, CANVAS_HEIGHT};

pub struct Boundary {
    pub top_margin: i32,
    pub left_margin: i32,
    pub bottom_margin: i32,
    pub right_margin: i32,
}

pub const GREY_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

impl Boundary {
    pub fn draw(&self, ctx: &Context, g: &mut GlGraphics) {
        rectangle(
            GREY_COLOR,
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
            GREY_COLOR,
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
            GREY_COLOR,
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
            GREY_COLOR,
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
