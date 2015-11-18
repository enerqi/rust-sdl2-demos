
# rust-sdl2-demos

Playing with SDL2 via the Rust programming language.


## Building

The SDL static C libs for libSDL2_*, libSDL2_ttf, libSDL2_image are required for the build. 
These development C Libs are found at [libsdl.org](libsdl.org):

[https://www.libsdl.org/projects/SDL_ttf/](sdl_ttf 2)
[https://www.libsdl.org/projects/SDL_image/](sdl_image 2)
... etc.

`./build.rs` is the pre-main build step ran by cargo. It adds the directory designated by the environment variable `SDL_LIBS_DIR` to rustc link library search path so you do not have to put the sdl2 libs within the rust installation directory (e.g. C:\Program Files\Rust stable 1.4\bin\rustlib\x86_64-pc-windows-gnu\lib). For convenience, if `SDL_LIBS_DIR` is not set we use the libraries in `./sdl_libs`.

## Running 

DLLs are include for convenience in the project root.
We could also put the dll in C:\Windows\System32 or C:\Windows\SysWOW64 though we should never do that for anything but our own machine as other applications may expect to use a different version.
Anywhere on the PATH should do.
