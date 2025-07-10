use macroquad;

#[derive(PartialEq, Clone)]
pub enum StateOfMatter {
    Solid,
    Liquid,
    Gas,
    Powder
}

#[derive(PartialEq, Clone)]
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
    pub sand: Element,
    pub metal: Element,
    pub water: Element,
    pub steam: SubElement,
}

impl Element {
    pub fn new(
        name: impl ToString,
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
        name: impl ToString,
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
        let smoke = SubElement::new(            // --- Smoke ---
            "Smoke",
            macroquad::color::Color::new(0.2,0.2,0.2,0.1),
            -0.01,
            StateOfMatter::Gas,
            30
        );

        let fire = Element::new(            // --- Fire --- 
            "Fire",
            macroquad::color::Color::new(1.0,0.5,0.0,1.0),
            -0.25,
            smoke.clone(),
            StateOfMatter::Gas,
            30
        );

        let sand = Element::new(     // TODO: Add lava 'SubElement' --- Sand ---
            "Sand",
            macroquad::color::Color::new(0.8,0.8,0.55,1.0),
            1.0,
            smoke.clone(),
            StateOfMatter::Powder,
            u16::MAX
        );

        let metal = Element::new(           // --- Metal ---
            "Metal",
            macroquad::color::Color::new(0.5,0.5,0.6,1.0),
            0.0,
            smoke.clone(),
            StateOfMatter::Solid,
            u16::MAX
        );

        let steam = SubElement::new(           // --- Steam ---
            "Steam",
            macroquad::color::SKYBLUE,
            -0.1,
            StateOfMatter::Gas,
            u16::MAX
        );

        let water = Element::new(           // --- Water ---
            "Water",
            macroquad::color::Color::new(0.1,0.1,0.9, 0.75),
            1.0,
            steam.clone(),
            StateOfMatter::Liquid,
            u16::MAX
        );

        Self {
            fire,
            smoke,
            sand,
            metal,
            water,
            steam,
        }
    }
}
