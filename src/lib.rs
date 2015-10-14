// This file is our crate root

// We need to link to these other crates
extern crate sdl2;

// We export all these modules
pub mod simplereblitted;
pub mod simplesurface;
pub mod simplewindow;
pub mod surfacekeyswaps;

// Crate internal modules
mod setup;
