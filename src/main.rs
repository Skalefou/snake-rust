pub mod conf;
pub mod snake;

use macroquad::prelude::*;
use conf::window_conf;
use snake::Snake;

#[macroquad::main(window_conf)]
async fn main() {
    let snake: Snake = Default::default();
    loop {
        clear_background(DARKGRAY);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        snake.draw();

        next_frame().await;
    }
}
