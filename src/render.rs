use crate::elements::Element;

pub struct Pixel {
    x: u16,
    y: u16,
    element: Element,
}

pub struct Frame {
    grid: Vec<Vec<Pixel>>,
    grid_scaling: u16,
}

pub struct Diagnostics {
    pixel_amount: u64,
    grid_size: usize,
}

impl Pixel {
    pub fn empty() -> Self {
        Self {
            x: 0,
            y: 0,
            element: Element::none()
        }
    }
}

impl Frame {
    pub fn new() -> Self {
        Self {
            grid: vec![vec![Pixel::empty()]],
            grid_scaling: 1
        }
    }
}
