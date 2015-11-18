// This file is our crate root

// We need to link to these other crates
extern crate num;
extern crate sdl2;
extern crate sdl2_image;

// We export all these modules
// These are the modules we want main.rs to be able to access as public symbols
pub mod alphablend;
pub mod colorkeying;
pub mod geometry;
pub mod simplereblitted;
pub mod simplesurface;
pub mod simplewindow;
pub mod spritesheet;
pub mod surfacekeyswaps;
pub mod texturerenderer;
pub mod viewport;

// Crate internal modules
// This means create a module (a namespace basically relative to here, the crate root)
// As there is no body (e.g. mod setup { ... }) rustc looks for setup.rs or ./setup/mod.rs
// Whatever it finds in setup.rs (in this case) is put into the `setup` namespace
// which can later be `use`d.
mod setup;
