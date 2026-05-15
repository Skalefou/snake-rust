use macroquad::color::{DARKGREEN, GREEN};
use macroquad::math::Vec2;
use macroquad::shapes::draw_rectangle;
use crate::conf::{CASE_SIZE, GRID_WIDTH, GRID_HEIGHT};

#[derive(PartialEq)]
pub enum Direction {
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

    pub fn move_snake(&mut self) {
        for i in (1..self.body_position.len()).rev() {
            self.body_position[i] = self.body_position[i-1];
        }

        let move_direction = match self.direction {
            Direction::UP => Vec2::new(0.0, -1.0),
            Direction::DOWN => Vec2::new(0.0, 1.0),
            Direction::LEFT => Vec2::new(-1.0, 0.0),
            Direction::RIGHT => Vec2::new(1.0, 0.0),
        };

        self.body_position[0] += move_direction;
    }

    pub fn change_direction(&mut self, new_direction: Direction) {
        let invalid = match new_direction {
            Direction::UP => self.direction == Direction::DOWN,
            Direction::DOWN => self.direction == Direction::UP,
            Direction::LEFT => self.direction == Direction::RIGHT,
            Direction::RIGHT => self.direction == Direction::LEFT,
        };

        if !invalid {
            self.direction = new_direction;
        }
    }
}