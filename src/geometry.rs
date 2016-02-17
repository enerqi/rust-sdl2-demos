use num::range_step;

use sdl2;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

use setup;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

pub fn point_drawer() {
    let basic_window_setup = setup::init("Geometry Rendering", SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut events = basic_window_setup.sdl_context.event_pump().unwrap();

    let mut renderer = basic_window_setup.window
                                         .renderer()
                                         .present_vsync()
                                         .accelerated()
                                         .build()
                                         .unwrap();

    let black = Color::RGB(0, 0, 0);
    let white = Color::RGB(0xff, 0xff, 0xff);
    let red = Color::RGB(0xff, 0, 0);
    let green = Color::RGB(0, 0xff, 0);
    let blue = Color::RGB(0, 0, 0xff);
    let yellow = Color::RGB(0xff, 0xff, 0);

    let mut bright = false;
    let mut frame = 0;

    // loop until we receive a QuitEvent
    'event: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => break 'event,
                // keycode: Option<KeyCode>
                // https://doc.rust-lang.org/book/patterns.html
                Event::KeyDown{keycode: Some(sdl2::keyboard::Keycode::Q), ..} => break 'event,
                _ => continue,
            }
        }

        // Clear screen - white clear colour on each frame
        // SDL_SetRenderDrawColor( gRenderer, 0xFF, 0xFF, 0xFF, 0xFF );
        // SDL_RenderClear( gRenderer );
        renderer.set_draw_color(if bright {
            white
        } else {
            black
        });
        frame += 1;
        if frame % 60 == 0 {
            bright = !bright;
        }
        renderer.clear(); // clear with current draw color

        let rect = Rect::new(SCREEN_WIDTH as i32 / 4,
                             SCREEN_HEIGHT as i32 / 4,
                             SCREEN_WIDTH / 2,
                             SCREEN_HEIGHT / 2)
                       .ok()
                       .expect("sdl create rect failed")
                       .expect("width or height must not be 0");
        renderer.set_draw_color(red);
        renderer.fill_rect(rect);

        let unfilled_rect = Rect::new(SCREEN_WIDTH as i32 / 6,
                                      SCREEN_HEIGHT as i32 / 6,
                                      SCREEN_WIDTH * 2 / 3,
                                      SCREEN_HEIGHT * 2 / 3)
                                .ok()
                                .expect("sdl create rect failed")
                                .expect("width or height must not be 0");
        renderer.set_draw_color(green);
        renderer.draw_rect(unfilled_rect);

        renderer.set_draw_color(blue);
        renderer.draw_line(Point::new(0, SCREEN_HEIGHT as i32 / 2),
                           Point::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32 / 2));

        renderer.set_draw_color(yellow);
        // future (nightly) rust has `step_by` instead of the extern num crate `range_step`
        for i in range_step(0, SCREEN_HEIGHT, 4) {
            renderer.draw_point(Point::new(SCREEN_WIDTH as i32 / 2, i as i32));
        }

        renderer.present(); // screen update from the backbuffer
    }

    setup::quit();
}
