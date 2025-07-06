#![allow(static_mut_refs)]

use macroquad::window::next_frame;
use macroquad::prelude::Conf;

use crate::elements::Elements;
use crate::render::Pixel;

mod elements;
mod render;
mod helpers;

static SCREEN_WIDTH: i32 = 1440;
static SCREEN_HEIGHT: i32 = 810;
pub static mut PIXEL_AMOUNT: u64 = 0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rusty Pixels".to_owned(),
        window_width: SCREEN_WIDTH,
        window_height: SCREEN_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    let mut grid = render::Frame::new();
    let fullscreen = false;
    let elements = Elements::init();

    loop {
        helpers::draw_diagnostics();
        helpers::handle_input(fullscreen);

        grid.grid[400][200] = Some(Box::new(Pixel::new(400.0,200.0,0.0,0.0,&elements.fire)));

        grid.update();
        next_frame().await;
    }
}

