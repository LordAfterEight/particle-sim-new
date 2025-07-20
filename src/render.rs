use std::{cell::RefCell, rc::Rc};

use macroquad::rand::gen_range;
use macroquad::rand::ChooseRandom;

use crate::elements::{self, Element, StateOfMatter};

type Grid = Vec<Vec<Option<Rc<RefCell<Pixel>>>>>;

#[derive(PartialEq, Clone)]
pub struct Pixel {
    x_velocity: f32,
    y_velocity: f32,
    element: Box<Element>,
}

pub struct Frame {
    pub grid: Grid,
    pub grid_scaling: f32,
    pub grid_size: (usize, usize),
}

impl Pixel {
    pub fn new(x_vel: f32, y_vel: f32, elem: Element) -> Self {
        unsafe {
            crate::PIXEL_AMOUNT += 1;
        }
        Self {
            x_velocity: x_vel,
            y_velocity: y_vel,
            element: Box::new(elem),
        }
    }

    pub fn update(
        &mut self,
        grid: &mut Grid,
        grid_size: (usize, usize),
        position: (usize, usize),
    ) -> (usize, usize) {
        let (x, y) = position;
        let (size_x, size_y) = grid_size;
        let random = get_random_plus_minus_one();
        let random_x: i32 = x as i32 + random;
        self.y_velocity = self.element.weight;

        if self.element.lifetime < u16::MAX && self.element.lifetime > 0 {
            self.element.lifetime -= get_random_one_or_zero();
        }

        if self.element.lifetime == 0 && self.element.sub_element.is_some() {
            self.element = self.element.sub_element.clone().unwrap();
        }

        // TODO: Vertical velocity
        if (y + self.y_velocity as usize) < size_y {
            let a_x = ((x as i32) + random) as usize;
            let b_x = ((x as i32) - random) as usize;

            match x as f32 {
                crate::SCREEN_WIDTH => grid[x][y] = None,
                0.0 => grid[x][y] = None,
                _ => {}
            }

            match y as f32 {
                crate::SCREEN_HEIGHT => grid[x][y] = None,
                1.0 => {
                    grid[x][y] = None;
                    unsafe {
                        crate::PIXEL_AMOUNT -= 1;
                    }
                }
                _ => {}
            }

            // --- Check Pixel above, to the left and right ---

            if let Some(pixel_rc_refcell_r) = &grid[x + 1][y] {
                if let Some(pixel_rc_refcell_l) = &grid[x - 1][y] {
                    if let Some(pixel_rc_refcell_u) = &grid[x][(y as f32 - self.y_velocity) as usize] {
                        let pixel_r = pixel_rc_refcell_r.try_borrow(); // Borrow the pixel to access its fields
                        let pixel_l = pixel_rc_refcell_l.try_borrow(); // Borrow the pixel to access its fields
                        let pixel_u = pixel_rc_refcell_u.try_borrow(); // Borrow the pixel to access its fields
                        if pixel_r.is_ok() {
                            let pixel_r = pixel_r.unwrap();

                            if pixel_r.element.name == "Water" && self.element.name == "Burning Coal" {
                                self.element.color = macroquad::color::Color::new(
                                    gen_range(0.4, 0.43),
                                    gen_range(0.4, 0.43),
                                    gen_range(0.4, 0.43),
                                    1.0,
                                );
                                self.element.lifetime = u16::MAX;
                            }

                            if pixel_r.element.name == "Fire" && self.element.name == "Water" {
                                self.element = self.element.sub_element.clone().unwrap();
                            }

                            if pixel_r.element.name == "Fire" && self.element.name == "Coal" {
                                self.element.color = macroquad::color::Color::new(
                                    gen_range(0.2, 0.3),
                                    gen_range(0.1, 0.13),
                                    gen_range(0.1, 0.13),
                                    1.0,
                                );
                                self.element.lifetime = gen_range(20, 60);
                            }

                            if pixel_r.element.name == "Burning Coal" && self.element.name == "Coal" {
                                let random_number = gen_range(1, 2);
                                if random_number == 1 {
                                    self.element = self.element.sub_element.clone().unwrap();
                                }
                            }
                        }
                        if pixel_l.is_ok() {
                            let pixel_l = pixel_l.unwrap();

                            if pixel_l.element.name == "Water" && self.element.name == "Burning Coal" {
                                self.element.color = macroquad::color::Color::new(
                                    gen_range(0.4, 0.43),
                                    gen_range(0.4, 0.43),
                                    gen_range(0.4, 0.43),
                                    1.0,
                                );
                                self.element.lifetime = u16::MAX;
                            }

                            if pixel_l.element.name == "Fire" && self.element.name == "Water" {
                                self.element = self.element.sub_element.clone().unwrap();
                            }

                            if pixel_l.element.name == "Fire" && self.element.name == "Coal" {
                                self.element.color = macroquad::color::Color::new(
                                    gen_range(0.2, 0.3),
                                    gen_range(0.1, 0.13),
                                    gen_range(0.1, 0.13),
                                    1.0,
                                );
                                self.element.lifetime = gen_range(20, 60);
                            }

                            if pixel_l.element.name == "Burning Coal" && self.element.name == "Coal" {
                                let random_number = gen_range(1, 2);
                                if random_number == 1 {
                                    self.element = self.element.sub_element.clone().unwrap();
                                }
                            }
                        }
                        if pixel_u.is_ok() {
                            let pixel_u = pixel_u.unwrap();
                            if pixel_u.element.name == "Water" && self.element.name == "Burning Coal" {
                                self.element.color = macroquad::color::Color::new(
                                    gen_range(0.4, 0.43),
                                    gen_range(0.4, 0.43),
                                    gen_range(0.4, 0.43),
                                    1.0,
                                );
                                self.element.lifetime = u16::MAX;
                            }
                            if pixel_u.element.name == "Fire" && self.element.name == "Water" {
                                self.element = self.element.sub_element.clone().unwrap();
                            }

                            if pixel_u.element.state == StateOfMatter::Powder &&
                                ((self.element.state == StateOfMatter::Gas) ||
                                (self.element.state == StateOfMatter::Liquid))
                            {
                                for i in 0..100 {
                                    if grid[x][y - i].is_none() {
                                        return (x, y - i - 1);
                                    }
                                }
                            }

                            if pixel_u.element.name == "Fire" && self.element.name == "Coal" {
                                self.element.color = macroquad::color::Color::new(
                                    gen_range(0.2, 0.3),
                                    gen_range(0.1, 0.13),
                                    gen_range(0.1, 0.13),
                                    1.0,
                                );
                                self.element.lifetime = gen_range(20, 60);
                            }

                            if pixel_u.element.name == "Burning Coal" && self.element.name == "Coal" {
                                let random_number = gen_range(1, 2);
                                if random_number == 1 {
                                    self.element = self.element.sub_element.clone().unwrap();
                                }
                            }
                        }
                    }
                }
            }

            // --- Element Specific Behaviour

            if self.element.name == "Fire" {
                self.element.color = macroquad::color::Color::new(
                    1.0,
                    self.element.lifetime as f32 / 200.0 * 3.0,
                    0.0,
                    1.0
                );
            }

            // --- State Of Matter Specific Behaviour

            match self.element.state {
                // --- Powder ---
                StateOfMatter::Powder => {
                    if grid[position.0][position.1 + self.y_velocity as usize].is_none() {
                        return (x, y + self.y_velocity as usize);
                    } else {
                        self.y_velocity = 1.0;
                    }
                    let a_side_free = a_x > 0
                        && a_x < grid_size.0
                        && grid[a_x][y + self.y_velocity as usize].is_none();
                    &&grid[x + 1][y].is_none();
                    let b_side_free = b_x > 0
                        && b_x < grid_size.0
                        && grid[b_x][y + self.y_velocity as usize].is_none();
                    &&grid[x - 1][y].is_none();

                    if a_side_free {
                        return (a_x, y + self.y_velocity as usize);
                    }

                    if b_side_free {
                        return (b_x, y + self.y_velocity as usize);
                    }
                }

                // --- Liquid ---
                StateOfMatter::Liquid => {
                    if grid[x][y + self.y_velocity as usize].is_none() {
                        return (x, y + self.y_velocity as usize);
                    }

                    match random {
                        1 => {
                            if (x + 1) < size_x
                                && (grid[x + 1][y].is_none() && grid[x + 1][y - 1].is_none())
                            {
                                return (x + 1, y);
                            }
                        }
                        -1 => {
                            if (x - 1) > 0
                                && (grid[x - 1][y].is_none() && grid[x - 1][y - 1].is_none())
                            {
                                return (x - 1, y);
                            }
                        }
                        0 => return (x, y),
                        _ => return (x, y),
                    }
                }

                // --- Gas ---
                StateOfMatter::Gas => {
                    if grid[x][(y as f32 + self.y_velocity) as usize].is_none() {
                        let (mut ret_x, ret_y) = (x, y);
                        match random {
                            1 => {
                                if (x + 1) < size_x && grid[x + 1][y].is_none() {
                                    ret_x += 1;
                                }
                            }
                            -1 => {
                                if (x - 1) > 0 && grid[x - 1][y].is_none() {
                                    ret_x -= 1;
                                }
                            }
                            _ => {}
                        }
                        return (ret_x, (y as f32 + self.y_velocity) as usize);
                    }

                    match random {
                        1 => {
                            if (x + 1) < size_x && grid[x + 1][y].is_none() {
                                return (x + 1, y);
                            }
                        }
                        -1 => {
                            if (x - 1) > 0 && grid[x - 1][y].is_none() {
                                return (x - 1, y);
                            }
                        }
                        0 => return (x, y),
                        _ => return (x, y),
                    }
                }
                _ => {}
            }
        }
        return (x, y);
    }
}

impl Frame {
    pub fn new(size_x: usize, size_y: usize, scaling: f32) -> Self {
        Self {
            grid: create_grid(size_x, size_y),
            grid_scaling: scaling,
            grid_size: (size_x, size_y),
        }
    }

    pub fn update(&mut self, settings: &crate::settings::Settings) {
        let mut new_grid = create_grid(self.grid_size.0, self.grid_size.1);

        for x in 0..self.grid_size.0 {
            for y in 0..self.grid_size.1 {
                if let Some(pixel) = self.grid[x][y].as_ref().map(|p| p.clone()) {
                    let mut pixel_ref = pixel.borrow_mut();

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

                    // --- Create a new Grid ---
                    let new_pos = pixel_ref.update(&mut self.grid, self.grid_size, (x, y));
                    new_grid[new_pos.0][new_pos.1] = Some(pixel.clone());
                }
            }
        }

        if settings.pause_state == false
            || macroquad::input::is_key_pressed(macroquad::input::KeyCode::F)
        {
            self.grid = new_grid;
        }
    }
}

#[inline]
pub fn create_grid(width: usize, height: usize) -> Grid {
    vec![vec![None; height]; width]
}

pub fn get_random_plus_minus_one() -> i32 {
    let nums = [1, 0, -1];
    *nums.choose().unwrap()
}

pub fn get_random_one_or_zero() -> u16 {
    let nums = [1, 0];
    *nums.choose().unwrap()
}
