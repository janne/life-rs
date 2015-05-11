extern crate rand;
use std::thread;

struct World {
    width: i16,
    height: i16,
    data: Vec<Vec<bool>>,
}

impl World {
    fn new(width: i16, height: i16) -> World {
        World { height: height, width: width, data: vec![vec![false; height as usize]; width as usize] }
    }

    fn randomize(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                if rand::random() {
                    self.set_at(col, row);
                }
            }
        }
    }

    fn draw(&self) {
        for row in (0..self.height) {
            for col in (0..self.width) {
                let c = if self.get_at(col, row) { "\x1B[32mo\x1B[0m" } else { "." };
                print!("{} ", c);
            }
            print!("\n");
        }
        print!("\n");
    }

    fn get_at(&self, x: i16, y: i16) -> bool {
        if y < 0 || x < 0 || y >= self.height || x >= self.width {
            return false
        }
        self.data[x as usize][y as usize]
    }

    fn set_at(&mut self, x: i16, y: i16) {
        self.data[x as usize][y as usize] = true;
    }

    fn count_at(&self, x: i16, y: i16) -> i16 {
        let mut count = 0;
        for xdiff in (-1..2) {
            for ydiff in (-1..2) {
                if xdiff == 0 && ydiff == 0 {
                    continue;
                }
                let test_x = x + xdiff;
                let test_y = y + ydiff;
                if self.get_at(test_x, test_y) {
                    count += 1;
                }
            }
        }
        count
    }

    fn next(&self) -> World {
        let mut world = World::new(self.width, self.height);
        for row in (0..self.height) {
            for col in (0..self.width) {
                let count = self.count_at(col, row);
                if count == 3 || (count == 2 && self.get_at(col, row)) {
                    world.set_at(col, row);
                }
            }
        }
        world
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[0;0H");
}

fn main() {
    let mut world = World::new(60, 30);
    world.randomize();
    loop {
        world = world.next();
        clear_screen();
        world.draw();
        thread::sleep_ms(250);
    }
}
