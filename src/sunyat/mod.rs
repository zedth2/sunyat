extern crate libc;


use std::io::Read;
use std::fs::File;
use std::string::String;

use ncurses;

pub mod constants;

mod sat_scr;

struct SunyAT {
    //terminal: [[u8; constants::TERMINAL_HEIGHT]; TERMINAL_WIDTH+1];
    linefeed_buffered: bool,
    debug: bool,
    clock_ticks: usize,

    ram: [u8; constants::SIZE_APP_RAM],
    registers: [u8; constants::SIZE_REG],
}

impl Default for SunyAT {
    fn default() -> SunyAT {
        let mut newSat = SunyAT { linefeed_buffered: false, debug: false, clock_ticks: 0, ram: [0; constants::SIZE_APP_RAM], registers: [0; constants::SIZE_REG]};
        newSat.registers[0] = 0;
        newSat.registers[1] = 0;
        newSat.registers[2] = 0;
        newSat.registers[3] = constants::NUM_SYS_REG as u8;
        newSat.registers[4] = constants::SIZE_APP_RAM as u8;
        newSat
    }
}

pub fn start_sunyat(rom: &String, lState: bool, lDebug: bool) -> usize{
    //let clock_start = unsafe {libc::clock()}; //Use this to pause eventually
    let mut reVal = constants::EXIT_SUCCESS;
    let mut curSunyAT = SunyAT { ..Default::default() };
	let mut win: sat_scr::SatWin;
    if lState {
        reVal = load_state(&mut curSunyAT, rom);
    } else {
        reVal = load_rom(&mut curSunyAT, rom);
    }
    if constants::EXIT_SUCCESS != reVal {
        return reVal;
    }



	//if !lDebug {
		win = sat_scr::SatWin::new();
		if constants::EXT_ERR_NCURSES == win.setup_ncurses_terminal(){
			return constants::EXT_ERR_NCURSES;
		}
	//} else {

	//}



	sunyat_execute(&mut curSunyAT, &mut win, lDebug);


    return reVal;
}

fn load_rom(sunyat: &mut SunyAT, rom: &String) -> usize {
    let mut file_buffer: [u8; constants::SIZE_APP_RAM] = [0; constants::SIZE_APP_RAM];
    let mut app_msg: [u8; constants::SIZE_APP_MSG] = ['a' as u8; constants::SIZE_APP_MSG]; //This should be char so we can read the app message thats stored in the first bytes of the rom
    let mut inFile = match File::open(rom){
        Ok(file) => file,
        Err(err) => {
            println!("Error: {}", err);
            return constants::EXT_ERR_FILE_NOT_OPEN;
        },
    };
    let size = match inFile.read(&mut app_msg[..]){
		Ok(size) => size,
		Err(err) => {
			println!("ERROR : {}", err);
			return constants::EXT_ERR_FILE_READ;
		},
	};
    let size = match inFile.read(&mut file_buffer[..]){
		Ok(size) => size,
		Err(err) => {
			println!("ERROR : {}", err);
			return constants::EXT_ERR_FILE_READ;
		},
	};
    /*match inFile.read_to_end(&mut file_buffer){
        Ok(file) => file,
        Err(err) =>{
            println!("Error: {}", err);
            return constants::EXT_ERR_FILE_READ;
        },
    };*/
    if constants::SIZE_APP_RAM != file_buffer.len(){ //Should this be > ?
        println!("Error: {}", constants::ERR_BYTE_SIZE);
        return constants::EXT_ERR_ROM_BIG;
    } //Deleted the else that was in original C code.

    sunyat.ram = file_buffer ;



    return constants::EXIT_SUCCESS;
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
		let mut sreg: u8;
		let mut dreg: u8;
		let mut mem: u8;
		let mut imm: i8;
		let mut cmp_result: u8;

		let mut curHeight: i32 = scr.mainWin.get_max_y();
		let mut curWidth: i32 = scr.mainWin.get_max_x();

		if curWidth < constants::TERMINAL_WIDTH || curHeight < constants::TERMINAL_HEIGHT {
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

		if sat.registers[constants::REG_PC] > ((constants::SIZE_APP_RAM - 2) as u8) {
			println!("ERROR : {} {}", sat.registers[constants::REG_PC], sat.clock_ticks);
			println!("ERROR : {}", constants::ERR_INVALID_PC);
			return;
		}

		sat.registers[constants::REG_PC] += 1;
		sat.registers[constants::REG_IRH] = sat.ram[sat.registers[constants::REG_PC] as usize];
		sat.registers[constants::REG_PC] += 1;
		sat.registers[constants::REG_IRL] = sat.ram[sat.registers[constants::REG_PC] as usize];

		opcode = get_opcode(sat.registers[constants::REG_IRH]);

		sreg = get_grwp(sat.registers[constants::REG_WIN], get_sreg(sat.registers[constants::REG_IRL]));

		dreg = get_grwp(sat.registers[constants::REG_WIN], get_dreg(sat.registers[constants::REG_IRH]));

		imm = get_imm(sat.registers[constants::REG_IRL] as i8);
		mem = get_mem(sat.registers[constants::REG_IRL]);

		if lDebug {

		}
		println!("OPCODE : {}", opcode);
		match opcode {
			constants::OPCODE_MOV_RR => {
				println!("OPCODE_MOV_RR");
			},
			constants::OPCODE_MOV_RI => {
				println!("OPCODE_MOV_RI");
			},

			constants::OPCODE_ADD_RR => {
				println!("OPCODE_ADD_RR");
			},

			constants::OPCODE_ADD_RI => {
				println!("OPCODE_ADD_RI");
			},

			constants::OPCODE_SUB_RR => {
				println!("OPCODE_SUB_RR");
			},

			constants::OPCODE_MUL_RR => {
				println!("OPCODE_MUL_RR");
			},

			constants::OPCODE_MUL_RI => {
				println!("OPCODE_MUL_RI");
			},

			constants::OPCODE_DIV_RR => {
				println!("OPCODE_MUL_RI");
			},

			constants::OPCODE_DIV_RI => {
				println!("OPCODE_DIV_RI");
			},

			constants::OPCODE_CMP_RR => {
				println!("OPCODE_CMP_RR");
			},

			constants::OPCODE_CMP_RI => {
				println!("OPCODE_CMP_RI");
			},

			constants::OPCODE_JMP_M => {
				println!("OPCODE_JMP_M");
			},

			constants::OPCODE_JEQ_M => {
				println!("OPCODE_JEQ_M");
			},

			constants::OPCODE_JNE_M => {
				println!("OPCODE_JNE_M");
			},

			constants::OPCODE_JGR_M => {
				println!("OPCODE_JGR_M");
			},

			constants::OPCODE_JLS_M => {
				println!("OPCODE_JLS_M");
			},

			constants::OPCODE_CALL_M => {
				println!("OPCODE_CALL_M");
			},

			constants::OPCODE_RET => {
				println!("OPCODE_RET");
			},

			constants::OPCODE_AND_RR => {
				println!("OPCODE_AND_RR");
			},

			constants::OPCODE_AND_RI => {
				println!("OPCODE_AND_RI");
			},

			constants::OPCODE_OR_RR => {
				println!("OPCODE_OR_RR");
			},

			constants::OPCODE_OR_RI => {
				println!("OPCODE_OR_RI");
			},

			constants::OPCODE_XOR_RR => {
				println!("OPCODE_XOR_RR");
			},

			constants::OPCODE_XOR_RI => {
				println!("OPCODE_XOR_RI");
			},

			constants::OPCODE_LOAD_RM => {
				println!("OPCODE_LOAD_RM");
			},

			constants::OPCODE_LOADP_RR => {
				println!("OPCODE_JEQ_M");
			},

			constants::OPCODE_STOR_MR => {
				println!("OPCODE_STOR_MR");
			},

			constants::OPCODE_STORP_RR => {
				println!("OPCODE_STORP_RR");
			},
			/*
			constants::OPCODE_PUSH_R => {

			},

			constants::OPCODE_POP_R => {

			},
			*/

			constants::OPCODE_STACKER_R => {
				println!("OPCODE_STACKER_R");
			},

			constants::OPCODE_SWR_I => {
				println!("OPCODE_SWR_I");
			},

			constants::OPCODE_AWR_I => {
				println!("OPCODE_AWR_I");
			},

			constants::OPCODE_AUX_I => {
				println!("OPCODE_AUX_I");
			},

			_ => {
				println!("Should never happen");
			},
		};
	}
}

fn get_grwp(regWin: u8, reg: u8) -> u8{
	((((regWin - constants::NUM_SYS_REG as u8) +
		(constants::NUM_GEN_REG as u8 +
		((constants::NUM_GEN_REG as u8 * (reg / constants::NUM_GEN_REG as u8) + reg) % constants::NUM_GEN_REG as u8))) %
		constants::NUM_GEN_REG as u8) + constants::NUM_SYS_REG as u8)
}

fn get_grimm(regWin: u8, imm: i8) -> i8{
	let mut highLow: i8 = 0;
	if 0 > imm {
		highLow = (imm - (constants::NUM_GEN_REG as i8 * (imm / constants::NUM_GEN_REG as i8)));
	}
	else if 0 < imm {
		highLow = (constants::NUM_GEN_REG as i8 * (imm / constants::NUM_GEN_REG as i8) + imm) % constants::NUM_GEN_REG as i8;
	}
	((((regWin as i8 - constants::NUM_SYS_REG as i8) + (constants::NUM_GEN_REG as i8 + highLow)) % constants::NUM_GEN_REG as i8) + constants::NUM_SYS_REG as i8)
}


fn get_opcode(highBits: u8) -> u8 {
	highBits >> 3
}

fn get_dreg(highBits: u8) -> u8 {
	highBits & 0x07
}

fn get_sreg(lowBits: u8) -> u8 {
	lowBits & 0x07
}

fn get_mem(lowBits: u8) -> u8 {
	lowBits
}

fn get_imm(lowBits: i8) -> i8 {
	lowBits
}
