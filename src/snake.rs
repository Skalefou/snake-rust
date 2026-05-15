use macroquad::color::{DARKGREEN, GREEN};
use macroquad::math::Vec2;
use macroquad::shapes::draw_rectangle;
use crate::conf::{CASE_SIZE, GRID_WIDTH, GRID_HEIGHT};

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

pub struct Snake {
    body_position: Vec<Vec2>,
    direction: Direction
}

impl Default for Snake {
    fn default () -> Self {
        Snake {
            body_position: vec![Vec2::new(12.0,12.0)],
            direction: Direction::RIGHT
        }
    }
}

impl Snake {
    pub fn draw(&self) {
        for (i, part) in self.body_position.iter().enumerate() {
            let color = if i == 1 { GREEN } else { DARKGREEN };

            draw_rectangle(
                part.x * CASE_SIZE,
                part.y * CASE_SIZE,
                CASE_SIZE,
                CASE_SIZE,
                color,
            )
        }
    }
}