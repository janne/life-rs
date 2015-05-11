Life in Rust
------------

Add to Cargo.toml:

```toml
[dependencies]
life = "*"
```

Use like this:

```rust
extern crate life;
use std::thread;
use life::Life;

fn main() {
    let mut life = Life::new(60, 30);
    loop {
        life.next();
        print!("\x1B[2J\x1B[0;0H");
        for row in &life.world {
            let line = row.iter().map(|&a| if a { "o" } else { "." } ).collect::<Vec<&str>>().concat();
            println!("{}", line);
        }
        thread::sleep_ms(250);
    }
}
```
