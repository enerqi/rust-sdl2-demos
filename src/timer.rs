use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::{Renderer, Texture, TextureQuery};
use sdl2_ttf::Font;
use setup;


const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

pub fn timer() {


    let mut basic_window_setup = setup::init("Timer", SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut events = basic_window_setup.sdl_context.event_pump().unwrap();
    let mut renderer = basic_window_setup.window
                                         .renderer()
                                         .present_vsync()
                                         .accelerated()
                                         .build()
                                         .unwrap();

    let font = setup::load_font("resources/lazy.ttf", 16, &basic_window_setup.ttf_context);

    let mut time_elapsed = basic_window_setup.timer_subsystem.ticks();
    let mut timer_start = time_elapsed;

    'event: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => break 'event,
                Event::KeyDown{keycode: Some(::sdl2::keyboard::Keycode::Q), ..} => break 'event,
                Event::KeyDown{keycode: Some(::sdl2::keyboard::Keycode::Return), ..} => {
                    timer_start = basic_window_setup.timer_subsystem.ticks();
                }
                _ => continue,
            }
        }

        time_elapsed = basic_window_setup.timer_subsystem.ticks();
        let mut timer_display_texture = time_texture(time_elapsed - timer_start, &font, &renderer);
        let target = time_texture_draw_position(&timer_display_texture);

        renderer.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        renderer.clear();
        renderer.copy(&mut timer_display_texture, None, Some(target));
        renderer.present();
    }

    setup::quit();
}

fn time_texture(ms: u32, font: &Font, renderer: &Renderer) -> Texture {
    let msg = format!("Milliseconds since start time {}", ms);
    setup::text_texture(&msg, Color::RGB(0, 0, 255), font, renderer)
}

fn time_texture_draw_position(texture: &Texture) -> ::sdl2::rect::Rect {
    let padding = 64;
    let TextureQuery { width, height, .. } = texture.query();
    setup::get_centered_rect(SCREEN_WIDTH,
                             SCREEN_HEIGHT,
                             width,
                             height,
                             SCREEN_WIDTH - padding,
                             SCREEN_HEIGHT - padding)
}
