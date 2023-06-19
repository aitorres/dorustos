use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::backend::{Chip8, SCREEN_WIDTH, SCREEN_HEIGHT};

/// A scaling factor for the screen
const SCALE: u32 = 15;

/// Scaled width of the window
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;

/// Scaled height of the window
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

/// Amount of CPU ticks to run per frame
const TICKS_PER_FRAME: usize = 10;

/// Runs a Chip-8 emulator instance with a loaded program using SDL2 to draw the screen
/// and capture the keypresses.
///
/// # Arguments
///
/// * `chip8` - Chip-8 emulator instance
pub fn run_game(mut chip8: Chip8) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.
        window("dorustos Chip-8 Emulator", WINDOW_WIDTH, WINDOW_HEIGHT).
        position_centered().
        opengl().
        build().
        unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'gameloop: loop {
        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => {
                    break 'gameloop;
                },
                Event::KeyDown{keycode: Some(key), ..} => {
                    if let Some(btn) = key2btn(key) {
                        chip8.keypress(btn, true);
                    }
                },
                Event::KeyUp { keycode: Some(key), ..} => {
                    if let Some(btn) = key2btn(key) {
                        chip8.keypress(btn, false);
                    }
                }
                _ => ()
            }
        }

        for _ in 0..TICKS_PER_FRAME {
            chip8.tick();
        }
        chip8.tick_timers();
        draw_screen(&chip8, &mut canvas)
    }
}

/// Draws the current screen buffer to the canvas.
///
/// # Arguments
///
/// * `chip8` - Chip-8 emulator instance
/// * `canvas` - SDL2 canvas to draw to
fn draw_screen(chip8: &Chip8, canvas: &mut Canvas<Window>) {
    // Clear canvas
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let screen_buf = chip8.get_display();

    // Set draw color to white, iterate and check if each point should be drawn
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    for (i, pixel) in screen_buf.iter().enumerate() {
        if *pixel {
            // Convert from 1D array to 2D coordinates
            let x = (i % SCREEN_WIDTH) as u32;
            let y = (i / SCREEN_WIDTH) as u32;

            // Draw a rectangle at the coordinates scaled up by SCALE value
            let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
            canvas.fill_rect(rect).unwrap();
        }
    }

    canvas.present();
}

/// Maps a SDL2 keycode to the respective Chip-8 button.
/// Note that we're mapping the Chip-8 keypad to the left
/// side of a standard QWERTY keyboard.
///
/// # Arguments
///
/// * `key` - SDL2 keycode to map
fn key2btn(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),
        Keycode::Z => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use sdl2::keyboard::Keycode;

    #[test]
    fn key2btn_maps_appropriately() {
        let result_1 = super::key2btn(Keycode::Num1);
        assert_eq!(result_1, Some(0x1));

        let result_2 = super::key2btn(Keycode::Num2);
        assert_eq!(result_2, Some(0x2));

        let result_3 = super::key2btn(Keycode::W);
        assert_eq!(result_3, Some(0x5));

        let result_4 = super::key2btn(Keycode::K);
        assert_eq!(result_4, None);
    }
}
