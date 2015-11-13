
use sdl2::event::{Event};
use sdl2::keyboard::{Keycode};
use sdl2::pixels::{Color};
use setup;

pub fn sprite_sheet() {
    let basic_window_setup = setup::init("Sprite Sheet Display", 640, 480);
    let mut events = basic_window_setup.sdl_context.event_pump().unwrap();
    let mut renderer = basic_window_setup.window.renderer()
        .present_vsync().accelerated().build()
        .ok().expect("Failed to build accelerated vsync renderer");

    let cyan = Color::RGB(0, 0xff, 0xff);
    let sheet_load_data = setup::load_keyed_texture("resources/dots.png", cyan, &renderer);
    let mut sprite_sheet_texture = sheet_load_data.0;

    let clip_rects = [setup::make_rect((0, 0), (100, 100)),
                      setup::make_rect((100, 0), (100, 100)),
                      setup::make_rect((0, 100), (100, 100)),
                      setup::make_rect((100, 100), (100, 100))];

    let mut r = 0;
    let mut g = 0;
    let mut b = 0;

    'event : loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => break 'event,

                Event::KeyDown{keycode: Some(Keycode::Q), ..} => break 'event,

                _ => continue
            }
        }

        renderer.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        renderer.clear();

        sprite_sheet_texture.set_color_mod(r, g, b);
        let inc_modulation = |n: u8| n.wrapping_add(1);
        r = inc_modulation(r);
        g = inc_modulation(g);
        b = inc_modulation(b);

        for i in 0 .. clip_rects.len() {
            let scalar: i32 = (i as i32 + 1) * 90;
            renderer.copy(&sprite_sheet_texture,
                      Some(clip_rects[i]),
                      Some(setup::make_rect((scalar, scalar),
                                            (100, 100)))
                      );
        }

        renderer.present(); // screen update from the backbuffer
    }

    setup::quit();
}
