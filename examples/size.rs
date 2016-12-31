extern crate termion;

use termion::{terminal_size, init};

fn main() {
    init();
    println!("Size is {:?}", terminal_size().unwrap())
}
