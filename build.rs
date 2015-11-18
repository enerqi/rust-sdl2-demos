use std::env;

fn main() {

    let libs_dir =
        if let Ok(dir) = env::var("SDL_LIBS_DIR") {
            Some(dir)
        }
        else {
            if let Ok(cargo_root_dir) = env::var("CARGO_MANIFEST_DIR") {
                Some(format!("{}/sdl_libs", cargo_root_dir))
            }
            else {
                None
            }
        };

    if let Some(libs) = libs_dir {
        println!("cargo:rustc-flags=-L {}", libs);
    }
}
