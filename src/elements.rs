use macroquad::rand::gen_range;

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
    pub sub_element: Option<Box<Element>>,
    pub state: StateOfMatter,
    pub lifetime: u16,
}

pub struct Elements {
    pub fire: Element,
    pub smoke: Element,
    pub sand: Element,
    pub metal: Element,
    pub water: Element,
    pub steam: Element,
    pub lava: Element,
    pub burning_coal: Element,
    pub coal: Element,
}

impl Element {
    pub fn new(
        name: impl ToString,
        color: macroquad::color::Color,
        weight: f32,
        sub_element: Option<Box<Element>>,
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

impl Elements {
    pub fn init() -> Self {
        let smoke = Element::new(            // --- Smoke ---
            "Smoke",
            macroquad::color::Color::new(0.2,0.2,0.2,1.0),
            -1.0,
            None,
            StateOfMatter::Gas,
            30
        );

        let fire = Element::new(            // --- Fire --- 
            "Fire",
            macroquad::color::Color::new(1.0,gen_range(0.3,0.5),0.0,1.0),
            -3.0,
            Some(Box::new(smoke.clone())),
            StateOfMatter::Gas,
            30
        );


        let steam = Element::new(           // --- Steam ---
            "Steam",
            macroquad::color::SKYBLUE,
            -1.0,
            None,
            StateOfMatter::Gas,
            u16::MAX
        );

        let water = Element::new(           // --- Water ---
            "Water",
            macroquad::color::Color::new(gen_range(0.25,0.3),gen_range(0.25,0.3),gen_range(0.7,0.75), 0.75),
            1.0,
            Some(Box::new(steam.clone())),
            StateOfMatter::Liquid,
            u16::MAX
        );

        let lava = Element::new(            // --- Lava ---
            "Lava",
            macroquad::color::Color::new(1.0, 0.3, 0.0, 1.0),
            1.0,
            None,//Some(Box::new(stone.clone())),
            StateOfMatter::Liquid,
            u16::MAX
        );
        
        let metal = Element::new(           // --- Metal ---
            "Metal",
            macroquad::color::Color::new(0.5,0.5,0.6,1.0),
            0.0,
            Some(Box::new(lava.clone())),
            StateOfMatter::Solid,
            u16::MAX
        );

        let sand = Element::new(            //  --- Sand ---    TODO: Add lava 'SubElement'
            "Sand",
            macroquad::color::Color::new(1.0,gen_range(0.9,1.0),0.5,1.0),
            1.0,
            Some(Box::new(lava.clone())),
            StateOfMatter::Powder,
            u16::MAX
        );
        
        let burning_coal = Element::new(            // --- Burning Coal ---
            "Coal",
            macroquad::color::Color::new(gen_range(0.4,0.5),gen_range(0.1,0.13),gen_range(0.1,0.13),1.0),
            1.0,
            Some(Box::new(fire.clone())),
            StateOfMatter::Powder,
            gen_range(2,10),
        );
        
        let coal = Element::new(            // --- Coal ---
            "Coal",
            macroquad::color::Color::new(gen_range(0.1, 0.13),gen_range(0.1,0.13),gen_range(0.1,0.13),1.0),
            1.0,
            Some(Box::new(burning_coal.clone())),
            StateOfMatter::Powder,
            u16::MAX
        );

        Self {
            fire,
            smoke,
            sand,
            metal,
            water,
            steam,
            lava,
            burning_coal,
            coal,
        }
    }
}
