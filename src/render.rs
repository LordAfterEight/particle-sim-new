use std::{cell::RefCell, rc::Rc};


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

        if y + 1 < size_y && grid[x][y + 1].is_none() {
            return (x, y + 1);
        }
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

    pub fn update(&mut self, cursor_x: f32, cursor_y: f32) {
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
