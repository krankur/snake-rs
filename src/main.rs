extern crate piston_window;
extern crate rand;

use std::collections::LinkedList;
use piston_window::*;
use piston_window::types::Color;

use rand::{thread_rng, Rng};

const BLOCK_SIZE: i32 = 10;
const CANVAS_WIDTH: i32 = 30;
const CANVAS_HEIGHT: i32 = 30;
const REFRESH_INTERVAL: f64 = 0.1;

const SNAKE_COLOR: Color = [0.42, 0.64, 0.15, 1.0];
const APPLE_COLOR: Color = [0.84, 0.25, 0.25, 1.0];
const BLACK_COLOR: Color = [0.1, 0.1, 0.1, 1.0];

#[derive(PartialEq)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn get_opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
        }
    }
}

struct Block {
    x: i32,
    y: i32,
}

impl Block {
    fn draw(&self, ctx: &Context, g: &mut G2d, color: Color) {
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

struct Snake {
    body: LinkedList<Block>,
    direction: Direction,
    should_grow: bool,
    is_dead: bool,
}

impl Snake {
    fn new(x: i32, y: i32) -> Snake {
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

    fn draw(&mut self, ctx: &Context, g: &mut G2d) {
        for block in self.body.iter_mut() {
            block.draw(ctx, g, SNAKE_COLOR);
        }
    }

    fn update(&mut self, dir: Direction) {
        self.direction = dir;
    }

    fn get_head_position(&self) -> (i32, i32) {
        let head = self.body.front().unwrap();
        (head.x, head.y)
    }

    fn is_overlapped(&self, x: i32, y: i32) -> bool {
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

    fn slither(&mut self, x: i32, y: i32) {
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

struct Apple {
    body: Block,
    is_eaten: bool,
}

impl Apple {
    fn new(x: i32, y: i32) -> Self {
        Apple {
            body: Block {
                x,
                y,
            },
            is_eaten: true,
        }
    }
    
    fn draw(&self, ctx: &Context, g: &mut G2d) {
        self.body.draw(ctx, g, APPLE_COLOR);
    }

    fn get_position(&self) -> (i32, i32) {
        (self.body.x, self.body.y)
    }
}

struct Boundary {
    top_margin: i32,
    left_margin: i32,
    bottom_margin: i32,
    right_margin: i32,
}

impl Boundary {
    fn draw(&self, ctx: &Context, g: &mut G2d, color: Color) {
        rectangle(
            color,
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
            color,
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
            color,
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
            color,
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

    fn is_overstepped(&self, x: i32, y: i32) -> bool {
        x == self.left_margin ||
        x == CANVAS_WIDTH - self.right_margin - 1 ||
        y == self.top_margin ||
        y == CANVAS_HEIGHT - self.bottom_margin - 1
    }
}

struct Canvas {
    width: i32,
    height: i32,
    boundary: Boundary,
    snake: Snake,
    apple: Option<Apple>,
    time_since_refresh: f64,
}

impl Canvas {
    fn new(width: i32, height: i32) -> Self {
        let boundary = Boundary {
            top_margin: 2,
            left_margin: 1,
            bottom_margin: 1,
            right_margin: 1,
        };
        Canvas {
            width,
            height,
            boundary,
            snake: Snake::new(width / 2, height / 2),
            apple: Some(Apple::new(2, 2)),
            time_since_refresh: 0.0,
        }
    }

    fn detect_key_press(&mut self, key: Key) {
        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Left => Some(Direction::Left),
            Key::Down => Some(Direction::Down),
            Key::Right => Some(Direction::Right),
            _ => None
        };
        if let Some(unwrapped_dir) = dir {
            if self.snake.direction != unwrapped_dir.get_opposite() {
                self.snake.update(unwrapped_dir);
            }
        }
    }

    fn draw(&mut self, ctx: &Context, g: &mut G2d) {
        if self.is_apple_eaten() {
            self.update_apple_state();
        }
        let apple = self.apple.as_ref().unwrap();
        apple.draw(ctx, g);
        self.snake.draw(ctx, g);
        self.boundary.draw(ctx, g, BLACK_COLOR);
    }

    fn get_new_apple_position(&self) -> (i32, i32) {
        let mut rng = thread_rng();
        let mut x = rng.gen_range(2, self.width - 2);
        let mut y = rng.gen_range(3, self.height - 2);
        while self.snake.is_overlapped(x, y) {
            x = rng.gen_range(2, self.width - 2);
            y = rng.gen_range(3, self.height - 2);
        }
        (x, y)
    }

    fn is_apple_eaten(&self) -> bool {
        let apple = self.apple.as_ref().unwrap();
        apple.is_eaten
    }

    fn set_apple_eaten(&mut self) {
        let apple = self.apple.as_mut().unwrap();
        apple.is_eaten = true;
    }
    
    fn update_apple_state(&mut self) {
        let (x, y) = self.get_new_apple_position();
        let apple = self.apple.as_mut().unwrap();
        apple.body.x = x;
        apple.body.y = y;
        apple.is_eaten = false;
    }

    fn is_snake_eating_apple(&mut self) -> bool {
        let apple = self.apple.as_mut().unwrap();
        let (apple_x, apple_y) = apple.get_position();
        let (snake_x, snake_y) = self.snake.get_head_position();
        snake_x == apple_x && snake_y == apple_y
    }

    fn update(&mut self, delta_time: f64) {
        if self.is_snake_eating_apple() {
            self.snake.should_grow = true;
            self.set_apple_eaten();
        }

        if self.snake.is_dead {
            self.reset();
        }

        self.time_since_refresh += delta_time;

        if self.time_since_refresh > REFRESH_INTERVAL {
            let (x, y) = self.snake.get_head_position();
            let new_x: i32;
            let new_y: i32; 
            
            match self.snake.direction {
                Direction::Up => {
                    new_x = x;
                    new_y = y - 1;
                },
                Direction::Left => {
                    new_x = x - 1;
                    new_y = y;
                },
                Direction::Down => {
                    new_x = x;
                    new_y = y + 1;
                },
                Direction::Right => {
                    new_x = x + 1;
                    new_y = y;
                }
            };

            if self.snake.is_overlapped(new_x, new_y) || self.boundary.is_overstepped(new_x, new_y) {
                self.snake.is_dead = true;
            } else {
                self.snake.slither(new_x, new_y);
            }
            self.time_since_refresh = 0.0;
        }
    }
    
    fn reset(&mut self) {
        let (apple_x, apple_y) = self.get_new_apple_position();
        self.snake = Snake::new(CANVAS_WIDTH / 2, CANVAS_HEIGHT / 2);
        self.apple = Some(Apple::new(apple_x, apple_y));
        self.time_since_refresh = 0.0;
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake", [(CANVAS_WIDTH * BLOCK_SIZE) as u32, (CANVAS_HEIGHT * BLOCK_SIZE) as u32])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            canvas.detect_key_press(key);
        }
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);
            canvas.draw(&context, graphics);
        });
        event.update(|arg| {
            canvas.update(arg.dt);
        });
    }
}
