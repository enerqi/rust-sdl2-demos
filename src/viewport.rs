
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use setup;

const SCREEN_WIDTH: u32 = 960;
const SCREEN_HEIGHT: u32 = 720;

pub fn viewports() {
    let basic_window_setup = setup::init("Viewports", SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut events = basic_window_setup.sdl_context.event_pump().unwrap();
    let mut renderer = basic_window_setup.window
                                         .renderer()
                                         .present_vsync()
                                         .accelerated()
                                         .build()
                                         .ok()
                                         .expect("Failed to build accelerated vsync renderer");
    let texture = setup::load_image("resources/ice-troll.png", &renderer);

    let top_left_view_port = make_rect(0, 0, SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
    let top_right_view_port = make_rect((SCREEN_WIDTH as i32) / 2,
                                        0,
                                        SCREEN_WIDTH / 2,
                                        SCREEN_HEIGHT / 2);
    let bottom_view_port = make_rect(0,
                                     (SCREEN_HEIGHT as i32) / 2,
                                     SCREEN_WIDTH,
                                     SCREEN_HEIGHT / 2);

    'event: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => break 'event,

                Event::KeyDown{keycode: Some(Keycode::Q), ..} => break 'event,

                _ => continue,
            }
        }

        renderer.set_viewport(Some(top_left_view_port));
        renderer.copy(&texture, None, None);

        renderer.set_viewport(Some(top_right_view_port));
        renderer.copy(&texture, None, None);

        renderer.set_viewport(Some(bottom_view_port));
        renderer.copy(&texture, None, None);

        renderer.present(); // screen update from the backbuffer
    }

    setup::quit();
}

fn make_rect(x: i32, y: i32, w: u32, h: u32) -> Rect {
    ::sdl2::rect::Rect::new(x, y, w, h)
}
