pub fn handle_input(mut fullscreen: bool) {
    let input = match macroquad::input::get_char_pressed() {
        Some(key) => key,
        None => ' '
    };

    match input {
        'q' => std::process::exit(1),
        'f' => {
            match fullscreen {
                true => {
                    macroquad::window::set_fullscreen(fullscreen);
                    fullscreen = false;
                },
                false => {
                    macroquad::window::set_fullscreen(fullscreen);
                    fullscreen = true;
                }
            }
        },
        _ => {}
    };
}

pub fn draw_diagnostics() {
    let (mouse_x, mouse_y) = macroquad::input::mouse_position();
    unsafe {
        macroquad::prelude::draw_text(
            &format!("FPS: {} | Particles: {}", macroquad::time::get_fps(), crate::PIXEL_AMOUNT) as &str,
            5.0,
            13.0,
            16.0,
            macroquad::color::WHITE
        );
    }

    macroquad::prelude::draw_text("Press [q] to exit", crate::SCREEN_WIDTH as f32 - 125.0, 13.0, 16.0, macroquad::color::WHITE);
    macroquad::prelude::draw_text(
        &format!("X: {} | Y: {}", mouse_x, mouse_y) as &str,
        crate::SCREEN_WIDTH as f32 - 125.3,
        28.0,
        16.0,
        macroquad::color::WHITE
    );
    macroquad::prelude::draw_text(
        &format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")) as &str,
        crate::SCREEN_WIDTH as f32 - 148.0,
        crate::SCREEN_HEIGHT as f32 - 4.0,
        16.0,
        macroquad::color::Color::new(1.0,1.0,1.0,0.5)
    );
}
