extern crate sdl2;

use sdl2::event::{Event};

fn main() {

    // well yes, might as well unwrap in this case
    // the sdl::Sdl rust struct is not displayable by default any way
    let mut sdl_context = match sdl2::init() {
        Ok(sdl) => {
            println!("yay context initialised");
            sdl
        },
        Err(err) => panic!("Failed initialise sdl context: {}", err),
    };

    let video_subsystem = match sdl_context.video() {
        Ok(vid_sys) => vid_sys,
        Err(err) => panic!("Failed to init video subsystem: {}", err),
    };

    let mut window_builder = video_subsystem.window("A basic window", 640, 480);

    // window set to centered and usable within an OpenGL context
    // build() -> SdlResult<video::Window>
    let mut window: sdl2::video::Window = match window_builder.position_centered().opengl().build() {
        Ok(window) => window,
        Err(err)   => panic!("Failed to create window: {}", err)
    };

    let mut events = match sdl_context.event_pump() {
        Ok(pump) => pump,
        Err(err) => panic!("Failed to get the single event pump: {}", err),
    };

    window.show();

    // loop until we receive a QuitEvent
    'event : loop {
        // poll_event returns the most recent event or NoEvent if nothing has happened

        let mut events_iter: sdl2::event::EventPollIterator = events.poll_iter();

        for event in events_iter {
            use sdl2::event::Event;
            match event {
                Event::Quit{..} => break 'event,
                _               => continue
            }
        }
    }
}
