
Windows [ Surface ]


Surface ~= 2D Image
        - loaded from file as image
        - drawn etc.
    Note: surfaces live in system ram, not gpu video ram and are software rendered.

SDL_Init => Can choose to use only certain subsystems, e.g. video
    *Rust: generic init all? init()->SdlResult<Sdl>
SDL_GetError => generic last error string

SDL_FillRect => draw on a surface got from a Window
SDL_UpdateWindowSurface => or you won't see anything, still in backbuffer etc.
SDL_Delay(ms) => blocking wait
SDL_GetWindowSurface =>
    *Rust: video::Window.surface
