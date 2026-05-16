use macroquad::color::RED;
use macroquad::math::IVec2;
use macroquad::prelude::{draw_rectangle, rand};
use crate::conf::{GRID_WIDTH, GRID_HEIGHT, CASE_SIZE};

pub struct Apple {
    position: IVec2
}

impl Apple {
    pub fn new(snake_body: &Vec<IVec2>) -> Self {
        let mut apple = Apple {
            position: IVec2::ZERO
        };
        apple.respawn(snake_body);
        apple
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.position.x as f32 * CASE_SIZE,
            self.position.y as f32 * CASE_SIZE,
            CASE_SIZE,
            CASE_SIZE,
            RED,
        )
    }

    fn random_position() -> IVec2 {
        IVec2::new(
            rand::gen_range(0i32, GRID_WIDTH as i32),
            rand::gen_range(0i32, GRID_HEIGHT as i32),
        )
    }

    pub fn respawn(&mut self, snake_body: &Vec<IVec2>) {
        loop {
            let new_position = Self::random_position();

            if !snake_body.contains(&new_position) {
                self.position = new_position;
                break;
            }
        }
    }
}