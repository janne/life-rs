extern crate rand;
use std::io;
use std::thread;
static SIZE:i16 = 30;

struct World {
    data: [[bool; 30]; 30]
}

impl World {
    fn new() -> World {
        World { data: [[false; 30]; 30] }
    }

    fn randomize() -> World {
        let mut world = World::new();
        for row in 0..SIZE {
            for col in 0..SIZE {
                if rand::random() {
                    world.set_at(col,row);
                }
            }
        }
        world
    }

    fn draw(&self) {
        for row in (0..SIZE) {
            for col in (0..SIZE) {
                let c = if self.get_at(col, row) { "\x1B[32mo\x1B[0m" } else { "." };
                print!("{} ", c);
            }
            print!("\n");
        }
        print!("\n");
    }

    fn get_at(&self, x: i16, y: i16) -> bool {
        if y < 0 || x < 0 || y >= SIZE || x >= SIZE {
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
        let mut world = World::new();
        for row in (0..SIZE) {
            for col in (0..SIZE) {
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
    let mut world = World::randomize();
    loop {
        world = world.next();
        clear_screen();
        world.draw();
        thread::sleep_ms(250);
    }
}
