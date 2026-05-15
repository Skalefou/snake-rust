mod conf;
use macroquad::prelude::*;
use conf::window_conf;

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(GREEN);
        next_frame().await;
    }
}
