use crate::elements::Element;
use crate::{SCREEN_WIDTH,SCREEN_HEIGHT};

#[derive(Clone)]
pub struct Pixel {
    x: f32,
    y: f32,
    x_velocity: f32,
    y_velocity: f32,
    element: Element,
}

pub struct Frame {
    pub grid: Vec<Vec<Option<Box<Pixel>>>>,
    pub grid_scaling: u16,
}

pub struct Diagnostics {
    pixel_amount: u64,
    grid_size: usize,
}

impl Pixel {
    pub fn new(x: f32, y: f32, x_vel: f32, y_vel: f32, elem: &Element) -> Self {
        Self {
            x,
            y,
            x_velocity: x_vel,
            y_velocity: y_vel,
            element: elem.clone()
        }
    }

    pub fn update(&mut self) {
        macroquad::prelude::draw_rectangle(self.x, self.y, 1.0, 1.0, self.element.color);

        println!("{} at X: {} | Y: {}", self.element.name, self.x, self.y);

        self.y_velocity += self.element.weight;

        self.y += self.y_velocity;
    }
}

impl Frame {
    pub fn new() -> Self {
        Self {
            grid: vec![vec![None; SCREEN_HEIGHT as usize]; SCREEN_WIDTH as usize],
            grid_scaling: 1
        }
    }

    pub fn update(&mut self) {
        for pixelx in &mut self.grid {
            for pixel in pixelx {
                if let Some(pixel) = pixel {
                    pixel.update();
                }
            }
        }
    }
}
