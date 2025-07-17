#![allow(static_mut_refs)]
#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use macroquad::input::show_mouse;
use macroquad::prelude::Conf;
use macroquad::window::next_frame;
use macroquad::rand::gen_range;

use crate::elements::Elements;
use crate::render::Pixel;

mod cursor;
mod elements;
mod helpers;
mod render;
mod settings;

const SCREEN_WIDTH: f32 = 1440.0;
const SCREEN_HEIGHT: f32 = 810.0;
static SCALING: f32 = 1.0;
pub const GRID_SCALING: f32 = 2.0;
pub static mut PIXEL_AMOUNT: usize = 0;

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
    let mut settings = settings::Settings::default();
    let mut cursor = cursor::Cursor::init();
    let pixels = grid.grid.clone();
    show_mouse(false);

    loop {
        let mut elements = Elements::init();
        cursor.update();
        let (mut pos_x, mut pos_y) = cursor.position;
        if pos_y >= SCREEN_HEIGHT { pos_y = SCREEN_HEIGHT - grid.grid_scaling as f32; }
        pos_x /= grid.grid_scaling as f32;
        pos_y /= grid.grid_scaling as f32;

        let shift_pressed = macroquad::input::is_key_down(macroquad::input::KeyCode::LeftShift);

        if macroquad::input::is_mouse_button_down(macroquad::input::MouseButton::Left) && !shift_pressed {
            for x in 0..cursor.size_modifier as u16 {
                for y in 0..cursor.size_modifier as u16 {
                    elements = Elements::init();
                    if (y as f32) < SCREEN_HEIGHT && grid.grid[(pos_x as usize - (cursor.size_modifier as usize / 2)) + x as usize][(pos_y as usize - (cursor.size_modifier as usize / 2)) + y as usize].is_none() {
                        grid.grid[(pos_x as usize - (cursor.size_modifier as usize / 2)) + x as usize][(pos_y as usize - (cursor.size_modifier as usize / 2)) + y as usize] =
                            Some(Rc::new(RefCell::new(Pixel::new(
                                0.0,
                                0.0,
                                elements.coal.clone(),
                            ))));
                    }
                }
            }
        }
        
        if macroquad::input::is_mouse_button_down(macroquad::input::MouseButton::Left) && shift_pressed {
            for x in 0..cursor.size_modifier as u16 {
                for y in 0..cursor.size_modifier as u16 {
                    elements = Elements::init();
                    if (y as f32) < SCREEN_HEIGHT && grid.grid[(pos_x as usize - (cursor.size_modifier as usize / 2)) + x as usize][(pos_y as usize - (cursor.size_modifier as usize / 2)) + y as usize].is_none() {
                        let mut water = elements.water.clone();
                        water.color = macroquad::color::Color::new(
                            0.2,
                            0.2,
                            gen_range(0.7,0.8),
                            1.0
                        );
                        grid.grid[(pos_x as usize - (cursor.size_modifier as usize / 2)) + x as usize][(pos_y as usize - (cursor.size_modifier as usize / 2)) + y as usize] =
                            Some(Rc::new(RefCell::new(Pixel::new(
                                0.0,
                                0.0,
                                elements.water.clone(),
                            ))));
                    }
                }
            }
        }
        
        if macroquad::input::is_mouse_button_down(macroquad::input::MouseButton::Middle) && !shift_pressed {
            for x in 0..cursor.size_modifier as u16 {
                for y in 0..cursor.size_modifier as u16 {
                    elements = Elements::init();
                    if (y as f32) < SCREEN_HEIGHT && grid.grid[(pos_x as usize - (cursor.size_modifier as usize / 2)) + x as usize][(pos_y as usize - (cursor.size_modifier as usize / 2)) + y as usize].is_none() {
                        grid.grid[(pos_x as usize - (cursor.size_modifier as usize / 2)) + x as usize][(pos_y as usize - (cursor.size_modifier as usize / 2)) + y as usize] =
                            Some(Rc::new(RefCell::new(Pixel::new(
                                0.0,
                                0.0,
                                elements.metal.clone(),
                            ))));
                    }
                }
            }
        }

        if macroquad::input::is_mouse_button_down(macroquad::input::MouseButton::Middle) && shift_pressed {
            for x in 0..cursor.size_modifier as u16 {
                for y in 0..cursor.size_modifier as u16 {
                    elements = Elements::init();
                    if (y as f32) < SCREEN_HEIGHT && grid.grid[(pos_x as usize - (cursor.size_modifier as usize / 2)) + x as usize][(pos_y as usize - (cursor.size_modifier as usize / 2)) + y as usize].is_none() {
                        grid.grid[(pos_x as usize - (cursor.size_modifier as usize / 2)) + x as usize][(pos_y as usize - (cursor.size_modifier as usize / 2)) + y as usize] =
                            Some(Rc::new(RefCell::new(Pixel::new(
                                0.0,
                                0.0,
                                elements.fire.clone(),
                            ))));
                    }
                }
            }
        }
        if macroquad::input::is_mouse_button_down(macroquad::input::MouseButton::Right) && !shift_pressed {
            for x in 0..cursor.size_modifier as u16 {
                for y in 0..cursor.size_modifier as u16 {
                    if (y as f32) < SCREEN_HEIGHT && grid.grid[(pos_x as usize - (cursor.size_modifier as usize / 2)) + x as usize][(pos_y as usize - (cursor.size_modifier as usize / 2)) + y as usize].is_some() {
                        grid.grid[(pos_x as usize - (cursor.size_modifier as usize / 2)) + x as usize][(pos_y as usize - (cursor.size_modifier as usize / 2)) + y as usize] = None;
                        unsafe { PIXEL_AMOUNT -= 1; }
                    }
                }
            }
        }

        grid.update(&settings);
        helpers::draw_info(&mut grid, &mut settings, &mut cursor, &elements);
        helpers::handle_input(&mut grid, &mut settings, &mut cursor);
        next_frame().await;
    }
}
