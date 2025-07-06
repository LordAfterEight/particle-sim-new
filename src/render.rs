use std::{cell::RefCell, rc::Rc};

use macroquad::rand::ChooseRandom;

use crate::{
    SCREEN_HEIGHT,
    elements::{Element, StateOfMatter},
};

type Grid<'a> = Vec<Vec<Option<Rc<RefCell<Pixel<'a>>>>>>;

#[derive(PartialEq, Clone)]
pub struct Pixel<'a> {
    x_velocity: f32,
    y_velocity: f32,
    element: &'a Element,
}

pub struct Frame<'a> {
    pub grid: Grid<'a>,
    pub grid_scaling: f32,
    pub grid_size: (usize, usize),
}

impl<'a> Pixel<'a> {
    pub fn new(x_vel: f32, y_vel: f32, elem: &'a Element) -> Self {
        unsafe {
            crate::PIXEL_AMOUNT += 1;
        }
        Self {
            x_velocity: x_vel,
            y_velocity: y_vel,
            element: elem,
        }
    }

    pub fn update(
        &mut self,
        grid: &Grid,
        grid_scaling: f32,
        grid_size: (usize, usize),
        position: (usize, usize),
    ) -> (usize, usize) {
        let (x, y) = position;
        let (_size_x, size_y) = grid_size;
        self.y_velocity = 1.0;

        // TODO: Vertical velocity
        if (y + self.y_velocity as usize) < size_y {
            let random = get_random_plus_minus_one();
            let a_x = ((x as i32) + random) as usize;
            let b_x = ((x as i32) - random) as usize;

            match self.element.state {
                StateOfMatter::Powder => {
                    if grid[x][y + self.y_velocity as usize].is_none() {
                        return (x, y + self.y_velocity as usize);
                    }
                    let a_side_free = a_x > 0
                        && a_x < grid_size.0
                        && grid[a_x][y + self.y_velocity as usize].is_none();
                    let b_side_free = b_x > 0
                        && b_x < grid_size.0
                        && grid[b_x][y + self.y_velocity as usize].is_none();

                    if a_side_free {
                        return (a_x, y + self.y_velocity as usize);
                    }

                    if b_side_free {
                        return (b_x, y + self.y_velocity as usize);
                    }
                }
                _ => {}
            }
        }

        // TODO: Horizontal velocity

        return (x, y);
    }
}

impl Frame<'_> {
    pub fn new(size_x: usize, size_y: usize, scaling: f32) -> Self {
        Self {
            grid: create_grid(size_x, size_y),
            grid_scaling: scaling,
            grid_size: (size_x, size_y),
        }
    }

    pub fn update(&mut self) {
        let mut new_grid = create_grid(self.grid_size.0, self.grid_size.1);

        for x in 0..self.grid_size.0 {
            for y in 0..self.grid_size.1 {
                if let Some(pixel) = self.grid[x][y].as_ref().map(|p| p.clone()) {
                    let mut pixel_ref = pixel.borrow_mut();
                    let new_pos =
                        pixel_ref.update(&self.grid, self.grid_scaling, self.grid_size, (x, y));

                    // TODO: Rendering things
                    macroquad::shapes::draw_rectangle(
                        x as f32 * self.grid_scaling,
                        y as f32 * self.grid_scaling,
                        self.grid_scaling,
                        self.grid_scaling,
                        pixel_ref.element.color,
                    );

                    // TODO: State of Matter specific behaviour

                    // TODO: Element specific behaviour
                    new_grid[new_pos.0][new_pos.1] = Some(pixel.clone());
                }
            }
        }

        self.grid = new_grid;
    }
}

#[inline]
pub fn create_grid<'a>(width: usize, height: usize) -> Grid<'a> {
    vec![vec![None; height]; width]
}

pub fn get_random_plus_minus_one() -> i32 {
    let nums = [1, -1];
    *nums.choose().unwrap()
}
