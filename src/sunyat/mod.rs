extern crate libc;


use std::io::Read;
use std::fs::File;

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
        SunyAT { linefeed_buffered: false, debug: false, clock_ticks: 0, ram: [0; constants::SIZE_APP_RAM], registers: [0; constants::SIZE_REG]}
    }
}

pub fn start_sunyat(rom: &str, lState: bool, lDebug: bool) -> usize{
    //let clock_start = unsafe {libc::clock()}; //Use this to pause eventually
    let mut reVal = constants::EXIT_SUCCESS;
    let mut curSunyAT = SunyAT { ..Default::default() };
	let mut win: sat_scr::SatWin;
    if lState {
        reVal = load_rom(&mut curSunyAT, rom);
    } else {
        reVal = load_state(&mut curSunyAT, rom);
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



	sunyat_execute(&mut curSunyAT, &mut win);


    return reVal;
}

fn load_rom(sunyat: &mut SunyAT, rom: &str) -> usize {
    let mut file_buffer: Vec<u8> = Vec::new();
    let mut inFile = match File::open(rom){
        Ok(file) => file,
        Err(err) => {
            println!("Error: {}", err);
            return constants::EXT_ERR_FILE_NOT_OPEN;
        },
    };
    match inFile.read_to_end(&mut file_buffer){
        Ok(file) => file,
        Err(err) =>{
            println!("Error: {}", err);
            return constants::EXT_ERR_FILE_READ;
        },
    };
    if constants::SIZE_APP_ROM != file_buffer.len(){ //Should this be > ?
        println!("Error: {}", constants::ERR_BYTE_SIZE);
        return constants::EXT_ERR_ROM_BIG;
    } //Deleted the else that was in original C code.

    return constants::EXIT_SUCCESS;
}

fn load_state(sunyat: &mut SunyAT, rom: &str) -> usize
{
return 255;
}

fn get_grwp(regWin: u8, reg: u8){
	((((regWin - constants::NUM_SYS_REG as u8) +
		(constants::NUM_GEN_REG as u8 +
		((constants::NUM_GEN_REG as u8 * (reg / constants::NUM_GEN_REG as u8) + reg) % constants::NUM_GEN_REG as u8))) %
		constants::NUM_GEN_REG as u8) + constants::NUM_SYS_REG as u8)
}

fn get_grimm(regWin: u8, imm: i8){
	let mut highLow: i8 = 0;
	if 0 > imm {
		highLow = (imm - (constants::NUM_GEN_REG as i8 * (imm / constants::NUM_GEN_REG as i8)));
	}
	else if 0 < imm {
		highLow = (constants::NUM_GEN_REG as i8 * (imm / constants::NUM_GEN_REG as i8) + imm) % constants::NUM_GEN_REG as i8;
	}
	return ((((regWin as i8 - constants::NUM_SYS_REG as i8) +
		(constants::NUM_GEN_REG as i8 +
		highLow)) %
		constants::NUM_GEN_REG as i8) + constants::NUM_SYS_REG as i8);
}


//macro_rules! GET_GRWP {
	//($regWin:expr, $imm:expr) => {
		//(((($regWin - constants::NUM_SYS_REG) + (constants::NUM_GEN_REG + HIGH_OR_LOW!($imm))) % constants::NUM_GEN_REG) + constants::NUM_SYS_REG)
	//};
//}

//macro_rules! HIGH_OR_LOW {
	//($imm:expr) => {
		//(((0 > $imm) as usize * ($imm - (constants::NUM_GEN_REG * ($imm / constants::NUM_GEN_REG)))) + ((0 < $imm) * (constants::NUM_GEN_REG * ($imm / constants::NUM_GEN_REG) + $imm) % constants::NUM_GEN_REG))
	//};
//}


fn sunyat_execute(sat: &mut SunyAT, scr: &mut sat_scr::SatWin){
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
			println!("ERROR : {}", constants::ERR_INVALID_PC);
			return;
		}

		sat.registers[constants::REG_PC] += 1;
		sat.registers[constants::REG_IRH] = sat.ram[sat.registers[constants::REG_PC] as usize];
		sat.registers[constants::REG_PC] += 1;
		sat.registers[constants::REG_IRL] = sat.ram[sat.registers[constants::REG_PC] as usize];

		opcode = get_opcode(sat.registers[constants::REG_IRH]);

		//sreg = GET_GRWP!(sat.registers[constants::REG_WIN], get_sreg(sat.registers[constants::REG_IRL]));

		//dreg = GET_GRWP!(sat.registers[constants::REG_WIN], get_dreg(sat.registers[constants::REG_IRH]));

		imm = get_imm(sat.registers[constants::REG_IRL] as i8);
		mem = get_mem(sat.registers[constants::REG_IRL]);

	}


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
