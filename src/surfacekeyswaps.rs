
// show different images depending upon what key is pressed

use std::path::Path;
use std::thread;
use std::time::Duration;

use sdl2;
use sdl2::event::Event;
// use sdl2::pixels::{PixelFormat};
use sdl2::surface::{Surface, SurfaceRef};

use setup;

// `enum_primitive` crate for when we want to give the
// discriminants distinct values as integers
// Otherwise enums can be cast as integers. We will need an extra "count" discriminant
// or a count outside the type (which is worse).
//
// Copy derived so we don't have to clone it or get move complaints after assigning to mut
#[derive(Copy, Clone)]
enum KeyPressSurface {
    Default,
    Up,
    Down,
    Left,
    Right,
    Count,
}

// C Like enums are often used to map from a constant to some value in an Array
// E.g, index to image surface
// Hash tables maybe a bit wordy and unperformant when we know the exact static
// size of the array required.
pub fn surface_keyswaps() {

    let basic_window_setup = setup::init("Image swaps", 640, 480);

    let mut events = basic_window_setup.sdl_context.event_pump().unwrap();

    let images_count = KeyPressSurface::Count as usize;
    let mut images: Vec<Surface> = Vec::with_capacity(images_count);

    // As a closure so it's a bit more ergonomic when called
    // The different syntax to a normal function is a bit of friction in Jai's philosophy
    // and a nod to pontential performance differences in Rust's eyes
    // scope so we don't borrow `images` as mutable for too long
    {
        let window_surface_ref: &SurfaceRef = basic_window_setup.window.surface(&events).unwrap();
        let mut add_image_surface = |image_path| {
            let surface = load_surface(image_path);
            let err_msg = format!("Surface conversion to the window's surface pixel format \
                                   failed: {}",
                                  image_path);
            let optimized_surface = surface.convert(&window_surface_ref.pixel_format())
                                           .ok()
                                           .expect(&err_msg);
            images.push(optimized_surface);
        };
        // fn add_image_surface<'a>(image_path: &'a str, images: &mut Vec<Surface<'a>>) {
        //     let surface = load_surface(image_path);
        //     images.push(surface);
        // }
        // Converting an integer to an enum still requires enum_primitive, we need more than the
        // ability to convert from enum to integer, which std rust can do.
        // There is this clunkier way though
        for i in 0..images_count {
            match i {
                i if i == KeyPressSurface::Default as usize => {
                    add_image_surface("resources/press.bmp")
                }
                i if i == KeyPressSurface::Up as usize => add_image_surface("resources/up.bmp"),
                i if i == KeyPressSurface::Down as usize => add_image_surface("resources/down.bmp"),
                i if i == KeyPressSurface::Left as usize => add_image_surface("resources/left.bmp"),
                i if i == KeyPressSurface::Right as usize => {
                    add_image_surface("resources/right.bmp")
                }
                _ => (),
            }
        }
    }


    let mut current_key = KeyPressSurface::Default;

    // loop until we receive a QuitEvent
    'event: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => break 'event,
                // keycode: Option<KeyCode>
                // https://doc.rust-lang.org/book/patterns.html
                Event::KeyDown{keycode: Some(sdl2::keyboard::Keycode::Q), ..} => break 'event,
                Event::KeyDown{keycode: Some(sdl2::keyboard::Keycode::Up), ..} => {
                    current_key = KeyPressSurface::Up
                }
                Event::KeyDown{keycode: Some(sdl2::keyboard::Keycode::Down), ..} => {
                    current_key = KeyPressSurface::Down
                }
                Event::KeyDown{keycode: Some(sdl2::keyboard::Keycode::Left), ..} => {
                    current_key = KeyPressSurface::Left
                }
                Event::KeyDown{keycode: Some(sdl2::keyboard::Keycode::Right), ..} => {
                    current_key = KeyPressSurface::Right
                }
                Event::KeyUp{..} => current_key = KeyPressSurface::Default,

                _ => continue,
            }
        }

        let draw_image = &images[current_key as usize];
        let window_surface_ref: &SurfaceRef = basic_window_setup.window.surface(&events).unwrap();
        unsafe {
            let mut window_surface: Surface = Surface::from_ll(window_surface_ref.raw());
            draw_image.blit_scaled(None, &mut window_surface, None).unwrap();
        }

        basic_window_setup.window.update_surface().unwrap();

        // A very primitive frame limiting mechanism for now
        thread::sleep(Duration::from_millis(10));
    }
}


fn load_surface(image_path: &str) -> Surface {

    let err_msg: String = format!("Failed to load image bmp at path: {}", image_path);
    // .as_str() has some RFC convert warning
    Surface::load_bmp(&Path::new(image_path)).ok().expect(&err_msg)
}
