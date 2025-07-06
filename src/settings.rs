pub struct Settings {
    pub display_grid: bool,
    pub display_help: bool,
    pub fullscreen: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            display_grid: false,
            display_help: false,
            fullscreen: false,
        }
    }
}
