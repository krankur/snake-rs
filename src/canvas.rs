use piston_window::{Context, Key};
use opengl_graphics::*;
use rand::{thread_rng, Rng};

use apple::*;
use snake::*;
use boundary::*;

pub const CANVAS_WIDTH: i32 = 30;
pub const CANVAS_HEIGHT: i32 = 30;
const REFRESH_INTERVAL: f64 = 0.1;

pub struct Canvas {
    width: i32,
    height: i32,
    boundary: Boundary,
    snake: Snake,
    apple: Option<Apple>,
    time_since_refresh: f64,
}

impl Canvas {
    pub fn new(width: i32, height: i32) -> Self {
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

    pub fn detect_key_press(&mut self, key: Key) {
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

    pub fn draw(&mut self, ctx: &Context, g: &mut GlGraphics) {
        if self.is_apple_eaten() {
            self.update_apple_state();
        }
        let apple = self.apple.as_ref().unwrap();
        apple.draw(ctx, g);
        self.snake.draw(ctx, g);
        self.boundary.draw(ctx, g);
    }

    fn generate_new_apple_position(&self) -> (i32, i32) {
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
        let (x, y) = self.generate_new_apple_position();
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

    pub fn update(&mut self, delta_time: f64) {
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
        let (apple_x, apple_y) = self.generate_new_apple_position();
        self.snake = Snake::new(CANVAS_WIDTH / 2, CANVAS_HEIGHT / 2);
        self.apple = Some(Apple::new(apple_x, apple_y));
        self.time_since_refresh = 0.0;
    }
}
