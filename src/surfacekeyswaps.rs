
// show different images depending upon what key is pressed

use setup;

enum KeyPressSurface {

    Default,
    Up,
    Down,
    Left,
    Right
}

pub fn surface_keyswaps() {

    let basic_window_setup = setup::init("Image swaps", 640, 480);

    let mut events = basic_window_setup.sdl_context.event_pump().unwrap();



}

