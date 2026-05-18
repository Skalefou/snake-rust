pub mod conf;
pub mod snake;
pub mod apple;

use macroquad::prelude::*;
use conf::window_conf;
use snake::*;
use crate::apple::Apple;
use crate::conf::SPEED_GAME;

enum GameState {
    RUNNING,
    GAMEOVER,
}

#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as u64);
    let mut snake: Snake = Default::default();
    let mut apple: Apple = Apple::new(snake.get_body_position());
    let mut timer: f64 = 0.0;
    let mut game_state = GameState::RUNNING;

    loop {
        clear_background(DARKGRAY);
        match game_state {
            GameState::RUNNING => {
                timer += get_frame_time() as f64;
                if timer >= SPEED_GAME {
                    snake.move_snake();
                    timer = 0.0;
                }

                if is_key_pressed(KeyCode::Escape) {
                    break;
                }

                if is_key_pressed(KeyCode::Up) { snake.change_direction(Direction::UP)}
                if is_key_pressed(KeyCode::Down) { snake.change_direction(Direction::DOWN)}
                if is_key_pressed(KeyCode::Left) { snake.change_direction(Direction::LEFT)}
                if is_key_pressed(KeyCode::Right) { snake.change_direction(Direction::RIGHT)}

                if snake.is_eating(&apple.get_position()) {
                    snake.eat();
                    apple.respawn(snake.get_body_position());
                }

                if snake.is_game_over() {
                    game_state = GameState::GAMEOVER;
                }
            },

            GameState::GAMEOVER => {

            }
        }

        draw_text(
            &format!("Score : {}", snake.get_score()),
            10.0,
            24.0,
            32.0,
            WHITE,
        );

        apple.draw();
        snake.draw();

        next_frame().await;
    }
}
