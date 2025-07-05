use macroquad::window::next_frame;
use macroquad::prelude::Conf;

mod elements;
mod render;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rusty Pixels".to_owned(),
        window_width: 1440,
        window_height: 810,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    let grid = render::Frame::new();
    loop {
        next_frame().await;
    }
}
