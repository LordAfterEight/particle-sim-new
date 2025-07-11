#![allow(static_mut_refs)]
#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use macroquad::input::show_mouse;
use macroquad::prelude::Conf;
use macroquad::rand::gen_range;
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


enum SelectedElement {

}


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
    let mut grid = render::Frame::new(
        (SCREEN_WIDTH as f32 / GRID_SCALING) as usize,
        ((SCREEN_HEIGHT as f32 / GRID_SCALING)) as usize,
        GRID_SCALING
    );
    println!("X: {} | Y: {}", grid.grid_size.0, grid.grid_size.1);
    let mut elements = Elements::init();
    let mut settings = settings::Settings::default();
    let mut cursor = cursor::Cursor::init();
    let mut pixels = grid.grid.clone();
    show_mouse(false);

    loop {
        cursor.update();
        let (mut pos_x, mut pos_y) = cursor.position;
        if pos_y >= SCREEN_HEIGHT { pos_y = SCREEN_HEIGHT - grid.grid_scaling as f32; }
        pos_x /= grid.grid_scaling as f32;
        pos_y /= grid.grid_scaling as f32;

        let shift_pressed = macroquad::input::is_key_down(macroquad::input::KeyCode::LeftShift);

        if macroquad::input::is_mouse_button_pressed(macroquad::input::MouseButton::Left) && !shift_pressed {
            if pos_y.floor() < SCREEN_HEIGHT && grid.grid[pos_x.floor() as usize][pos_y.floor() as usize] == None {
                grid.grid[pos_x.floor() as usize][pos_y.floor() as usize] =
                    Some(Rc::new(RefCell::new(Pixel::new(
                        0.0,
                        0.0,
                        &elements.sand,
                    ))));
            }
        }
        
        if macroquad::input::is_mouse_button_down(macroquad::input::MouseButton::Left) && shift_pressed {
            if grid.grid[pos_x.floor() as usize][pos_y.floor() as usize] == None {
                grid.grid[pos_x.floor() as usize][pos_y.floor() as usize] =
                    Some(Rc::new(RefCell::new(Pixel::new(
                        0.0,
                        0.0,
                        &elements.water,
                    ))));
            }
        }

        if macroquad::input::is_mouse_button_down(macroquad::input::MouseButton::Middle) {
            if grid.grid[pos_x.floor() as usize][pos_y.floor() as usize] == None {
                grid.grid[pos_x.floor() as usize][pos_y.floor() as usize] =
                    Some(Rc::new(RefCell::new(Pixel::new(
                        0.0,
                        0.0,
                        &elements.metal,
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

        grid.update(&settings);
        helpers::draw_info(&mut grid, &mut settings, &mut cursor, &elements);
        helpers::handle_input(&mut grid, &mut settings);
        next_frame().await;
    }
}
