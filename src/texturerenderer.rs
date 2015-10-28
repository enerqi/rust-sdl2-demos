use sdl2;
use sdl2::event::{Event};

use setup;


pub fn texture_render() {
    let basic_window_setup = setup::init("Textures", 640, 480);

    let mut events = basic_window_setup.sdl_context.event_pump().unwrap();

    let mut renderer = basic_window_setup.window.renderer()
            .present_vsync().accelerated().build().unwrap();
            // software is 10x more cpu usage
            //software().build().unwrap();

    let texture = setup::load_image("resources/ice-troll.png", &renderer);

    // loop until we receive a QuitEvent
    'event : loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => break 'event,
                // keycode: Option<KeyCode>
                // https://doc.rust-lang.org/book/patterns.html
                Event::KeyDown{keycode: Some(sdl2::keyboard::Keycode::Q), ..} => {
                    break 'event
                },
                _ => continue
            }
        }

        renderer.copy(&texture, None, None);
        renderer.present(); // screen update from the backbuffer

        // Not needed if using vsync - that acts as a throttle
        // A very primitive frame limiting mechanism for now
        //thread::sleep_ms(10);
    }

    setup::quit();
}

