extern crate piston_window;

use piston_window::*;
use piston_window::types::Color;

const WINDOW_SIZE: [u32; 2] = [600, 400];
const BG_COLOR: Color = [0.204, 0.596, 0.859, 1.0];
const SNAKE_COLOR: Color = [0.792, 0.894, 0.965, 1.0];

struct Point<T> {
    x: T,
    y: T
}

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Meow", WINDOW_SIZE)
            .exit_on_esc(true).build().unwrap();

    let mut snake_pos: Point<f64> =
        Point {
            x: 200.0,
            y: 200.0
        };

    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g| {
            clear(BG_COLOR, g);
            rectangle(SNAKE_COLOR,
                      [snake_pos.x, snake_pos.y, 100.0, 100.0],
                      c.transform, g);
        });

        if let Some(Button::Keyboard(key)) = event.press_args() {
            println!("{:?}", key);
            match key {
                Key::Up => snake_pos.y -= 1.0,
                Key::Down => snake_pos.y += 1.0,
                Key::Left => snake_pos.x -= 1.0,
                Key::Right => snake_pos.x += 1.0,
                _ => {},
            }
        }
    }
}
