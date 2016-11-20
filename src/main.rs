extern crate ncurses;
extern crate pancurses;
extern crate libc;

use std::env;

mod sunyat;
fn main() {
    println!("Hello, world!");
    let args: Vec<_> = env::args().collect();
    sunyat::start_sunyat(&args[1], false, false);
}
