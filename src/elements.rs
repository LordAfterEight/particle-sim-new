use macroquad;

#[derive(PartialEq, Clone)]
pub enum StateOfMatter {
    Solid,
    Liquid,
    Gas,
    Powder
}

#[derive(PartialEq)]
pub struct Element {
    pub name: String,
    pub color: macroquad::color::Color,
    pub weight: f32,
    pub sub_element: SubElement,
    pub state: StateOfMatter,
    pub lifetime: u16
}

#[derive(PartialEq, Clone)]
pub struct SubElement {
    pub name: String,
    pub color: macroquad::color::Color,
    pub weight: f32,
    pub state: StateOfMatter,
    pub lifetime: u16
}

pub struct Elements {
    pub fire: Element,
    pub smoke: SubElement,
    pub sand: SubElement,
    pub metal: SubElement,
}

impl Element {
    pub fn new(
        name: &str,
        color: macroquad::color::Color,
        weight: f32,
        sub_element: SubElement,
        state: StateOfMatter,
        lifetime: u16
    ) -> Self {
        Self {
            name: name.to_string(),
            color,
            weight,
            sub_element,
            state,
            lifetime
        }
    }
}

impl SubElement {
    pub fn new(
        name: &str,
        color: macroquad::color::Color,
        weight: f32,
        state: StateOfMatter,
        lifetime: u16
    ) -> Self {
        Self {
            name: name.to_string(),
            color,
            weight,
            state,
            lifetime
        }
    }
}

impl Elements {
    pub fn init() -> Self {
        let smoke = SubElement::new(
            "Smoke",
            macroquad::color::Color::new(0.2,0.2,0.2,0.1),
            -0.01,
            StateOfMatter::Gas,
            30
        );
        let fire = Element::new(
            "Fire",
            macroquad::color::Color::new(1.0,0.75,0.0,0.9),
            -0.25,
            smoke.clone(),
            StateOfMatter::Gas,
            30
        );
        let sand = SubElement::new(     // TODO: Change this to 'Element' later and add lava 'SubElement'
            "Sand",
            macroquad::color::Color::new(0.8,0.8,0.6,1.0),
            1.0,
            StateOfMatter::Solid,
            u16::MAX
        );
        let metal = SubElement::new(
            "Metal",
            macroquad::color::Color::new(0.5,0.5,0.6,1.0),
            0.0,
            StateOfMatter::Solid,
            u16::MAX
        );

        Self {
            fire,
            smoke,
            sand,
            metal
        }
    }
}
