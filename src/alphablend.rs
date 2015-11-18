use sdl2::event::{Event};
use sdl2::keyboard::{Keycode};
use sdl2::render::{BlendMode};
use sdl2::pixels::{Color};

use setup;

pub fn alpha_blend() {
    let basic_window_setup = setup::init("Alpha Blending", 640, 480);

    let mut events = basic_window_setup.sdl_context.event_pump().unwrap();
    let mut renderer = basic_window_setup.window.renderer()
        .present_vsync().accelerated().build().unwrap();

    // unmodulated alpha - shows up only when the foreground texture goes transparent
    let bg_fade_in_texture = setup::load_image("resources/fadein.png", &renderer);

    // not a sprite (with a background color key), but will be alpha modulated
    let mut fg_fade_out_texture = setup::load_image("resources/fadeout.png", &renderer);
    fg_fade_out_texture.set_blend_mode(BlendMode::Blend);

    let mut alpha_modulation: u8 = 255;
    let mod_increment: u8 = 32;

    'event : loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => break 'event,
                // keycode: Option<KeyCode>
                // https://doc.rust-lang.org/book/patterns.html
                Event::KeyDown{keycode: Some(Keycode::Q), ..} => {
                    break 'event
                },

                Event::KeyDown{keycode: Some(key), ..} => {
                    match key {
                        Keycode::W => {
                            if (alpha_modulation as u32) + (mod_increment as u32) > 255 {
                                alpha_modulation = 255;
                            }
                            else {
                                alpha_modulation += mod_increment;
                            }
                        },
                        Keycode::S => {
                            if (alpha_modulation as i32) - (mod_increment as i32) < 0 {
                                alpha_modulation = 0;
                            }
                            else {
                                alpha_modulation -= mod_increment;
                            }
                        },
                        _ => continue
                    }
                },
                _ => continue
            }
        }

        fg_fade_out_texture.set_alpha_mod(alpha_modulation);

        renderer.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        renderer.clear();

        renderer.copy(&bg_fade_in_texture, None, None);
        renderer.copy(&fg_fade_out_texture, None, None);
        renderer.present();
    }
}
