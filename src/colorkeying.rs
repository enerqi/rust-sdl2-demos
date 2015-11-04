
pub fn color_keying() {


    // Note every Texuture is owned by a Renderer. All drops not dropped
    // beforehand are nuked when the Renderer is dropped.

    // SurfaceRef.color_key (getter)
    // SurfaceRef.set_color_key(boolean, Color)

    // Alright, because set_color_key is a function on SurfaceRef, does
    // that mean we cannot use setup::load_image to create a Texture
    // Presumably we need to load a surface before doing any conversion
    // to texture?

    // Can sdl image do a plain IMG_load to get an SDL_Surface before we
    // do renderer.create_texture_from_surface(surface)?
    // how to get the surface from the file name pointing to the texture?
    //
    // sdl2_image ::LoadSurface trait. .from_file(...path) for Surface
    // vs
    // sdl2_image ::LoadTexture trait. .load_texture(...path) for Renderer
    //
    //

    // sdl_texture (Texture) (null) default
    // texture's: w & h (0, 0)
    //
    // load from file
    // render_at!: x & y
    // cleanup

}
