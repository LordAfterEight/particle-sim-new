#![allow(static_mut_refs)]
#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use macroquad::prelude::Conf;
use macroquad::window::next_frame;

use crate::elements::Elements;
use crate::render::Pixel;

mod cursor;
mod elements;
mod helpers;
mod render;
mod settings;

static SCREEN_WIDTH: f32 = 1440.0;
static SCREEN_HEIGHT: f32 = 810.0;
static SCALING: f32 = 1.0;
pub const GRID_SCALING: f32 = 8.0;
pub static mut PIXEL_AMOUNT: usize = 0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rusty Pixels".to_owned(),
        window_width: (SCREEN_WIDTH / SCALING) as i32,
        window_height: (SCREEN_HEIGHT / SCALING) as i32,
        window_resizable: false,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    let mut grid = render::Frame::new(
        (SCREEN_WIDTH as f32 / GRID_SCALING) as usize,
        (SCREEN_HEIGHT as f32 / GRID_SCALING) as usize,
        GRID_SCALING
    );
    println!("X: {} | Y: {}", grid.grid_size.0, grid.grid_size.1);
    let elements = Elements::init();
    let mut settings = settings::Settings::default();
    let mut cursor = cursor::Cursor::init();

    loop {
        cursor.update();
        let (mut pos_x, mut pos_y) = cursor.position;
        pos_x /= grid.grid_scaling as f32;
        pos_y /= grid.grid_scaling as f32;

        if macroquad::input::is_mouse_button_down(macroquad::input::MouseButton::Left) {
            if grid.grid[pos_x.floor() as usize][pos_y.floor() as usize] == None {
                grid.grid[pos_x.floor() as usize][pos_y.floor() as usize] =
                    Some(Rc::new(RefCell::new(Pixel::new(
                        /*pos_x as f32 * grid.grid_scaling,
                        pos_y as f32 * grid.grid_scaling,*/
                        0.0,
                        0.0,
                        &elements.sand,
                    ))));
            }
        }

        if macroquad::input::is_mouse_button_down(macroquad::input::MouseButton::Right) {
            if grid.grid[pos_x.floor() as usize][pos_y.floor() as usize] != None {
                grid.grid[pos_x.floor() as usize][pos_y.floor() as usize] = None;
                unsafe {
                    PIXEL_AMOUNT -= 1;
                }
            }
        }

        helpers::draw_info(&mut grid, &mut settings, &mut cursor);
        helpers::handle_input(&mut grid, &mut settings);
        grid.update(pos_x.floor(), pos_y.floor());
        next_frame().await;
    }
}
