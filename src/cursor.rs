pub enum CursorShapes {
    Circle,
    Rectangle,
    Triangle,
}

pub struct Cursor {
    pub size_modifier: f32,
    pub position: (f32,f32),
    pub shape: CursorShapes
}

impl Cursor {
    pub fn init() -> Self {
        Self {
            size_modifier: 1.0,
            position: macroquad::input::mouse_position(),
            shape: CursorShapes::Circle
        }
    }

    pub fn update(&mut self) {
        self.position = macroquad::input::mouse_position();
    }
}
