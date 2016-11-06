
use ncurses;
use pancurses;

use std::str;

use super::constants;

const ERR_NCURSES_INIT: &'static str = "\tCould not initialize ncurses\n";

const ERR_NCURSES_CBREAK: &'static str = "\tCould not disable character buffering\n";

const ERR_NCURSES_NODELAY: &'static str = "\tCould not disable blocking on \"getch\"\n";

const ERR_NCURSES_NOECHO: &'static str = "\tCould not disable echo\n";

const ERR_NCURSES_KEYPAD: &'static str = "\tCould not enable keypad usage\n";

const ERR_NCURSES_CURSOR: &'static str = "\tCould not modify cursor\n";


pub struct SatWin {
	terminal: [[char; constants::TERMINAL_HEIGHT as usize]; constants::TERMINAL_WIDTH as usize],
    pub mainWin: pancurses::Window,
    cur_X: i32,
    cur_Y: i32,
    max_X: i32,
    max_Y: i32,
}

impl SatWin {
	pub fn new() -> SatWin {
		let mut w = SatWin {
			terminal: [[' '; constants::TERMINAL_HEIGHT as usize]; constants::TERMINAL_WIDTH as usize],
			cur_X: 0,
			cur_Y: 0,
			mainWin: pancurses::initscr(),
			max_X: 0,
			max_Y: 0,
		};
		w.cur_X = w.mainWin.get_max_x();
		w.cur_Y = w.mainWin.get_max_y();
		//pancurses::getmaxyx(w.win, &mut w.max_X, &mut w.max_Y);
		w
	}

	pub fn setup_ncurses_terminal(&self) -> usize {
		//pancurses::initscr();

		if pancurses::has_colors(){
			pancurses::start_color();
		}

		if pancurses::ERR == pancurses::cbreak() {
			println!("ERROR : {}", ERR_NCURSES_CBREAK);
			return constants::EXT_ERR_NCURSES;
		}
		if pancurses::ERR == pancurses::noecho(){
			println!("ERROR : {}", ERR_NCURSES_NOECHO);
			return constants::EXT_ERR_NCURSES;
		}

		if pancurses::ERR == self.mainWin.nodelay(true){
			println!("ERROR : {}", ERR_NCURSES_NODELAY);
		}

		if pancurses::ERR == self.mainWin.keypad(true){
			println!("ERROR : {}", ERR_NCURSES_KEYPAD);
			return constants::EXT_ERR_NCURSES;
		}

		if pancurses::ERR == pancurses::curs_set(1){
			println!("ERROR : {}", ERR_NCURSES_CURSOR);
			return constants::EXT_ERR_NCURSES;
		}

		pancurses::init_pair(1, pancurses::COLOR_WHITE, pancurses::COLOR_BLACK);
		self.mainWin.bkgd(pancurses::COLOR_PAIR(1));

		return constants::EXIT_SUCCESS;
	}

	pub fn terminal_restore(&self){
		let mut y: i32 = 0;
		let mut x: i32 = 0;
		self.mainWin.erase();
		for y in 0..constants::TERMINAL_HEIGHT{
		//for(y = 0; y < constants::TERMINAL_HEIGHT; ++y){
			for x in 0..constants::TERMINAL_WIDTH {
			//for(x = 0; x < constants::TERMINAL_WIDTH; ++x){
				let curChar = &[self.terminal[x as usize][y as usize] as u8];
				self.mainWin.mvprintw(y, x, str::from_utf8(curChar).unwrap());
			}
		}
		self.mainWin.mv(self.cur_X, self.cur_Y);
	}

}
