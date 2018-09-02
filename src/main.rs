extern crate piston_window;
extern crate opengl_graphics;
extern crate rand;

mod block;
mod apple;
mod snake;
mod boundary;
mod game;

use piston_window::*;
use piston_window::types::Color;
use opengl_graphics::{GlGraphics, GlyphCache};

use block::BLOCK_SIZE;
use game::*;

const BLACK_COLOR: Color = [0.0; 4];

fn main() {
    let width = (GAME_WIDTH * BLOCK_SIZE) as u32;
    let height = (GAME_HEIGHT * BLOCK_SIZE) as u32;
    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new("Snake", [width, height])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(GAME_WIDTH, GAME_HEIGHT);
    let mut gl = GlGraphics::new(opengl);

    let mut glyph_cache = GlyphCache::new(
        "./assets/fonts/Lato-Regular.ttf",
        (),
        TextureSettings::new().min(Filter::Nearest),
    ).unwrap();

    while let Some(event) = window.next() {
        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |context, graphics| {
                clear(BLACK_COLOR, graphics);
                game.draw(&context, graphics, &mut glyph_cache);
            });
        }
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.detect_key_press(key);
        }
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
