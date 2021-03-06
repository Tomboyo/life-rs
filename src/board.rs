use rand::Rng;

use num_traits::ToPrimitive;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::vec2d::Vec2D;

#[derive(Debug)]
pub struct Board {
    data: Vec2D<Cell>,
}

#[derive(PartialEq, Debug)]
enum Cell {
    Alive(),
    Dead(),
}

impl Board {
    pub fn new_random(width: u32, height: u32) -> Self {
        let mut rand = rand::thread_rng();
        Board::new(width, height, &mut |_x, _y|
            if rand.gen() {
                Cell::Alive()
            } else {
                Cell::Dead()
            }
        )
    }

    fn new<F>(width: u32, height: u32, initializer: &mut F) -> Self
    where F: FnMut(u32, u32) -> Cell {
        Board {
            data: Vec2D::new(width, height, initializer)
        }
    }

    pub fn advance(&mut self) {
        self.data = Vec2D::from_vec(
            self.data.width,
            self.data.height,
            self.data.iter()
                .map(|((x, y), cell)| self.advance_cell(x, y, cell))
                .collect()
        ).unwrap();
    }

    fn advance_cell(&self, x: u32, y: u32, cell: &Cell) -> Cell {
        let n = self.living_neighbors(x, y);
        match cell {
            Cell::Alive() if n == 2 || n == 3 => Cell::Alive(),
            Cell::Alive() => Cell::Dead(),
            Cell::Dead() if n == 3 => Cell::Alive(),
            Cell::Dead() => Cell::Dead(),
        }
    }

    fn living_neighbors(&self, x: u32, y: u32) -> u32 {
        let x: i64 = x as i64;
        let y: i64 = y as i64;
        let points = [
                 (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
                 (x - 1, y    ), /*origin*/  (x + 1, y    ),
                 (x - 1, y + 1), (x, y + 1), (x + 1, y + 1)];
        points.iter()
            .map(|(x, y)| self.data.get(*x, *y))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .filter(|cell| **cell == Cell::Alive())
            .count() as u32 // always between 0 and 8.
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        let width: u32 = 20;
        let height: u32 = 20;
        let alive_color = Color::RGB(0, 0, 0);
        let dead_color = Color::RGB(255, 255, 255);

        self.data.iter()
            .map(|((x, y), cell)| {
                let x = (x * width).to_i32().unwrap();
                let y = (y * height).to_i32().unwrap();
                let color = match cell {
                    Cell::Alive() => alive_color,
                    Cell::Dead() => dead_color,
                };
                ((x, y), color)
            })
            .for_each(|((x, y), color)| {
                canvas.set_draw_color(color);
                let rect = Rect::new(x, y, width, height);
                canvas.fill_rect(rect).unwrap();
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_living_neighbors() {
        let board = Board::new(3, 3, &mut |_, _| Cell::Alive());
        let actual = board.living_neighbors(1, 1);
        assert_eq!(8, actual);
    }

    #[test]
    fn test_living_neighbors_upper_right() {
        let board = Board::new(3, 3, &mut |_, _| Cell::Alive());
        let actual = board.living_neighbors(2, 0);
        assert_eq!(3, actual);
    }

    #[test]
    fn test_living_neighbors_lower_left() {
        let board = Board::new(3, 3, &mut |_, _| Cell::Alive());
        let actual = board.living_neighbors(0, 2);
        assert_eq!(3, actual);
    }
}
