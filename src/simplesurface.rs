
use std::path::Path;

use sdl2;
use sdl2::event::{Event};
//use sdl2::rect::{Rect};
use sdl2::surface::{Surface, SurfaceRef};
use sdl2::version;
use sdl2::VideoSubsystem;

pub fn simple_surface() {

    let version = version::version();
    println!("SDL version: {:?}", version);

    // well yes, might as well unwrap in this case
    // the sdl::Sdl rust struct is not displayable by default any way
    let sdl_context: sdl2::Sdl = match sdl2::init() {
        Ok(sdl) => {
            println!("Initialised");
            sdl
        },
        Err(err) => panic!("Failed initialise sdl context: {}", err),
    };

    let video_subsystem: sdl2::VideoSubsystem = match sdl_context.video() {
        Ok(vid_sys) => vid_sys,
        Err(err) => panic!("Failed to init video subsystem: {}", err),
    };

    let mut window_builder: sdl2::video::WindowBuilder = video_subsystem.window("A basic window", 640, 480);

    // window set to centered and usable within an OpenGL context
    // build() -> SdlResult<video::Window>
    let mut window: sdl2::video::Window = match window_builder.position_centered().opengl().build() {
        Ok(window) => window,
        Err(err)   => panic!("Failed to create window: {}", err)
    };
    println!("Window size: {:?}", window.size());

    let mut events: sdl2::EventPump = match sdl_context.event_pump() {
        Ok(pump) => pump,
        Err(err) => panic!("Failed to get the single event pump: {}", err),
    };

    let image_surface = match Surface::load_bmp(&Path::new("resources/ice-troll.bmp")) {
        Ok(surface) => surface,
        Err(err)    => panic!("failed to load surface: {}", err)
    };

    // why does getting the surface need a reference to the event pump??????
    // Create new scope so we don't have conflicting mutable borrows (with the event loop)
    {
        // blit (copy) the source surface onto this surface
        let window_surface_ref: &SurfaceRef = window.surface(&events).unwrap();
        //let source_image_rect: Option<Rect> = image_surface.clip_rect();

        // pub fn blit<S: AsMut<SurfaceRef>>(&self, src_rect: Option<Rect>, mut dst: S, mut dst_rect: Option<Rect>) -> SdlResult<Option<Rect>>
        // blit(...) arg2 is an AsMut<SurfaceRef> trait.
        // - Only Surface implements that.
        // - It's really hard to get a Surface reference and not a SurfaceRef
        // - src_rect is moved
        // .. Surface::from_ll (from low level) gets it from a raw low level interface.
        unsafe {
            let mut window_surface: Surface = Surface::from_ll(window_surface_ref.raw());
            // This api is broken? Apart from this stupid contortion to call blit...
            // blit's implementation seems to be effectively a static method that does not use self?
            // Note we don't need the clip_rect 1st parameter, by default use all of self for clipping the src image
            // So if we alter the dst_rect instead of using None...can we stretch the small image to the entire window? No
            // let dst_clip_rectangle = window_surface_ref.clip_rect();
            // I think we would need sdl convert surface on the image surface followed by destroying the initial image surface
            //let dst_clip_rectangle = None;
            //image_surface.blit(None, &mut window_surface, dst_clip_rectangle);
            // "What blitting does is take a source surface and stamps a copy of it onto the destination surface. The first argument (this)
            // of SDL_BlitSurface is the source image. The third argument is the destination. We'll worry about the 2nd and 4th arguments in future tutorials."
            image_surface.blit_scaled(None, &mut window_surface, None).unwrap();

        }

        // Now that all the backbuffer updating is done
        window.update_surface().unwrap();
    }

    window.set_title("Ice Troll");

    // loop until we receive a QuitEvent
    'event : loop {
        // poll_event returns the most recent event or NoEvent if nothing has happened

        let events_iter: sdl2::event::EventPollIterator = events.poll_iter();
        for event in events_iter {
            match event {
                Event::Quit{..} => break 'event,

                // keycode: Option<KeyCode>
                // https://doc.rust-lang.org/book/patterns.html
                Event::KeyDown{keycode: Some(sdl2::keyboard::Keycode::Q), ..} => {
                //Event::KeyDown{keycode, ..} if keycode.is_some() => {
                    break 'event
                    // if let Some(sdl2::keyboard::Keycode::Q) = keycode {
                    //     break 'event
                    // }
                    // if keycode.unwrap() == sdl2::keyboard::Keycode::Q { break 'event }
                    // else { continue }
                },

                _               => continue
            }
        }
    }
}


