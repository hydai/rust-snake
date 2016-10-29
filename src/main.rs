extern crate piston_window;

use piston_window::*;

fn main() {
    println!("Hello, world!");
    let mut window: PistonWindow =
        WindowSettings::new("Meow", [600, 400])
            .exit_on_esc(true).build().unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0],
                      c.transform, g);
        });
    }
}
