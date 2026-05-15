use macroquad::prelude::Conf;

pub const CASE_SIZE: f32 = 32.0;
pub const GRID_WIDTH: f32 = 25.0;
pub const GRID_HEIGHT: f32 = 25.0;
pub const SPEED_GAME: f64= 0.125;

pub fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Snake Rust"),
        window_width: 800,
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}