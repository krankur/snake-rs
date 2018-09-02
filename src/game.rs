// use piston_window::{Context, Key, types::Color, Transformed};
use piston_window::*;
use piston_window::types::Color;
use opengl_graphics::*;
use rand::{thread_rng, Rng};

use apple::*;
use snake::*;
use boundary::*;

const YELLOW_COLOR: Color = [1.0, 0.99, 0.22, 1.0];
pub const GAME_WIDTH: i32 = 30;
pub const GAME_HEIGHT: i32 = 30;
const REFRESH_INTERVAL: f64 = 0.1;

pub struct Game {
    width: i32,
    height: i32,
    boundary: Boundary,
    snake: Snake,
    apple: Apple,
    time_since_refresh: f64,
    score: i32,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Self {
        let boundary = Boundary {
            top_margin: 2,
            left_margin: 1,
            bottom_margin: 1,
            right_margin: 1,
        };
        Game {
            width,
            height,
            boundary,
            snake: Snake::new(width / 2, height / 2),
            apple: Apple::new(),
            time_since_refresh: 0.0,
            score: 0,
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
    
    fn add_score(&self, ctx: &Context, g: &mut GlGraphics, glyph_cache: &mut GlyphCache) {
        text(
            YELLOW_COLOR,
            12,
            format!("Score: {}", self.score).as_str(),
            glyph_cache,
            ctx.transform.trans(15.0, 15.0),
            g,
        ).unwrap();
    }

    pub fn draw(&mut self, ctx: &Context, g: &mut GlGraphics, glyph_cache: &mut GlyphCache) {
        if self.apple.body.is_none() {
            let (x, y) = self.get_random_unoccupied_coordinate();
            self.apple.update_state(x, y);
        }
        self.apple.draw(ctx, g);
        self.snake.draw(ctx, g);
        self.boundary.draw(ctx, g);

        self.add_score(ctx, g, glyph_cache);
    }

    fn get_random_unoccupied_coordinate(&self) -> (i32, i32) {
        let mut rng = thread_rng();
        let mut x = rng.gen_range(2, self.width - 2);
        let mut y = rng.gen_range(3, self.height - 2);
        while self.snake.has_occupied(x, y) {
            x = rng.gen_range(2, self.width - 2);
            y = rng.gen_range(3, self.height - 2);
        }
        (x, y)
    }

    fn is_snake_eating_apple(&mut self) -> bool {
        let (apple_x, apple_y) = self.apple.get_position();
        let (snake_x, snake_y) = self.snake.get_head_position();
        snake_x == apple_x && snake_y == apple_y
    }

    fn is_coordinate_available(&self, x: i32, y: i32) -> bool {
        self.snake.has_occupied(x, y) || self.boundary.is_overstepped(x, y)
    }

    pub fn update(&mut self, delta_time: f64) {
        if self.apple.body.is_some() && self.is_snake_eating_apple() {
            self.snake.should_grow = true;
            self.apple.body = None;
            self.score += 100;
        }
        if self.snake.is_dead {
            self.reset();
        }
        self.time_since_refresh += delta_time;

        if self.time_since_refresh > REFRESH_INTERVAL {
            let (new_head_x, new_head_y) = self.snake.get_next_head_position();

            if self.is_coordinate_available(new_head_x, new_head_y) {
                self.snake.is_dead = true;
            } else {
                self.snake.slither(new_head_x, new_head_y);
            }
            self.time_since_refresh = 0.0;
        }
    }
    
    fn reset(&mut self) {
        self.snake = Snake::new(GAME_WIDTH / 2, GAME_HEIGHT / 2);
        self.apple = Apple::new();
        self.time_since_refresh = 0.0;
        self.score = 0;
    }
}
