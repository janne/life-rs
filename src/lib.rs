extern crate rand;
use std::mem;

pub struct Life {
    pub world: Vec<Vec<bool>>
}

impl Life {
    pub fn new(width: usize, height: usize) -> Life {
        Life { world: vec![vec![false; width as usize]; height as usize] }
    }

    fn width(&self) -> usize {
        self.world[0].len()
    }

    fn height(&self) -> usize {
        self.world.len()
    }

    pub fn randomize(&mut self) {
        for row in 0..self.height() {
            for col in 0..self.width() {
                if rand::random() {
                    self.set_at(col, row);
                }
            }
        }
    }

    pub fn get_at(&self, x: isize, y: isize) -> bool {
        if y < 0 || x < 0 || y >= self.height() as isize || x >= self.width() as isize {
            return false
        }
        self.world[y as usize][x as usize]
    }

    fn set_at(&mut self, x: usize, y: usize) {
        self.world[y][x] = true;
    }

    fn count_at(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for xdiff in (-1..2) {
            for ydiff in (-1..2) {
                if xdiff == 0 && ydiff == 0 {
                    continue;
                }
                let test_x: isize = x as isize + xdiff;
                let test_y: isize = y as isize + ydiff;
                if self.get_at(test_x, test_y) {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn next(&mut self) {
        let world = &mut Life::new(self.width(), self.height());
        for row in (0..self.height()) {
            for col in (0..self.width()) {
                let count = self.count_at(col, row);
                if count == 3 || (count == 2 && self.get_at(col as isize, row as isize)) {
                    world.set_at(col, row);
                }
            }
        }
        mem::swap(self, world);
    }
}
