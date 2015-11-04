// Our binary must link to our library crate
// The library crate itself links to sdl2 etc.
extern crate rust_sdl2_demos;

// #[macro_use]
//extern crate maplit;

use std::io;
use std::io::Write;

use rust_sdl2_demos::colorkeying as ck;
use rust_sdl2_demos::geometry as g;
use rust_sdl2_demos::simplereblitted as sr;
use rust_sdl2_demos::simplesurface as ss;
use rust_sdl2_demos::simplewindow as sw;
use rust_sdl2_demos::surfacekeyswaps as sks;
use rust_sdl2_demos::texturerenderer as tr;
use rust_sdl2_demos::viewport as v;

fn main() {

        // Each function has its own distinct type so need to give the compiler a hand
        // either typing an array of funcs or the `as fn()` (void -> void) example below
    let demos_info = [(sw::simple_window as fn(),
                  "Render a window that closes when q pressed, verbose"),
                 (ss::simple_surface,
                  "Render a surface image onto a window"),
                 (sr::simple_reblitted,
                  "Render a surface image onto a window, ensure reblitted if repainting required"),
                 (sks::surface_keyswaps,
                  "Render different surface images according to key pressed"),
                 (tr::texture_render,
                  "Hardware rendering images and loading pngs with sdl image."),
                 (g::point_drawer,
                  "Render basic geometry. No images required."),
                 (v::viewports,
                  "Drawing a texture to multiple view ports."),
                 (ck::color_keying,
                  "Draw background and sprite image with a specified transparency colour.")];


    println!("Rust SDL2 Demos");
    for (index, tuple) in demos_info.iter().enumerate() {
        let demo_description = tuple.1;
        println!("[{}] {}", index, demo_description);
    }

    let mut input = String::new();
    let demo_number: usize;
    loop {
        print!("Choose a demo [0-{}]: ", demos_info.len()-1);
        io::stdout().flush().ok().expect("flush failed");

        input.clear();
                          // ok (Result -> Option). expect(Option -> T with panic message)
        io::stdin().read_line(&mut input).ok().expect("read_line failed");

                            // read_line includes the newline at the end
        if let Ok(n) = input.trim().parse::<usize>() {
            if n < demos_info.len() {
                demo_number = n;
                break;
            }
        }
    }

    let demo = demos_info[demo_number].0;
    demo();
}
