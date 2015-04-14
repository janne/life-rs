extern crate rand;
use std::io;
static SIZE:usize = 10;

struct World {
    data: [bool; 100],
}

impl World {
    fn new() -> World {
        World { data: [false; 100] }
    }

    fn randomize() -> World {
        let mut world = World::new();
        for row in 0..10 {
            for col in 0..10 {
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
                let c = if self.get_at(col, row) { "o" } else { "." };
                print!("{} ", c);
            }
            print!("\n");
        }
        print!("\n");
    }

    fn get_at(&self, x: usize, y: usize) -> bool {
        if y >= 10 || x >= 10 {
            return false
        }
        self.data[y*10+x]
    }

    fn set_at(&mut self, x: usize, y: usize) {
        self.data[y*10+x] = true;
    }

    fn count_at(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for xdiff in (-1..2) {
            for ydiff in (-1..2) {
                if xdiff == 0 && ydiff == 0 {
                    continue;
                }
                let test_x: i32 = x as i32 + xdiff;
                let test_y: i32 = y as i32 + ydiff;
                if self.get_at(test_x as usize, test_y as usize) {
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

fn read_line() {
    let _ = io::stdin().read_line(&mut String::new());
}

fn main() {
    let mut world = World::randomize();
    loop {
        world = world.next();
        world.draw();
        read_line();
    }
}
