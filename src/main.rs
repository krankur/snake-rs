extern crate piston_window;
extern crate rand;

mod block;
mod apple;
mod snake;
mod boundary;
mod canvas;

use piston_window::*;

use block::BLOCK_SIZE;
use canvas::*;

fn main() {
    let width = (CANVAS_WIDTH * BLOCK_SIZE) as u32;
    let height = (CANVAS_HEIGHT * BLOCK_SIZE) as u32;
    let mut window: PistonWindow = WindowSettings::new("Snake", [width, height])
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
