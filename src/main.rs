extern crate piston_window;
extern crate opengl_graphics;
extern crate rand;

mod block;
mod apple;
mod snake;
mod boundary;
mod canvas;

use piston_window::*;
use piston_window::types::Color;
use opengl_graphics::*;

use block::BLOCK_SIZE;
use canvas::*;

const BLACK_COLOR: Color = [0.0; 4];
const YELLOW_COLOR: Color = [1.0, 0.99, 0.22, 1.0];

fn main() {
    let width = (CANVAS_WIDTH * BLOCK_SIZE) as u32;
    let height = (CANVAS_HEIGHT * BLOCK_SIZE) as u32;
    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new("Snake", [width, height])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT);
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
                canvas.draw(&context, graphics);
                text(
                    YELLOW_COLOR,
                    12,
                    "Score: 678",
                    &mut glyph_cache,
                    context.transform.trans(15.0, 15.0),
                    graphics,
                ).unwrap();
            });
        }
        if let Some(Button::Keyboard(key)) = event.press_args() {
            canvas.detect_key_press(key);
        }
        event.update(|arg| {
            canvas.update(arg.dt);
        });
    }
}
