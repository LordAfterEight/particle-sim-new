pub struct Settings {
    pub pause_state: bool,
    pub display_grid: bool,
    pub display_help: bool,
    pub display_elements: bool,
    pub fullscreen: bool,
    pub text_offset: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            pause_state: false,
            display_grid: false,
            display_help: false,
            display_elements: false,
            fullscreen: false,
            text_offset: 13.0
        }
    }
}
