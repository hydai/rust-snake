extern crate piston_window;

use piston_window::*;
use piston_window::types::Color;


mod snake;
use snake::Snake;

fn main() {

    // Create window.
    const WINDOW_SIZE: [u32; 2] = [600, 400];
    let mut window: PistonWindow =
        WindowSettings::new("Meow", WINDOW_SIZE)
            .exit_on_esc(true).build().unwrap();

    // Initialize snake at (100.0, 100.0).
    let mut snake: Snake = Snake::new(100.0, 100.0);

    // Game loop.
    const BG_COLOR: Color = [0.204, 0.596, 0.859, 1.0];
    while let Some(event) = window.next() {
        // Catch keyboard event.
        if let Some(Button::Keyboard(key)) = event.press_args() {
            // TODO: We need new method to control the direction of the snake.
            match key {
                Key::Up => /*snake_pos.y -= 1.0*/{},
                Key::Down => /*snake_pos.y += 1.0*/{},
                Key::Left => /*snake_pos.x -= 1.0*/{},
                Key::Right => /*snake_pos.x += 1.0*/{},
                _ => {},
            }
        }

        // Draw game frame.
        window.draw_2d(&event, |context, g2d| {
            // Draw background first.
            clear(BG_COLOR, g2d);
            // Draw the body of snake.
            snake.draw(&context, g2d);
        });
    }
}
