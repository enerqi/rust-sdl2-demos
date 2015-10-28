// This file is our crate root

// We need to link to these other crates
extern crate sdl2;
extern crate sdl2_image;

// We export all these modules
pub mod simplereblitted;
pub mod simplesurface;
pub mod simplewindow;
pub mod surfacekeyswaps;
pub mod texturerenderer;

// Crate internal modules
mod setup;
