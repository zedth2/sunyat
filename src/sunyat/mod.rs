use std::io::Read;
use std::fs::File;
use std::string::String;

use ncurses;
use pancurses;
use libc::isprint;

pub mod constants;
use self::constants::*;

mod sat_scr;

struct SunyAT {
	//terminal: [[u8; constants::TERMINAL_HEIGHT]; TERMINAL_WIDTH+1];
	linefeed_buffered: bool,
	debug: bool,
	clock_ticks: usize,
	zero_flag: bool,
	sign_flag: bool,
	interrupt_flag: bool,
	ram: [u8; SIZE_APP_RAM],
	registers: [u8; SIZE_REG],
}

impl Default for SunyAT {
	fn default() -> SunyAT {
		let mut newSat = SunyAT { linefeed_buffered: false, debug: false, clock_ticks: 0, zero_flag: false, sign_flag: false, interrupt_flag: false, ram: [0; constants::SIZE_APP_RAM], registers: [0; constants::SIZE_REG]};
		newSat.registers[0] = 0;
		newSat.registers[1] = 0;
		newSat.registers[2] = 0;
		newSat.registers[3] = NUM_SYS_REG as u8;
		newSat.registers[4] = SIZE_APP_RAM as u8;
		newSat
	}
}

pub fn start_sunyat(rom: &String, lState: bool, lDebug: bool) -> usize{
	//let clock_start = unsafe {libc::clock()}; //Use this to pause eventually
	let mut reVal = EXIT_SUCCESS;
	let mut curSunyAT = SunyAT { ..Default::default() };
	let mut win: sat_scr::SatWin;
	if lState {
		reVal = load_state(&mut curSunyAT, rom);
	} else {
		reVal = load_rom(&mut curSunyAT, rom);
	}
	if EXIT_SUCCESS != reVal {
		return reVal;
	}



	//if !lDebug {
		win = sat_scr::SatWin::new();
		if EXT_ERR_NCURSES == win.setup_ncurses_terminal(){
			return EXT_ERR_NCURSES;
		}
	//} else {

	//}



	sunyat_execute(&mut curSunyAT, &mut win, lDebug);


	return reVal;
}

fn load_rom(sunyat: &mut SunyAT, rom: &String) -> usize {
	let mut file_buffer: [u8; SIZE_APP_RAM] = [0; SIZE_APP_RAM];
	let mut app_msg: [u8; SIZE_APP_MSG] = ['a' as u8; SIZE_APP_MSG]; //This should be char so we can read the app message thats stored in the first bytes of the rom
	let mut inFile = match File::open(rom){
		Ok(file) => file,
		Err(err) => {
			println!("Error: {}", err);
			return EXT_ERR_FILE_NOT_OPEN;
		},
	};
	let size_msg = match inFile.read(&mut app_msg[..]){
		Ok(amt) => amt,
		Err(err) => {
			println!("ERROR : {}", err);
			return EXT_ERR_FILE_READ;
		},
	};
	let size_rom = match inFile.read(&mut file_buffer[..]){
		Ok(amt) => amt,
		Err(err) => {
			println!("ERROR : {}", err);
			return EXT_ERR_FILE_READ;
		},
	};
	/*match inFile.read_to_end(&mut file_buffer){
		Ok(file) => file,
		Err(err) =>{
			println!("Error: {}", err);
			return EXT_ERR_FILE_READ;
		},
	};*/
	if SIZE_APP_RAM != file_buffer.len(){ //Should this be > ?
		println!("Error: {}", ERR_BYTE_SIZE);
		return EXT_ERR_ROM_BIG;
	} //Deleted the else that was in original C code.

	sunyat.ram = file_buffer ;



	return EXIT_SUCCESS;
}

fn load_state(sunyat: &mut SunyAT, rom: &String) -> usize
{
return 255;
}

fn sunyat_execute(sat: &mut SunyAT, scr: &mut sat_scr::SatWin, lDebug: bool){
	let mut pause = false;
	let mut terminal_too_small_prev_cycle = false;

	loop {
		let mut opcode: u8;
		let mut sreg: usize;
		let mut dreg: usize;
		let mut mem: u8;
		let mut imm: i8;
		let mut cmp_result: u8;

		let curHeight: i32 = scr.mainWin.get_max_y();
		let curWidth: i32 = scr.mainWin.get_max_x();

		if curWidth < TERMINAL_WIDTH || curHeight < TERMINAL_HEIGHT {
			let mut x: i32;
			let mut y: i32;

			terminal_too_small_prev_cycle = true;

			for y in 0..curHeight {
			//for (y = 0; y < curHeight; ++y){
				scr.mainWin.mv(y, 0);
				for x in 0..curWidth {
				//for(x = 0 ; x < curWidth ; ++x){
					scr.mainWin.addch('@');
				}
			}
			let cx: i32 = curWidth / 2;
			let cy: i32 = curHeight / 2;
			scr.mainWin.mvprintw(cy-1, cx-10, "                    ");
			scr.mainWin.mvprintw(cy, cx-10,   "  Window too small  ");
			scr.mainWin.mvprintw(cy+1, cx-10, " resize to >= 80x24 ");
			scr.mainWin.mvprintw(cy+2, cx-10, "                    ");
			scr.mainWin.refresh();
			continue;
		}

		if terminal_too_small_prev_cycle {
			scr.terminal_restore();
			terminal_too_small_prev_cycle = false;
			scr.mainWin.refresh();
		}

		sat.clock_ticks += 1;

		if sat.registers[REG_PC] > ((SIZE_APP_RAM - 2) as u8) {
			println!("ERROR : {} {}", sat.registers[REG_PC], sat.clock_ticks);
			println!("ERROR : {}", ERR_INVALID_PC);
			return;
		}


		sat.registers[REG_IRH] = sat.ram[sat.registers[REG_PC] as usize];
		sat.registers[REG_PC] += 1;
		sat.registers[REG_IRL] = sat.ram[sat.registers[REG_PC] as usize];
		sat.registers[REG_PC] += 1;

		opcode = get_opcode(sat.registers[REG_IRH]);

		sreg = get_grwp(sat.registers[REG_WIN], get_sreg(sat.registers[REG_IRL])) as usize;

		dreg = get_grwp(sat.registers[REG_WIN], get_dreg(sat.registers[REG_IRH])) as usize;

		imm = get_imm(sat.registers[REG_IRL] as i8);
		mem = get_mem(sat.registers[REG_IRL]);

		if lDebug {

		}
		//println!("OPCODE : {}", opcode);
		match opcode {
			OPCODE_MOV_RR => {
				sat.registers[dreg] = sat.registers[sreg];
				//println!("OPCODE_MOV_RR");
			},
			OPCODE_MOV_RI => {
				sat.registers[dreg] = imm as u8;
				//println!("OPCODE_MOV_RI");
			},

			OPCODE_ADD_RR => {
				sat.registers[dreg] = sat.registers[dreg] + sat.registers[sreg];
				let re = set_flags(sat.registers[dreg] as i8);
				sat.zero_flag = re.0;
				sat.sign_flag = re.1;
				//println!("OPCODE_ADD_RR");
			},

			OPCODE_ADD_RI => {
				sat.registers[dreg] = (sat.registers[dreg] as i8 + imm) as u8;
				let re = set_flags(sat.registers[dreg] as i8);
				sat.zero_flag = re.0;
				sat.sign_flag = re.1;
				//println!("OPCODE_ADD_RI");
			},

			OPCODE_SUB_RR => {
				sat.registers[dreg] = sat.registers[dreg] - sat.registers[sreg];
				let re = set_flags(sat.registers[dreg] as i8);
				sat.zero_flag = re.0;
				sat.sign_flag = re.1;
				//println!("OPCODE_SUB_RR");
			},

			OPCODE_MUL_RR => {
				sat.registers[dreg] = sat.registers[dreg] * sat.registers[sreg];
				let re = set_flags(sat.registers[dreg] as i8);
				sat.zero_flag = re.0;
				sat.sign_flag = re.1;
				//println!("OPCODE_MUL_RR");
			},

			OPCODE_MUL_RI => {
				sat.registers[dreg] = (sat.registers[dreg] as i8 * imm) as u8;
				let re = set_flags(sat.registers[dreg] as i8);
				sat.zero_flag = re.0;
				sat.sign_flag = re.1;
				//println!("OPCODE_MUL_RI");
			},

			OPCODE_DIV_RR => {
				if 0 == sat.registers[sreg] {
					println!("{}", ERR_DIV_ZERO);
					return;
				}
				sat.registers[dreg] = (sat.registers[dreg] as i8 / imm) as u8;
				let re = set_flags(sat.registers[dreg] as i8);
				sat.zero_flag = re.0;
				sat.sign_flag = re.1;
				//println!("OPCODE_MUL_RI");
			},

			OPCODE_DIV_RI => {
				if 0 == sat.registers[sreg] {
					println!("{}", ERR_DIV_ZERO);
					return;
				}
				sat.registers[dreg] = (sat.registers[dreg] as i8 / imm) as u8;
				let re = set_flags(sat.registers[dreg] as i8);
				sat.zero_flag = re.0;
				sat.sign_flag = re.1;
				//println!("OPCODE_DIV_RI");
			},

			OPCODE_CMP_RR => {
				let re = set_flags((sat.registers[dreg] - sat.registers[sreg]) as i8);
				sat.zero_flag = re.0;
				sat.sign_flag = re.1;
				//println!("OPCODE_CMP_RR");
			},

			OPCODE_CMP_RI => {
				let re = set_flags(sat.registers[dreg] as i8 - imm);
				sat.zero_flag = re.0;
				sat.sign_flag = re.1;
				//println!("OPCODE_CMP_RI");
			},

			OPCODE_JMP_M => {
				if mem as usize >= SIZE_APP_RAM {
					println!("{}", ERR_JMP_RANGE);
					return;
				}
				sat.registers[REG_PC] = mem;
				//println!("OPCODE_JMP_M");
			},

			OPCODE_JEQ_M => {
				if sat.zero_flag {
					sat.registers[REG_PC] = mem;
				}
				//println!("OPCODE_JEQ_M");
			},

			OPCODE_JNE_M => {
				if !sat.zero_flag {
					sat.registers[REG_PC] = mem;
				}
				//println!("OPCODE_JNE_M");
			},

			OPCODE_JGR_M => {
				if !sat.sign_flag && !sat.zero_flag {
					sat.registers[REG_PC] = mem;
				}
				//println!("OPCODE_JGR_M");
			},

			OPCODE_JLS_M => {
				if !sat.sign_flag {
					sat.registers[REG_PC] = mem;
				} else {
					if 0 >= sat.registers[REG_SP] {
						println!("{}", ERR_CALL_OVERFLOW);
						return;
					}
					if mem as usize >= SIZE_APP_RAM {
						println!("{}", ERR_CALL_RANGE);
						return;
					}
					sat.registers[REG_SP] -= 1;
					sat.ram[sat.registers[REG_SP] as usize] = sat.registers[REG_PC];
					sat.registers[REG_PC] = mem;
				}
				//println!("OPCODE_JLS_M");
			},

			OPCODE_CALL_M => {
				if 0 >= sat.registers[REG_SP] {
					println!("{}", ERR_CALL_OVERFLOW);
					return;
				}
				if mem as usize >= SIZE_APP_RAM {
					println!("{}", ERR_CALL_RANGE);
					return ;
				}
				sat.registers[REG_SP] -= 1;
				sat.ram[sat.registers[REG_SP] as usize] = sat.registers[REG_PC];
				sat.registers[REG_PC] = mem;
				//println!("OPCODE_CALL_M");
			},

			OPCODE_RET => {
				if SIZE_APP_RAM <= sat.registers[REG_SP] as usize {
					return ;
				}
				sat.registers[REG_PC] = sat.ram[sat.registers[REG_SP] as usize];
				sat.registers[REG_SP] += 1;
				//println!("OPCODE_RET");
			},

			OPCODE_AND_RR => {
				sat.registers[dreg] = sat.registers[dreg] & sat.registers[sreg];
				let re = set_flags(sat.registers[dreg] as i8);
				sat.zero_flag = re.0;
				sat.sign_flag = re.1;
				//println!("OPCODE_AND_RR");
			},

			OPCODE_AND_RI => {
				sat.registers[dreg] = sat.registers[dreg] & imm as u8;
				let re = set_flags(sat.registers[dreg] as i8);
				sat.zero_flag = re.0;
				sat.sign_flag = re.1;
				//println!("OPCODE_AND_RI");
			},

			OPCODE_OR_RR => {
				sat.registers[dreg] = sat.registers[dreg] | sat.registers[sreg];
				let re = set_flags(sat.registers[dreg] as i8);
				sat.zero_flag = re.0;
				sat.sign_flag = re.1;
				//println!("OPCODE_OR_RR");
			},

			OPCODE_OR_RI => {
				sat.registers[dreg] = sat.registers[dreg] | imm as u8;
				let re = set_flags(sat.registers[dreg] as i8);
				sat.zero_flag = re.0;
				sat.sign_flag = re.1;
				//println!("OPCODE_OR_RI");
			},

			OPCODE_XOR_RR => {
				sat.registers[dreg] = sat.registers[dreg] ^ sat.registers[sreg];
				let re = set_flags(sat.registers[dreg] as i8);
				sat.zero_flag = re.0;
				sat.sign_flag = re.1;
				//println!("OPCODE_XOR_RR");
			},

			OPCODE_XOR_RI => {
				sat.registers[dreg] = sat.registers[dreg] ^ imm as u8;
				let re = set_flags(sat.registers[dreg] as i8);
				sat.zero_flag = re.0;
				sat.sign_flag = re.1;
				//println!("OPCODE_XOR_RI");
			},

			OPCODE_LOAD_RM => {
				//println!("OPCODE_LOAD_RM");
				if (mem as usize) < SIZE_APP_RAM {
					sat.registers[dreg] = sat.ram[mem as usize];
				} else if mem == IO_TERMINAL {
					if !sat.linefeed_buffered {
						match scr.mainWin.getch() {
							Some(pancurses::Input::Unknown(key)) => {
								if 0 < key {
									println!("ERROR : NCurses Key {}", key);
									sat.registers[dreg] = 0;
								} else {
									println!("ERROR : Unknown Key {}", key);
									sat.registers[dreg] = key as u8;
								}
							},
							Some(pancurses::Input::KeyEnter) => {
								sat.registers[dreg] = 0xD;
								sat.linefeed_buffered = true;
							},
							/*DEBUG_PAUSE_KEY => {
								sat.interrupt_flag = true;
							},*/
							_ => {},
						};
					} else {
						sat.registers[dreg] = 0xA;
						sat.linefeed_buffered = false;
					}
				} else {
					println!("{}", ERR_LOAD);
					return;
				}

			},

			OPCODE_LOADP_RR => {
				if (sat.registers[sreg] as usize) < SIZE_APP_RAM {
					sat.registers[dreg] = sat.ram[sat.registers[sreg] as usize];
				} else if sat.ram[sat.registers[sreg] as usize] == IO_TERMINAL {
					if sat.linefeed_buffered {
						sat.registers[dreg] = 0xA;
						sat.linefeed_buffered = false;
					} else {
						match scr.mainWin.getch() {
							Some(pancurses::Input::Unknown(key)) => {
								if 0 < key {
									println!("ERROR : NCurses Key {}", key);
									sat.registers[dreg] = 0;
								} else {
									println!("ERROR : Unknown Key {}", key);
									sat.registers[dreg] = key as u8;
								}
							},
							Some(pancurses::Input::KeyEnter) => {
								sat.registers[dreg] = 0xD;
								sat.linefeed_buffered = true;
							},
							/*DEBUG_PAUSE_KEY => {
								sat.interrupt_flag = true;
							},*/
							_ => {},
						};
					}
				} else {
					println!("{}", ERR_LOAD);
					return;
				}
				//println!("OPCODE_LOADP_RR");
			},

			OPCODE_STOR_MR => {
				if (mem as usize) < SIZE_APP_RAM {
					sat.ram[mem as usize] = sat.registers[dreg as usize];
				} else if mem == IO_TERMINAL {
					//let c: char = sat.registers[dreg as usize] as char;
					match sat.registers[dreg as usize] {
						0x9 => {
							scr.cur_X += TAB_SIZE as i32 - (scr.cur_X % TAB_SIZE as i32);
							if scr.cur_X >= TERMINAL_WIDTH {
								scr.cur_X = 0;
								scr.cur_Y = (scr.cur_Y + 1) % TERMINAL_HEIGHT;
							}
						},
						0xD => {
							scr.cur_X = 0;
						},
						0xA => {
							scr.cur_Y = (scr.cur_Y + 1) % TERMINAL_HEIGHT;
						},
						_ => {
							let re = unsafe { isprint(sat.registers[dreg as usize] as i32)} ;
							if 0 != re {
								scr.mainWin.printw(format!("{}", sat.registers[dreg as usize]).as_str());
								scr.terminal[scr.cur_X as usize][scr.cur_Y as usize] = sat.registers[dreg as usize] as char ;
								scr.cur_X += 1;
								if scr.cur_X >= TERMINAL_WIDTH {
									scr.cur_X = 0;
									scr.cur_Y = (scr.cur_Y + 1) % TERMINAL_HEIGHT;
								}
							} else {
								scr.mainWin.printw(format!("<0x{:01$X}>", sat.registers[dreg as usize], 2).as_str());
								scr.terminal[scr.cur_X as usize][scr.cur_Y as usize] = ' ';
							}
						},
					};
					scr.mainWin.mvprintw(scr.cur_Y, scr.cur_X, "");
					scr.mainWin.refresh();
				} else {
					println!("{}", ERR_STOR);
					return;
				}
				//println!("OPCODE_STOR_MR");
			},

			OPCODE_STORP_RR => {
				if (sat.registers[dreg as usize] as usize) < SIZE_APP_RAM {
					sat.ram[sat.registers[dreg as usize] as usize] = sat.registers[sreg as usize];
				} else if sat.registers[dreg as usize] == IO_TERMINAL {
					//let c: char = sat.registers[dreg] as char;
					match sat.registers[dreg as usize] {
						0x9 => {
							scr.cur_X += TAB_SIZE as i32 - (scr.cur_X % TAB_SIZE as i32);
							if scr.cur_X >= TERMINAL_WIDTH {
								scr.cur_X = 0;
								scr.cur_Y = (scr.cur_Y + 1) % TERMINAL_HEIGHT;
							}
						},
						0xD => {
							scr.cur_X = 0;
						},
						0xA => {
							scr.cur_Y = (scr.cur_Y + 1) % TERMINAL_HEIGHT;
						},
						_ => {
							let re = unsafe { isprint(sat.registers[dreg as usize] as i32)} ;
							if 0 != re {
								scr.mainWin.printw(format!("{}", sat.registers[dreg as usize]).as_str());
								scr.terminal[scr.cur_X as usize][scr.cur_Y as usize] = sat.registers[dreg as usize] as char ;
								scr.cur_X += 1;
								if scr.cur_X >= TERMINAL_WIDTH {
									scr.cur_X = 0;
									scr.cur_Y = (scr.cur_Y + 1) % TERMINAL_HEIGHT;
								}
							} else {
								scr.mainWin.printw(format!("<0x{0:01$X}>", sat.registers[dreg as usize], 2).as_str());
								scr.terminal[scr.cur_X as usize][scr.cur_Y as usize] = ' ';
							}
						},
					};
					scr.mainWin.mvprintw(scr.cur_Y, scr.cur_X, "");
					scr.mainWin.refresh();
				} else {
					println!("{}", ERR_STOR);
					return;
				}
				//println!("OPCODE_STORP_RR");
			},
			/*
			OPCODE_PUSH_R => {

			},

			OPCODE_POP_R => {

			},
			*/

			OPCODE_STACKER_R => {
				if 0 == imm {
					if 0 >= sat.registers[REG_SP]{
						println!("{}", ERR_PUSH);
						return;
					}
					sat.registers[REG_SP] -= 1;
					sat.ram[sat.registers[REG_SP] as usize] = sat.registers[dreg as usize];
				} else {
					if (sat.registers[REG_SP] as usize) >= SIZE_APP_RAM {
						println!("{}", ERR_POP);
						return;
					}
					sat.registers[dreg as usize] = sat.ram[sat.registers[REG_SP] as usize];
					sat.registers[REG_SP] += 1;
				}
				//println!("OPCODE_STACKER_R");
			},

			OPCODE_SWR_I => {
				sat.registers[REG_WIN] = imm as u8;
				//println!("OPCODE_SWR_I");
			},

			OPCODE_AWR_I => {
				sat.registers[REG_WIN] = get_grimm(sat.registers[REG_WIN], imm) as u8;
				let re = set_flags(sat.registers[REG_WIN] as i8);
				sat.zero_flag = re.0;
				sat.sign_flag = re.1;
				//println!("OPCODE_AWR_I");
			},

			OPCODE_AUX_I => {

				//println!("OPCODE_AUX_I");
			},

			_ => {
				println!("Should never happen");
			},
		};
	}
}

fn get_grwp(reg_win: u8, reg: u8) -> u8{
	((((reg_win - NUM_SYS_REG as u8) +
		(NUM_GEN_REG as u8 +
		((NUM_GEN_REG as u8 * (reg / NUM_GEN_REG as u8) + reg) % NUM_GEN_REG as u8))) %
		NUM_GEN_REG as u8) + NUM_SYS_REG as u8)
}

fn get_grimm(reg_win: u8, imm: i8) -> i8{
	let mut highLow: i8 = 0;
	if 0 > imm {
		highLow = (imm - (NUM_GEN_REG as i8 * (imm / NUM_GEN_REG as i8)));
	}
	else if 0 < imm {
		highLow = (NUM_GEN_REG as i8 * (imm / NUM_GEN_REG as i8) + imm) % NUM_GEN_REG as i8;
	}
	((((reg_win as i8 - NUM_SYS_REG as i8) + (NUM_GEN_REG as i8 + highLow)) % NUM_GEN_REG as i8) + NUM_SYS_REG as i8)
}


fn get_opcode(high_bits: u8) -> u8 {
	high_bits >> 3
}

fn get_dreg(high_bits: u8) -> u8 {
	high_bits & 0x07
}

fn get_sreg(low_bits: u8) -> u8 {
	low_bits & 0x07
}

fn get_mem(low_bits: u8) -> u8 {
	low_bits
}

fn get_imm(low_bits: i8) -> i8 {
	low_bits
}

fn set_flags(result: i8) -> (bool, bool){
	if 0 == result {
		(true, false)
	} else if 0 < result {
		(false, false)
	} else {
		(false, true)
		//sat.zero_flag = false;
		//sat.sign_flag = true;
	}
}
