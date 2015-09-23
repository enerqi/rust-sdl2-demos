// Our binary must link to our library crate
// The library crate itself links to sdl2 etc.
extern crate rust_sdl2_demos;

use rust_sdl2_demos::simplewindow as sw;

fn main() {

    sw::simple_window();
}
