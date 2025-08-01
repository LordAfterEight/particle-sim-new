use macroquad::text::draw_text;

use crate::render::Frame;

pub fn handle_input(frame: &mut Frame, settings: &mut crate::settings::Settings, cursor: &mut crate::cursor::Cursor) {
    let (_wheel_x, wheel_y) = macroquad::input::mouse_wheel();
    let input = match macroquad::input::get_char_pressed() {
        Some(key) => key,
        None => '\n',
    };

    match input {
        'q' => std::process::exit(0),
        'g' => match settings.display_grid {
            true => settings.display_grid = false,
            false => settings.display_grid = true,
        },
        'h' => match settings.display_help {
            true => settings.display_help = false,
            false => settings.display_help = true,
        },
        'e' => match settings.display_elements {
            true => settings.display_elements = false,
            false => settings.display_elements = true,
        },
        ' ' => match settings.pause_state {
            true => settings.pause_state = false,
            false => settings.pause_state = true
        },
        _ => {}
    };

    match wheel_y {
        120.0  => {
            cursor.size_modifier += 1.0;
            if cursor.size_modifier as u16 % 2 == 0 {
                cursor.size_modifier += 1.0;
            }
        },
        -120.0 => {
            cursor.size_modifier -= 1.0;
            if cursor.size_modifier as u16 % 2 == 0 {
                cursor.size_modifier -= 1.0;
            }
        },
        _ => {}
    }

    if cursor.size_modifier < 1.0 {
        cursor.size_modifier = 1.0;
    }

}

pub fn draw_info(
    frame: &mut Frame,
    settings: &crate::settings::Settings,
    cursor: &mut crate::cursor::Cursor,
    elements: &crate::elements::Elements,
) {
    let (mut mouse_x, mut mouse_y) = cursor.position;
    let cursor_x = mouse_x / frame.grid_scaling as f32;
    let cursor_y = mouse_y / frame.grid_scaling as f32;
    if mouse_y >= crate::SCREEN_HEIGHT { mouse_y = crate::SCREEN_HEIGHT - frame.grid_scaling; }

    // display grid if enabled
    match settings.display_grid {
        true => {
            for x in 0..frame.grid.len() as usize {
                macroquad::prelude::draw_line(
                    x as f32 * frame.grid_scaling * 3.0,
                    0.0,
                    x as f32 * frame.grid_scaling * 3.0,
                    crate::SCREEN_HEIGHT,
                    1.0,
                    macroquad::color::Color::new(0.2, 0.2, 0.2, 0.5),
                );
            }

            for y in 0..frame.grid[0].len() as usize {
                macroquad::prelude::draw_line(
                    0.0,
                    y as f32 * frame.grid_scaling * 3.0,
                    crate::SCREEN_WIDTH,
                    y as f32 * frame.grid_scaling * 3.0,
                    1.0,
                    macroquad::color::Color::new(0.2, 0.2, 0.2, 0.5),
                );
            }
        }
        false => {}
    }

    // Draw cursor
    macroquad::prelude::draw_rectangle(
        cursor_x.floor() * frame.grid_scaling - (cursor.size_modifier / 2.0 - 0.5) * frame.grid_scaling,
        cursor_y.floor() * frame.grid_scaling - (cursor.size_modifier / 2.0 - 0.5) * frame.grid_scaling,
        frame.grid_scaling * cursor.size_modifier,
        frame.grid_scaling * cursor.size_modifier,
        macroquad::color::Color::new(0.75, 0.75, 0.75, 0.75),
    );

    // Draw FPS and particle counter
    macroquad::prelude::draw_text(
        &format!(
            "FPS: {} | Particles: {}",
            macroquad::time::get_fps(),
            unsafe { crate::PIXEL_AMOUNT }
        ) as &str,
        5.0,
        13.0,
        16.0,
        macroquad::color::WHITE,
    );

    // Draw currently selected Element
    let selected_element_text: &str = &format!("Selected element: {}", "None");
    macroquad::prelude::draw_text(
        selected_element_text,
        crate::SCREEN_WIDTH / 2.0 - (selected_element_text.len() as f32 * 8.0 / 2.0),
        13.0,
        16.0,
        macroquad::color::WHITE,
    );

    // Draw exit info and cursor coordinates
    macroquad::prelude::draw_text(
        "Press [q] to exit",
        crate::SCREEN_WIDTH / crate::SCALING - 125.0,
        1.0 * settings.text_offset,
        16.0,
        macroquad::color::WHITE,
    );
    macroquad::prelude::draw_text(
        &format!("X: {:.0} | Y: {:.0}", mouse_x.floor(), mouse_y.floor()) as &str,
        crate::SCREEN_WIDTH / crate::SCALING - 125.3,
        2.0 * settings.text_offset,
        16.0,
        macroquad::color::WHITE,
    );

    // Draw help info
    match settings.display_help {
        true => {
            draw_text(
                "v Help [h]",
                crate::SCREEN_WIDTH / crate::SCALING - 135.0,
                3.0 * settings.text_offset,
                16.0,
                macroquad::color::SKYBLUE,
            );
            draw_text(
                "[g] - Show grid",
                crate::SCREEN_WIDTH / crate::SCALING - 136.0,
                4.0 * settings.text_offset,
                16.0,
                macroquad::color::SKYBLUE,
            );
            draw_text(
                "[e] - Show elements",
                crate::SCREEN_WIDTH / crate::SCALING - 136.0,
                5.0 * settings.text_offset,
                16.0,
                macroquad::color::SKYBLUE,
            );
            draw_text(
                "[space] - pause",
                crate::SCREEN_WIDTH / crate::SCALING - 136.0,
                6.0 * settings.text_offset,
                16.0,
                macroquad::color::SKYBLUE,
            );
            draw_text(
                "[f] - single step",
                crate::SCREEN_WIDTH / crate::SCALING - 136.0,
                7.0 * settings.text_offset,
                16.0,
                macroquad::color::SKYBLUE,
            );
        }
        false => {
            draw_text(
                "> Help [h]",
                crate::SCREEN_WIDTH / crate::SCALING - 125.3,
                3.0 * settings.text_offset,
                16.0,
                macroquad::color::WHITE,
            );
        }
    }

    // Show element selection window
    if (mouse_x > 10.0) && (mouse_x < 750.0) && (mouse_y > crate::SCREEN_HEIGHT / 2.0 - 250.0) && (mouse_y < crate::SCREEN_HEIGHT / 2.0 + 250.0) && settings.display_elements {
        macroquad::input::show_mouse(true);
    } else { macroquad::input::show_mouse(false); }
    match settings.display_elements {
        true => {
            macroquad::shapes::draw_rectangle(
                10.0,
                crate::SCREEN_HEIGHT / 2.0 - 250.0,
                750.0,
                500.0,
                macroquad::color::Color::new(0.1, 0.1, 0.1, 0.9),
            );
            draw_text(
                "Elements",
                crate::SCREEN_WIDTH / 4.0,
                crate:: SCREEN_HEIGHT / 2.0 - 250.0 + 16.0,
                20.0,
                macroquad::color::WHITE,
            );
        }
        false => {}
    }

    // Draw app name and version info
    macroquad::prelude::draw_text(
        &format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")) as &str,
        crate::SCREEN_WIDTH / crate::SCALING - 165.0,
        crate::SCREEN_HEIGHT / crate::SCALING - 4.0,
        16.0,
        macroquad::color::Color::new(1.0, 1.0, 1.0, 0.5),
    );
}
