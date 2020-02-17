extern crate num_traits;
extern crate rand;
extern crate sdl2;

mod vec2d;
mod board;

use std::time::Duration;

use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use crate::board::Board;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Conway's Game of Life", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut board = Board::new_random(40, 30);

    loop {
        if trap_exit_events(&mut event_pump) {
            break;
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        board.render(&mut canvas);
        board.advance();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 4));
    }
}

fn trap_exit_events(pump: &mut EventPump) -> bool {
    pump.poll_iter().any(|event| match event {
        Event::Quit{
            ..
        } => true,
        Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
        } => true,
        _ => false,
    })
}
