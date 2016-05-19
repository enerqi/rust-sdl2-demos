use image;
use image::GenericImage;
use num::FromPrimitive;
use sdl2::event::Event;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::{Renderer, Texture, TextureQuery};
use sdl2::surface::{Surface, SurfaceRef};
use sdl2_ttf::Font;
use sdl2_image;
use sdl2_image::{SaveSurface, LoadSurface};
use sdl2_sys::pixels::SDL_PixelFormat;
use setup;

use std::fs::File;
use std::path::Path;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

pub fn image_test() {


    let mut basic_window_setup = setup::init("Timer", SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut events = basic_window_setup.sdl_context.event_pump().unwrap();
    let mut renderer = basic_window_setup.window
                                         .renderer()
                                         .present_vsync()
                                         .accelerated()
                                         .build()
                                         .unwrap();

    let image_path = "mask-60x60.png";
    let err_msg: String = format!("Failed to load image at path {} into a surface", image_path);
    // sdl2_image LoadSurface Trait
    // - note `self` is not used in the Trait receiver position. So we do need a type hint.
    let mut image_surface: Surface = LoadSurface::from_file(Path::new(image_path))
                                         .expect(&err_msg);
    let raw_pixel_data = image_surface.without_lock().unwrap();

    println!("Path {:?}", Path::new(image_path));
    println!("width, height, pitch, size ({:?}, {:?}, {:?}, {:?})",
             image_surface.width(), image_surface.height(), image_surface.pitch(), image_surface.size());
    println!("raw_pixel_data u8 length {:?}", raw_pixel_data.len());

    let bpp = raw_pixel_data.len() as u32 / (image_surface.height() * image_surface.pitch());
    println!("bytes per pixel: {:?}", bpp);

    // pixel [x, y] ->

    // for n in raw_pixel_data.iter().take(50) {
    //     println!("{:?}", n);
    // }

    // the left over bytes in the pitch are 0 -> 3 bytes of 28 are not used
    // > 127 then ON
    // < 127 then OFF?

    // Well when we carefully ensure every colour is pure black or white the pixel format is changed to just use 0/1 for the values
    // Presumably .BitsPerPixel switches
    unsafe {

        // high level sdl2::pixels::PixelFormatEnum (vs sdl2::pixels::PixelFormat which wraps the pointer to ll version)
        // Enum has byte size of pixels functions
        // Enum has from_u64 to parse from ll pixelformat enum presumably

        // To actually see the types here we need to add the crate sdl2_sys to the project. type SDL_PixelFormatEnum = uint32_t;
        let ll_pix_format_ptr: *mut SDL_PixelFormat = image_surface.pixel_format().raw();
        println!("BytesPerPixel: {:?}", (*ll_pix_format_ptr).BytesPerPixel);
        println!("BitsPerPixel: {:?}", (*ll_pix_format_ptr).BitsPerPixel);

        let pixFormat: PixelFormatEnum = FromPrimitive::from_u64((*ll_pix_format_ptr).format as u64).unwrap();
        // Or
        // use sys::pixels as ll;
        // unsafe{ FromPrimitive::from_u64(ll::SDL_GetWindowPixelFormat(ll_pix_format_ptr.raw()) as u64).unwrap() }
        println!("PixelFormatEnum: {:?}", pixFormat);
    }


    use image::{DynamicImage};
    use image::DynamicImage::*;
    use image::Luma;
    let img: DynamicImage = image::open(&Path::new(image_path)).unwrap();
    match img {
                // type GrayImage = ImageBuffer<Luma<u8>, Vec<u8>>;
                // pub struct ImageBuffer<P: Pixel, Container>
                // Trait image::Pixel: Copy + Clone ...
         ImageLuma8(ref grayImage) => println!("ImageLuma8(grayImage)"),
         ImageLumaA8(ref grayAlphaImage) => println!("ImageLumaA8(grayAlphaImage)"),
         ImageRgb8(ref rgbImage) => println!("ImageRgb8(rgbImage)"),
         ImageRgba8(ref rgbAlphaImage) => println!("ImageRgba8(rgbAlphaImage)"),
    }
    // The dimensions method returns the images width and height
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's ColorType
    println!("{:?}", img.color());

    for y in 0..img.dimensions().1 {
        println!("({:?}, {:?}): {:?}", 10, y, img.get_pixel(10, y));

        match img {
                // type GrayImage = ImageBuffer<Luma<u8>, Vec<u8>>;
                // pub struct ImageBuffer<P: Pixel, Container>
                // Trait image::Pixel: Copy + Clone ...
         ImageLuma8(ref grayImage) => {
            let pix: &Luma<u8> = grayImage.get_pixel(10, y);
            println!("LumaPix: {:?}", pix);
         },
         // ImageLumaA8(ref grayAlphaImage) => println!("ImageLumaA8(grayAlphaImage)"),
         // ImageRgb8(ref rgbImage) => println!("ImageRgb8(rgbImage)"),
         // ImageRgba8(ref rgbAlphaImage) => println!("ImageRgba8(rgbAlphaImage)"),
         _ => {},
    }
    }

    setup::quit();
}

