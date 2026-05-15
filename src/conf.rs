use macroquad::prelude::Conf;

pub fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Snake Rust"),
        window_width: 800,
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}