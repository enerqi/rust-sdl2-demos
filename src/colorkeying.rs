use setup;

use sdl2::event::Event;
use sdl2::pixels::Color;


const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

pub fn color_keying() {

    let basic_window_setup = setup::init("Sprite on background", SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut events = basic_window_setup.sdl_context.event_pump().unwrap();

    let mut renderer = basic_window_setup.window
                                         .renderer()
                                         .present_vsync()
                                         .accelerated()
                                         .build()
                                         .unwrap();

    let cyan = Color::RGB(0, 0xff, 0xff);

    let background = setup::load_image("resources/background.png", &renderer);
    let (sprite, w_h) = setup::load_keyed_texture("resources/stick-man.png", cyan, &renderer);
    let sprite_target = setup::make_rect((240, 190), w_h);

    'event: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => break 'event,
                // keycode: Option<KeyCode>
                // https://doc.rust-lang.org/book/patterns.html
                Event::KeyDown{keycode: Some(::sdl2::keyboard::Keycode::Q), ..} => break 'event,
                _ => continue,
            }
        }

        renderer.copy(&background, None, None);
        renderer.copy(&sprite, None, Some(sprite_target)); // after the background
        renderer.present(); // screen update from the backbuffer
    }

}
