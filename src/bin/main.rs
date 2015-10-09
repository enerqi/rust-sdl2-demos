// Our binary must link to our library crate
// The library crate itself links to sdl2 etc.
extern crate rust_sdl2_demos;

use rust_sdl2_demos::simplewindow as sw;
use rust_sdl2_demos::simplesurface as ss;
use rust_sdl2_demos::simplereblitted as sr;

fn main() {

    //sw::simple_window();
    //ss::simple_surface();
    sr::simple_reblitted();
}
