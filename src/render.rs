use crate::elements::Element;

#[derive(PartialEq, Clone)]
pub struct Pixel<'a> {
    x: f32,
    y: f32,
    x_velocity: f32,
    y_velocity: f32,
    element: &'a Element,
}

pub struct Frame<'a> {
    pub grid: Vec<Vec<Option<Box<Pixel<'a>>>>>,
    pub grid_scaling: f32,
}

impl <'a>Pixel<'a> {
    pub fn new(x: f32, y: f32, x_vel: f32, y_vel: f32, elem: &'a Element) -> Self {
        unsafe { crate::PIXEL_AMOUNT += 1; }
        Self {
            x,
            y,
            x_velocity: x_vel,
            y_velocity: y_vel,
            element: elem,
        }
    }

    pub fn update(&mut self) {
        // TODO: Rendering

        // TODO: Vertical velocity

        // TODO: Horizontal velocity

        // TODO: Element-specific modulations
    }
}

impl Frame<'_> {
    pub fn new(size_x: usize, size_y: usize) -> Self {
        Self {
            grid: vec![vec![None; size_y]; size_x],
            grid_scaling: 8.0
        }
    }

    pub fn update(&mut self) {
    }
}
