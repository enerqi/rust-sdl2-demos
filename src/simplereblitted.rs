use std::path::Path;
use std::thread;

use sdl2;
use sdl2::event::Event;
use sdl2::surface::{Surface, SurfaceRef};

use setup;


/// sdl example loop that will also repaint the blitted main window surface
/// if required
/// For the simple_surface example, the window surface is drawn to once and
/// comes back black after being minimised
pub fn simple_reblitted() {

    let basic_window_setup = setup::init("Ice Troll Reblitted!", 640, 480);

    let mut events = basic_window_setup.sdl_context.event_pump().unwrap();

    let troll_image_surface = Surface::load_bmp(&Path::new("resources/ice-troll.bmp")).unwrap();

    // loop until we receive a QuitEvent
    // Note there is nothing to stop the CPU burning out 1000 fps.
    'event: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => break 'event,
                // keycode: Option<KeyCode>
                // https://doc.rust-lang.org/book/patterns.html
                Event::KeyDown{keycode: Some(sdl2::keyboard::Keycode::Q), ..} => break 'event,
                _ => continue,
            }
        }

        let window_surface_ref: &SurfaceRef = basic_window_setup.window.surface(&events).unwrap();
        unsafe {
            // This is bad? Takes ownership and drops the surface at the end of the block
            let mut window_surface: Surface = Surface::from_ll(window_surface_ref.raw());
            troll_image_surface.blit_scaled(None, &mut window_surface, None).unwrap();
        }
        basic_window_setup.window.update_surface().unwrap();

        // A very primitive frame limiting mechanism for now
        thread::sleep_ms(10);
    }
}
