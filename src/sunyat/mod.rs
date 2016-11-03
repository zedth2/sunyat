extern crate libc;
extern crate ncurses;

mod constants;

struct SunyAT {
    //terminal: [[u8; constants::TERMINAL_HEIGHT]; TERMINAL_WIDTH+1];
    linefeed_buffered: bool,
    debug: bool,
    clock_ticks: usize,

    ram: [u8; constants::SIZE_APP_RAM],
    registers: [u8; constants::SIZE_REG],

}

pub fn start_sunyat(rom: &str, lState: bool, lDebug: bool){
    //let clock_start = unsafe {libc::clock()}; //Use this to pause eventually
    let reVal = constants::EXIT_SUCCESS;

    //if(lState){
        //if(
    //}
}

fn load_state(rom: &str) -> u16
{
return 0;
}

pub fn shit(){
    start_sunyat("", false, false);
    println!("SHIT DICKS");
}
