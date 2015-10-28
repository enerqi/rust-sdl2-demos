use std::path::{Path};

use sdl2;
use sdl2_image::{self, LoadTexture, INIT_PNG, INIT_JPG};
//use sdl2::VideoSubsystem;
use sdl2::render::{Renderer, Texture};

pub struct BasicWindow {

    pub sdl_context: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub window: sdl2::video::Window
}

pub fn init(window_title: &str, width: u32, height: u32) -> BasicWindow {

    let sdl_context: sdl2::Sdl = sdl2::init().unwrap();

    let video_subsystem: sdl2::VideoSubsystem = sdl_context.video().unwrap();

    let mut window_builder: sdl2::video::WindowBuilder = video_subsystem.window(window_title, width, height);

    let window: sdl2::video::Window = window_builder.position_centered().opengl().build().unwrap();

    sdl2_image::init(INIT_PNG | INIT_JPG);

    BasicWindow {sdl_context: sdl_context,
                 video_subsystem: video_subsystem,
                 window: window}
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
