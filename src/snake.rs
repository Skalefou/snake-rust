use macroquad::color::{DARKGREEN, GREEN};
use macroquad::math::{IVec2};
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
    body_position: Vec<IVec2>,
    direction: Direction,
    score: u32,
}

impl Default for Snake {
    fn default () -> Self {
        Snake {
            body_position: vec![IVec2::new(12,12)],
            direction: Direction::RIGHT,
            score: 0
        }
    }
}

impl Snake {
    pub fn draw(&self) {
        for (i, part) in self.body_position.iter().enumerate() {
            let color = if i == 0 { GREEN } else { DARKGREEN };

            draw_rectangle(
                part.x as f32 * CASE_SIZE,
                part.y as f32 * CASE_SIZE,
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
            Direction::UP => IVec2::new(0, -1),
            Direction::DOWN => IVec2::new(0, 1),
            Direction::LEFT => IVec2::new(-1, 0),
            Direction::RIGHT => IVec2::new(1, 0),
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

    pub fn is_eating(&self, apple_position: &IVec2) -> bool {
        self.body_position[0] == *apple_position
    }

    pub fn eat(&mut self) {
        let last = self.body_position.last().unwrap();
        self.body_position.push(*last);
        self.score += 1;
    }

    pub fn get_body_position(&self) -> &Vec<IVec2> {
        &self.body_position
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn is_game_over(&self) -> bool {
        let out_of_bound =  self.body_position[0].x < 0 ||
            self.body_position[0].y >= GRID_WIDTH as i32 ||
            self.body_position[0].y >= GRID_HEIGHT as i32 ||
            self.body_position[1].x < 0;

        let self_collision = self.body_position[1..].contains(&self.body_position[0]);

        self_collision || out_of_bound
    }
}