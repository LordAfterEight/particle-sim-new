use macroquad;

pub struct Element {
    name: String,
    color: macroquad::color::Color,
    weight: f32,
    sub_element: SubElement
}

pub struct SubElement {
    name: String,
    color: macroquad::color::Color,
    weight: f32,
}

impl Element {
    pub fn new(
        name: &str,
        color: macroquad::color::Color,
        weight: f32,
        sub_element: SubElement
    ) -> Self {
        Self {
            name: name.to_string(),
            color,
            weight,
            sub_element
        }
    }

    pub fn none() -> Self {
        Self {
            name: "None".to_string(),
            color: macroquad::color::BLACK,
            weight: 0.0,
            sub_element: SubElement::none()
        }
    }
}

impl SubElement {
    pub fn new(
        name: &str,
        color: macroquad::color::Color,
        weight: f32,
    ) -> Self {
        Self {
            name: name.to_string(),
            color,
            weight,
        }
    }

    pub fn none() -> Self {
        Self {
            name: "None".to_string(),
            color: macroquad::color::BLACK,
            weight: 0.0,
        }
    }
}
