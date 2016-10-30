use std::collections::LinkedList;

use piston_window::*;
use piston_window::types::Color;

const SNAKE_COLOR: Color = [0.792, 0.894, 0.965, 1.0];
const SNAKE_BODY_SIZE: f64 = 50.0;

enum Direction {
    Up, Down, Left, Right
}

struct Point {
    x: f64,
    y: f64
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Point>
}

impl Snake {
    // Constructor
    pub fn new(init_x: f64, init_y: f64) -> Snake {
        // Create the body of snake.
        let mut body: LinkedList<Point> = LinkedList::new();
        body.push_back(Point {
            x: init_x,
            y: init_y
        });

        Snake {
            body: body,
            direction: Direction::Right
        }
    }

    pub fn draw(&self, context: &Context, g2d: &mut G2d) {
        for snake_pos in &self.body {
            rectangle(SNAKE_COLOR,
                      [snake_pos.x,
                      snake_pos.y,
                      SNAKE_BODY_SIZE,
                      SNAKE_BODY_SIZE],
                      context.transform, g2d);
        }
    }
}
