extern crate libc;


use std::io::Read;
use std::fs::File;

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

    if(lState){
        reVal = load_rom(&mut curSunyAT, rom);
    } else {
        reVal = load_state(&mut curSunyAT, rom);
    }
    if(constants::EXIT_SUCCESS != reVal){
        return reVal;
    }


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
    if(constants::SIZE_APP_ROM != file_buffer.len()){ //Should this be > ?
        println!("Error: {}", constants::ERR_BYTE_SIZE);
        return constants::EXT_ERR_ROM_BIG;
    } //Deleted the else that was in original C code.

    return constants::EXIT_SUCCESS;
}

fn load_state(sunyat: &mut SunyAT, rom: &str) -> usize
{
return 255;
}

pub fn shit(){
    start_sunyat("", false, false);
    println!("SHIT DICKS");
}
