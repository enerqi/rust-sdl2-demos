use std::path::{Path};

use sdl2;
use sdl2::pixels::{Color};
use sdl2::rect::{Rect};
use sdl2::surface::{Surface};
use sdl2::render::{Renderer, Texture};
use sdl2_image::{self, LoadTexture, LoadSurface, INIT_PNG, INIT_JPG};
use sdl2_ttf;

pub struct BasicWindow {

    pub sdl_context: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub window: sdl2::video::Window,
    pub ttf_context: sdl2_ttf::Sdl2TtfContext,
}

pub fn init(window_title: &str, width: u32, height: u32) -> BasicWindow {

    let sdl_context: sdl2::Sdl = sdl2::init().unwrap();

    let video_subsystem: sdl2::VideoSubsystem = sdl_context.video().unwrap();

    let mut window_builder: sdl2::video::WindowBuilder = video_subsystem.window(window_title, width, height);

    let window: sdl2::video::Window = window_builder.position_centered().opengl().build().unwrap();

    sdl2_image::init(INIT_PNG | INIT_JPG);
    let ttf_context = sdl2_ttf::init().ok().expect("Failed to init true type fonts");

    BasicWindow {sdl_context: sdl_context,
                 video_subsystem: video_subsystem,
                 window: window,
                 ttf_context: ttf_context}
}

pub fn quit() {
    sdl2_image::quit();
}

// Using sdl image to loading a surface and then convert it into a
// texture that an sdl_renderer can hardware draw and stash in gpu ram.
pub fn load_image(image_path: &str, renderer: &Renderer) -> Texture {
    let err_msg: String = format!("Failed to load image at path: {}", image_path);

    // wtf I have no idea where load texture comes from!
    // maybe the documentation is out of date...no sdl2_image needs doc:
    // ...
    // trait LoadTexture in sdl2_image
    // ...
    // impl LoadTexture for Renderer
    renderer.load_texture(Path::new(image_path)).ok().expect(&err_msg)
}

pub fn load_keyed_texture(image_path: &str, key_color: Color, renderer: &Renderer) -> (Texture, (u32, u32)) {

    let err_msg: String = format!("Failed to load image at path {} into a surface", image_path);

    // sdl2_image LoadSurface Trait
    // - note `self` is not used in the Trait receiver position. So we do need a type hint.
    let mut image_surface: Surface = LoadSurface::from_file(Path::new(image_path))
                                            .ok().expect(&err_msg);

    image_surface.set_color_key(true, key_color).ok().expect("Failed to set color key");

    let w_h_size = image_surface.size();

    (renderer.create_texture_from_surface(image_surface)
        .ok().expect("Failed to create texture from image surface"),
     w_h_size)
}

pub fn make_rect(x_y: (i32, i32), w_h: (u32, u32)) -> Rect {
    Rect::new(x_y.0, x_y.1, w_h.0, w_h.1)
        .ok().expect("sdl create rect failed")
        .expect("width or height must not be 0")
}
