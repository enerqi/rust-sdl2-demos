
// why am i putting this here, why not just in the crate root lib.rs file?????????
// The following `use sdl2` fixes it. Presumably extern crate directives add the
// crate root symbol to the current module
// extern crate sdl2;
use sdl2;
use sdl2::event::Event;
use sdl2::version;

pub fn simple_window() {

    let version = version::version();
    println!("SDL version: {:?}", version);

    // well yes, might as well unwrap in this case
    // the sdl::Sdl rust struct is not displayable by default any way
    let sdl_context: sdl2::Sdl = match sdl2::init() {
        Ok(sdl) => {
            print!("...");
            sdl
        }
        Err(err) => panic!("Failed initialise sdl context: {}", err),
    };

    let video_subsystem: sdl2::VideoSubsystem = match sdl_context.video() {
        Ok(vid_sys) => vid_sys,
        Err(err) => panic!("Failed to init video subsystem: {}", err),
    };

    let mut window_builder: sdl2::video::WindowBuilder = video_subsystem.window("A basic window",
                                                                                640,
                                                                                480);

    // window set to centered and usable within an OpenGL context
    // build() -> SdlResult<video::Window>
    let mut window: sdl2::video::Window = match window_builder.position_centered()
                                                              .opengl()
                                                              .build() {
        Ok(window) => window,
        Err(err) => panic!("Failed to create window: {}", err),
    };

    let mut events: sdl2::EventPump = match sdl_context.event_pump() {
        Ok(pump) => pump,
        Err(err) => panic!("Failed to get the single event pump: {}", err),
    };

    // seems to be shown by default
    window.show();

    // loop until we receive a QuitEvent
    'event: loop {
        // poll_event returns the most recent event or NoEvent if nothing has happened

        let events_iter: sdl2::event::EventPollIterator = events.poll_iter();
        for event in events_iter {
            match event {
                Event::Quit{..} => break 'event,

                // keycode: Option<KeyCode>
                // https://doc.rust-lang.org/book/patterns.html
                Event::KeyDown{keycode: Some(sdl2::keyboard::Keycode::Q), ..} => {
                    // Event::KeyDown{keycode, ..} if keycode.is_some() => {
                    break 'event;
                    // if let Some(sdl2::keyboard::Keycode::Q) = keycode {
                    //     break 'event
                    // }
                    // if keycode.unwrap() == sdl2::keyboard::Keycode::Q { break 'event }
                    // else { continue }
                }

                _ => continue,
            }
        }
    }
}
