use std::path::Path;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2_ttf;
use sdl2_ttf::Font;

use setup;


const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new_unwrap($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

// SDL_ttf creates a new image from a font and a colour
// So, we will be loading our image from text rendered by SDL_ttf instead of a file.
pub fn font_rendering() {

    let basic_window_setup = setup::init("Font Rendering", SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut events = basic_window_setup.sdl_context.event_pump().unwrap();
    let ttf = basic_window_setup.ttf_context;
    let mut renderer = basic_window_setup.window
                                         .renderer()
                                         .present_vsync()
                                         .accelerated()
                                         .build()
                                         .unwrap();


    // Load a font
    let path: &Path = Path::new("resources/lazy.ttf");
    // let attr = fs::metadata(path).ok().expect("cannot query file");
    let font_px_size = 128;
    let font = ttf.load_font(&path, font_px_size)
                                 .ok().expect("Failed to load font");

    // render a surface, and convert it to a texture bound to the renderer
    let font = sdl2_ttf::Font::from_file(&path, font_px_size)
                   .ok()
                   .expect("Failed to load font");
    let surface = font.render("Hello Rust!").blended(Color::RGBA(255, 0, 0, 255)).unwrap();

    let mut text_texture = renderer.create_texture_from_surface(&surface)
                                   .ok()
                                   .expect("Failed to create texture from image surface");
    let TextureQuery { width, height, .. } = text_texture.query();

    // If the example text is too big for the screen, downscale it (and center irregardless)
    let padding = 64;
    let target = get_centered_rect(width,
                                   height,
                                   SCREEN_WIDTH - padding,
                                   SCREEN_HEIGHT - padding);

    'event: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => break 'event,
                Event::KeyDown{keycode: Some(::sdl2::keyboard::Keycode::Q), ..} => break 'event,
                _ => continue,
            }
        }

        renderer.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        renderer.clear();
        renderer.copy(&mut text_texture, None, Some(target));
        renderer.present();
    }

    setup::quit();
}

// Scale fonts to a reasonable size when they're too big (though they might look less smooth)
fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            println!("Scaling down! The text will look worse!");
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            println!("Scaling down! The text will look worse!");
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx = (SCREEN_WIDTH as i32 - w) / 2;
    let cy = (SCREEN_HEIGHT as i32 - h) / 2;
    rect!(cx, cy, w, h)
}
