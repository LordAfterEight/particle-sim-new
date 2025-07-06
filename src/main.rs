#![allow(static_mut_refs)]
#![allow(dead_code)]

use macroquad::window::next_frame;
use macroquad::prelude::Conf;

use crate::elements::Elements;
use crate::render::Pixel;

mod elements;
mod render;
mod helpers;
mod settings;
mod cursor;

static SCREEN_WIDTH: f32 = 1440.0;
static SCREEN_HEIGHT: f32 = 810.0;
static SCALING: f32 = 1.0;
pub static mut PIXEL_AMOUNT: u64 = 0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rusty Pixels".to_owned(),
        window_width: (SCREEN_WIDTH / SCALING) as i32,
        window_height: (SCREEN_HEIGHT / SCALING) as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    let mut grid = render::Frame::new((SCREEN_WIDTH as f32 * SCALING) as usize, (SCREEN_HEIGHT as f32 * SCALING) as usize);
    let elements = Elements::init();
    let mut settings = settings::Settings::default();
    let mut cursor = cursor::Cursor::init();

    loop {
        cursor.update();

        macroquad::prelude::draw_rectangle(
            0.0, 0.0,
            grid.grid.len() as f32,
            grid.grid[0].len() as f32,
            macroquad::color::Color::new(0.025,0.025,0.025,1.0)
        );

        helpers::draw_info(&mut grid, &mut settings, &mut cursor);
        helpers::handle_input(&mut grid, &mut settings);

        if macroquad::input::is_mouse_button_down(macroquad::input::MouseButton::Left) {
            let (mut pos_x, mut pos_y) = cursor.position;
            pos_x /= grid.grid_scaling as f32;
            pos_y /= grid.grid_scaling as f32;
            grid.grid[pos_x as usize][pos_y as usize] = Some(Box::new(Pixel::new(pos_x as f32, pos_y as f32,0.0,0.0,&elements.fire)));
        }

        grid.update();
        next_frame().await;
    }
}

