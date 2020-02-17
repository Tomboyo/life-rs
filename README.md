# Life, in Rust

This is a programming exercise to implement Conway's Game of Life in Rust.

The game is rendered using the [sdl2](https://github.com/Rust-SDL2/rust-sdl2)
library.

## Building

Call `cargo build` from the command line. Because we are using sdl2 for
rendering, you may need to install development tools like `libsdl2-dev` or
`gcc`. Follow the instructions in the
[sdl2 readme](https://github.com/Rust-SDL2/rust-sdl2) if you encounter any
problems.

## Run

Call `cargo run` to run the program, which will open a window and begin playing
Life with a board seeded randomly. It should look something like this:

![Example Gif](https://github.com/Tomboyo/life-rs/blob/master/images/life.gif?raw=true)

At any time, you may press Esc to close the window.
