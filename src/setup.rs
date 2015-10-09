
use sdl2;
//use sdl2::VideoSubsystem;

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

    BasicWindow {sdl_context: sdl_context,
                 video_subsystem: video_subsystem,
                 window: window}
}

