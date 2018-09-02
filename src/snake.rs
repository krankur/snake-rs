use std::collections::LinkedList;

use piston_window::{Context, types::Color};
use opengl_graphics::*;

use block::Block;

pub const SNAKE_COLOR: Color = [0.42, 0.64, 0.15, 1.0];

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    pub fn get_opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
        }
    }
}

pub struct Snake {
    body: LinkedList<Block>,
    pub direction: Direction,
    pub should_grow: bool,
    pub is_dead: bool,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();

        body.push_back(Block {
            x: x,
            y: y,
        });
        body.push_back(Block {
            x: x - 1,
            y: y,
        });
        body.push_back(Block {
            x: x - 2,
            y: y,
        });
        body.push_back(Block {
            x: x - 3,
            y: y,
        });
        Snake {
            body: body,
            direction: Direction::Right,
            should_grow: false,
            is_dead: false,
        }
    }

    pub fn draw(&mut self, ctx: &Context, g: &mut GlGraphics) {
        for block in self.body.iter_mut() {
            block.draw(ctx, g, SNAKE_COLOR);
        }
    }

    pub fn update(&mut self, dir: Direction) {
        self.direction = dir;
    }

    pub fn get_head_position(&self) -> (i32, i32) {
        let head = self.body.front().unwrap();
        (head.x, head.y)
    }

    pub fn has_occupied(&self, x: i32, y: i32) -> bool {
        let mut cnt = 0;
        for block in self.body.iter() {
            if block.x == x && block.y == y {
                return true;
            }
            cnt += 1;
            if cnt == self.body.len() {
                break;
            }
        }
        false
    }

    pub fn slither(&mut self, x: i32, y: i32) {
        self.body.push_front(Block {
            x,
            y,
        });
        if !self.should_grow {
            self.body.pop_back();
        } else {
            self.should_grow = false;
        }
    }
}
