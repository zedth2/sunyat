
use ncurses;

use super::constants;

struct SatWin {
	terminal: [[char; constants::TERMINAL_HEIGHT]; constants::TERMINAL_WIDTH],
    win: ncurses::WINDOW,
    cur_X: i32,
    cur_Y: i32,
    max_X: i32,
    max_Y: i32,
}

impl SatWin {
	fn new() -> SatWin {
		let mut w = SatWin {
			terminal: [[' '; constants::TERMINAL_HEIGHT]; constants::TERMINAL_WIDTH],
			cur_X: 0,
			cur_Y: 0,
			win: ncurses::stdscr(),
			max_X: 0,
			max_Y: 0,
		};
		ncurses::getmaxyx(w.win, &mut w.max_X, &mut w.max_Y);
		w
	}

    fn setup_ncurses_terminal(&self) -> usize {
		return 255;
    }

}
