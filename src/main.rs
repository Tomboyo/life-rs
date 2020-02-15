extern crate rand;
extern crate sdl2;

use std::time::Duration;

use rand::Rng;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
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

struct Board {
    width: u32,
    cells: Vec<Cell>,
}

#[derive(PartialEq)]
enum Cell {
    Alive(),
    Dead(),
}

impl Board {
    fn new_random(width: u32, height: u32) -> Self {
        let mut rand = rand::thread_rng();
        let mut cells = Vec::new();
        
        for _count in 0..(width * height) {
            cells.push(if rand.gen() {
                Cell::Dead()
            } else {
                Cell::Alive()
            });
        }

        Board { width, cells }
    }

    fn advance(&mut self) {
        let mut next_cells: Vec<Cell> = vec![];

        for (position, cell) in self.cells.iter().enumerate() {
            let x = position as u32 / self.width;
            let y = position as u32 % self.width;
            let n = self.living_neighbors(x, y);
            next_cells.push(match cell {
                Cell::Alive() => {
                    if n == 2 || n == 3 {
                        Cell::Alive()
                    } else {
                        Cell::Dead()
                    }
                },
                Cell::Dead() => {
                    if n == 3 {
                        Cell::Alive()
                    } else {
                        Cell::Dead()
                    }
                },
            });
        }

        self.cells = next_cells;
    }

    fn living_neighbors(&self, x: u32, y: u32) -> u32 {
        let x: i64 = x as i64;
        let y: i64 = y as i64;
        let points: Vec<(i64, i64)> =
            vec![(x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
                 (x - 1, y    ), /*origin*/  (x + 1, y    ),
                 (x - 1, y + 1), (x, y + 1), (x + 1, y + 1)];
        points.iter()
            .map(|(x, y)| (self.width as i64 * x) + y)
            .map(|position| self.cells.get(position as usize))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .filter(|cell| **cell == Cell::Alive())
            .count() as u32
    }

    fn render(&self, canvas: &mut Canvas<Window>) {
        let width: u32 = 20;
        let height: u32 = 20;

        for (position, cell) in self.cells.iter().enumerate() {
            let y = (position as u32 / self.width) * width;
            let x = (position as u32 % self.width) * height;
            let color = match cell {
                Cell::Alive() => Color::RGB(0, 0, 0),
                Cell::Dead() => Color::RGB(255, 255, 255),
            };

            canvas.set_draw_color(color);
            let rect = Rect::new(x as i32, y as i32, width, height);
            canvas.fill_rect(rect).unwrap();
        }
    }
}
