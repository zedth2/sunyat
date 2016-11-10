extern crate ncurses;
extern crate pancurses;

use std::env;

mod sunyat;
fn main() {
    println!("Hello, world!");
    let args: Vec<_> = env::args().collect();
    sunyat::start_sunyat(&args[0], false, false);
}
