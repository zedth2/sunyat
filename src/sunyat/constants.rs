//mod constants;
/*
 * On read:  0 if no key, key code otherwise
 * On write: character output w/ cursor adjust
 */
const IO_TERMINAL: i32 = 0xFF;

pub const EXIT_SUCCESS: usize = 0;

const SIZE_APP_MSG: usize = 70; //Probably pointless

//Apparently in Rust array sizes have to be of type usize
/**
 * Brief:
 *      This will be the size of the ram.
 */
pub const SIZE_APP_RAM: usize = 0xFF;

/**
 *  Brief:
 *      This will be the max size allowed for any load rom file.
 */
pub const SIZE_APP_ROM: usize = SIZE_APP_RAM + SIZE_APP_MSG;   //APP_ROM_SIZE (APP_MSG_SIZE + APP_RAM_SIZE)


//System Registers
/**
 *  Brief:
 *      This register will contain the current program counter.
 */
pub const REG_PC: usize = 0;

/**
 *  Brief:
 *      This is the register containing the 8 high bits of the currently
 *          loaded instruction.
 */
pub const REG_IRH: usize = 1;

/**
 * Brief:
 *      This is the register containing the 8 low bits of the currently
 *          loaded instruction.
 */
pub const REG_IRL: usize = 2;

/**
 *  Brief:
 *      This register will contain the current starting position of the
 *          register window.
 */
pub const REG_WIN: usize = 3;

/**
 *  Brief:
 *      FILL IN
 */
const REG_SP: i32 = 4;

/**
 *  Brief:
 *      This is the number of system registers.
 *          It will be used to figure out where the general purpose registers
 *          start.
 */
pub const NUM_SYS_REG: usize = 5; //Number of system registers (0-4) used for referencing GPRs starting at 0.


//General Purpose Registars

/**
 *  Brief:
 *      This is the number of general purpose registers.
 */
pub const NUM_GEN_REG: usize = 32;

/**
 *  Brief:
 *      This is the total number of registers. It should be the total number
 *          of system registers plus the number of general purpose registers.
 */
pub const SIZE_REG: usize = (NUM_SYS_REG + NUM_GEN_REG);

/**
 *  Brief:
 *      This is the starting point of the general purpose registers.
 */
const REG_GEN_START: usize = NUM_SYS_REG;

/**
 *  Brief:
 *      This is the ending index of the general purpose registers.
 */
const REG_GEN_END: usize = SIZE_REG - 1;

/**
 *  Brief:
 *      This is the size of the register window.
 */
const SIZE_WIN: usize = 8;

/**
 *  Brief:
 *      This is the greatest index that the start of the register window
 *          can be.
 */
const MAX_WIN_INDEX: usize = SIZE_REG - SIZE_WIN;





//opcodes

/*OPCODE 0: MOV (Register to Register)-----------------------------------
 *  Use:	MOV Reg_A Reg_B
 *  Brief:	Loads the immediate value into reg_A
 *
 *	Affected Flags: None
 */
const OPCODE_MOV_RR: i32 = 0;
//-----------------------------------------------------------------------

/*OPCODE 1: MOV (Immediate To Register)----------------------------------
 *  Use:	MOV Reg_A Reg_B
 *  Brief:	Copies the value in reg_B into reg_A
 *
 *	Affected Flags: None
 */
const OPCODE_MOV_RI: i32 = 1;
//-----------------------------------------------------------------------

/*OPCODE 2: ADD (Register to Register)-----------------------------------
 *  Use:	ADD Reg_A Reg_B
 *  Brief:	Adds reg_B to reg_A, storing the result in reg_A
 *
 *	Affected Flags: Zero and Sign
 */
const OPCODE_ADD_RR: i32 = 2;
//-----------------------------------------------------------------------

/*OPCODE 3: MOV (Immediate To Register)----------------------------------
 *  Use:	ADD Reg_A Imm
 *  Brief:	Adds immediate to reg_A, storing the result in reg_A
 *
 *	Affected Flags: Zero and Sign
 */
const OPCODE_ADD_RI: i32 = 3;
//-----------------------------------------------------------------------

/*OPCODE 4: SUB (Register to Register)-----------------------------------
 *  Use:	SUB Reg_A Reg_B
 *  Brief:	Subtracts reg_B from reg_A, storing the result in reg_A
 *
 *	Affected Flags: Zero and Sign
 */
const OPCODE_SUB_RR: i32 = 4;
//-----------------------------------------------------------------------

/*OPCODE 5: MUL (Register to Register)-----------------------------------
 *  Use:	MUL Reg_A Reg_B
 *  Brief:	Multiply reg_B and reg_A, storing the result in reg_A
 *
 *	Affected Flags: Zero and Sign
 */
const OPCODE_MUL_RR: i32 = 5;
//-----------------------------------------------------------------------

/*OPCODE 6: MUL (Immediate to Register)----------------------------------
 *  Use:	MUL Reg_A Imm
 *  Brief:	Multiply reg_B and Imm, storing the result in reg_A
 *
 *	Affected Flags: Zero and Sign
 */
const OPCODE_MUL_RI: i32 = 6;
//-----------------------------------------------------------------------

/*OPCODE 7: DIV (Register to Register)-----------------------------------
 *  Use:	DIV Reg_A Reg_B
 *  Brief:	Divides Reg_A by Reg_B, storing the result in reg_A
 *
 *	Affected Flags: Zero and Sign
 */
const OPCODE_DIV_RR: i32 = 7;
//-----------------------------------------------------------------------

/*OPCODE 8: DIV (Immediate to Register)----------------------------------
 *  Use:	DIV Reg_A Imm
 *  Brief:	Divides Reg_A by Imm, storing the result in reg_A
 *
 *	Affected Flags: Zero and Sign
 */
const OPCODE_DIV_RI: i32 = 8;
//-----------------------------------------------------------------------

/*OPCODE 9: CMP (Register to Register)-----------------------------------
 *  Use:	CMP Reg_A Reg_B
 *  Brief:	Compares the two register values via subtraction but does not
 *			store the result. However, the flags are set based on the result
 *			of the subtraction.
 *
 *	Affected Flags: Zero and Sign
 */
const OPCODE_CMP_RR: i32 = 9;
//-----------------------------------------------------------------------

/*OPCODE 10: CMP (Immediate to Register)---------------------------------
 *  Use:	CMP Reg_A Imm
 *  Brief:	Compares the register value and immediate via subtraction
 *			but does not store the result. However, the flags are set based
 *			on the result of the subtraction.
 *
 *	Affected Flags: Zero and Sign
 */
const OPCODE_CMP_RI: i32 = 10;
//-----------------------------------------------------------------------

/*OPCODE 11: JMP --------------------------------------------------------
 *  Use:	JMP !address
 *  Brief:	Jump (branch) unconditionally to the code beginning at
 *			address. Sets the PC to address. The address will typically be
 *			provided as a label, but can be written as an immediate, as
 *			well.
 *
 *	Affected Flags: None
 */
const OPCODE_JMP_M: i32 = 11;
//-----------------------------------------------------------------------

/*OPCODE 12: JEQ --------------------------------------------------------
 *  Use:	JEQ !address
 *  Brief:	Jump (branch) to the code beginning at address if the previous
 *			CMP found an equality or if an ALU instruction's result was
 *			zero... in either case the Zero flag would be high. Sets the PC
 *			to address. The address will typically be provided as a label,
 *			but can be written as an immediate, as well.
 *
 *	Affected Flags: None
 */
const OPCODE_JEQ_M: i32 = 12;
//-----------------------------------------------------------------------

/*OPCODE 13: JNE --------------------------------------------------------
 *  Use:	JNE !address
 *  Brief:	Jump (branch) to the code beginning at address if the previous
 *			CMP found an inequality or if an ALU instruction's result was
 *			not zero... in either case the Zero flag would be low. Sets the
 *			PC to address. The address will typically be provided as a
 *			label, but can be written as an immediate, as well.
 *
 *	Affected Flags: None
 */
const OPCODE_JNE_M: i32 = 13;
//-----------------------------------------------------------------------

/*OPCODE 14: JGR --------------------------------------------------------
 *  Use:	JGR !address
 *  Brief:	Jump (branch) to the code beginning at address if the previous
 *			CMP found the left operand to be greater than the right or if an
 *			ALU instruction's result was positive but not zero... in either
 *			case the Zero flag would be low and the Sign flag low. Sets the
 *			PC to address. The address will typically be provided as a
 *			label, but can be written as an immediate, as well.
 *
 *	Affected Flags: None
 */
const OPCODE_JGR_M: i32 = 14;
//-----------------------------------------------------------------------

/*OPCODE 15: JGR --------------------------------------------------------
 *  Use:	JLS !address
 *  Brief:	Jump (branch) to the code beginning at address if the previous
 *			CMP found the left operand to be less than the right or if an
 *			ALU instruction's result was negative... in either case the Sign
 *			flag would be high. Sets the PC to address. The address will
 *			typically be provided as a label, but can be written as an
 *			immediate, as well.
 *
 *	Affected Flags: None
 */
const OPCODE_JLS_M: i32 = 15;
//-----------------------------------------------------------------------

/*OPCODE 16: CALL -------------------------------------------------------
 *  Use:	CALL !address
 *  Brief:	Call function beginning at address. This pushes the address
 *			after the CALLing line of code to the system stack, and then
 *			sets the PC to address. The address will typically be provided
 *			as a label, but can be written as an immediate, as well.
 *
 *	Affected Flags: None
 */
const OPCODE_CALL_M: i32 = 16;
//-----------------------------------------------------------------------

/*OPCODE 17: RET --------------------------------------------------------
 *  Use:	RET
 *  Brief:	Returns from a function call. This pops the top of the system
 *			stack into the PC... presuming this was the address pushed to
 *			the stack by a previous CALL. RETurning when the stack is
 *			empty is the signal to halt the VM and print the total number
 *			of clock cycles executed by the application.
 *
 *	Affected Flags: None
 */
const OPCODE_RET: i32 = 17;
//-----------------------------------------------------------------------

/*OPCODE 18: AND (Register to Register) ---------------------------------
 *  Use:	AND Reg_A Reg_B
 *  Brief:	Perform a bitwise AND on reg_A and reg_B, storing the result in
 *			reg_A
 *
 *	Affected Flags: Zero and Sign
 */
const OPCODE_AND_RR: i32 = 18;
//-----------------------------------------------------------------------

/*OPCODE 19: AND (Immediate to Register) --------------------------------
 *  Use:	AND Reg_A Imm
 *  Brief:	Perform a bitwise AND on reg_A and Imm, storing the result in
 *			reg_A
 *
 *	Affected Flags: Zero and Sign
 */
const OPCODE_AND_RI: i32 = 19;
//-----------------------------------------------------------------------

/*OPCODE 20: OR (Register to Register) ----------------------------------
 *  Use:	OR Reg_A Reg_B
 *  Brief:	Perform a bitwise OR on reg_A and reg_B, storing the result in
 *			reg_A
 *
 *	Affected Flags: Zero and Sign
 */
const OPCODE_OR_RR: i32 = 20;
//-----------------------------------------------------------------------

/*OPCODE 21: OR (Immediate to Register) ---------------------------------
 *  Use:	OR Reg_A Imm
 *  Brief:	Perform a bitwise OR on reg_A and Imm, storing the result in
 *			reg_A
 *
 *	Affected Flags: Zero and Sign
 */
const OPCODE_OR_RI: i32 = 21;
//-----------------------------------------------------------------------

/*OPCODE 22: XOR (Register to Register) ---------------------------------
 *  Use:	XOR Reg_A Reg_B
 *  Brief:	Perform a bitwise XOR on reg_A and Imm, storing the result in
 *			reg_A
 *
 *	Affected Flags: Zero and Sign
 */
const OPCODE_XOR_RR: i32 = 22;
//-----------------------------------------------------------------------

/*OPCODE 23: XOR (Immediate to Register) --------------------------------
 *  Use:	XOR Reg_A Imm
 *  Brief:	Perform a bitwise XOR on reg_A and Imm, storing the result in
 *			reg_A
 *
 *	Affected Flags: Zero and Sign
 */
const OPCODE_XOR_RI: i32 = 23;
//-----------------------------------------------------------------------

/*OPCODE 24: LOAD -------------------------------------------------------
 *  Use:	LOAD Reg_A !Address
 *  Brief:	Loads (copies) a value from the given memory address into
 *			reg_A.
 *
 *	Affected Flags: None
 */
const OPCODE_LOAD_RM: i32 = 24;
//-----------------------------------------------------------------------

/*OPCODE 25: LOADP ------------------------------------------------------
 *  Use:	LOADP Reg_A Reg_B
 *  Brief:	Loads (copies) a value from the memory address in reg_B into
 *	a reg_A.
 *
 *	Affected Flags: None
 */
const OPCODE_LOADP_RR: i32 = 25;
//-----------------------------------------------------------------------

/*OPCODE 26: STOR -------------------------------------------------------
 *  Use:	STOR !Address Reg_A
 *  Brief:	Stores (copies) the value from reg_A to the given memory
 *			address.
 *
 *	Affected Flags: None
 */
const OPCODE_STOR_MR: i32 = 26;
//-----------------------------------------------------------------------

/*OPCODE 27: STORP ------------------------------------------------------
 *  Use:	STORP Reg_A Reg_B
 *  Brief:	Stores (copies) a value from reg_B into the memory address in
 *			reg_A.
 *
 *	Affected Flags: None
 */
const OPCODE_STORP_RR: i32 = 27;
//-----------------------------------------------------------------------

/*OPCODE 28: PUSH -------------------------------------------------------
 *  Use:	PUSH Reg_A
 *  Brief:	Pushes (copies) the value in reg_A to the top of the system
 *			stack. This is accomplished by first decrementing SP and then
 *			storing the at the new address in SP.
 *
 *	Affected Flags: None
 */
//const OPCODE_PUSH_R: i32 = 28;
//-----------------------------------------------------------------------

/*OPCODE 29: POP --------------------------------------------------------
 *  Use:	POP Reg_A
 *  Brief:	Pops (copies) the value at the top of the system stack into
 *			reg_A. This is accomplished by first copying the value at the
 *			address in SP and then incrementing SP.
 *
 *	Affected Flags: None
 */
//const OPCODE_POP_R: i32 = 29;
//-----------------------------------------------------------------------


/**OPCODE 28
 *  USE:    NONE
 *  Brief:  This replaces push and pop, if the low bits are zero then
 *          it's a push, if not pop.
 *
 *	Affected Flags: ???
 */
const OPCODE_STACKER_R: i32 = 28;



/*OPCODE 29: SWR --------------------------------------------------------
 *  Use:	SWR Imm
 *  Brief:	Copies value of Imm into the "Window" System Register. For Register Windowing.
 *
 *	Affected Flags: None
 */
const OPCODE_SWR_I: i32 = 29;
//-----------------------------------------------------------------------

/*OPCODE 30: AWR --------------------------------------------------------
 *  Use:	AWR Imm
 *  Brief:	Increments the "Window" System Register by Imm. For Register Windowing.
 *
 *	Affected Flags: None
 */
const OPCODE_AWR_I: i32 = 30;
//-----------------------------------------------------------------------

/*OPCODE 31: AUX --------------------------------------------------------
 *  Use:	AUX Imm
 *
 *	0: Save State
 *	1: Pause
 *	2:
 *	3
 *	4
 *	5
 *	6
 *	7
 *
 *	Affected Flags: None
 */
const OPCODE_AUX_I: i32 = 31;
//-----------------------------------------------------------------------


//Exit Code Errors

/**
 *  Brief:
 *      This will get thrown if the system wasn't given any rom file, or
 *          such, to load in.
 */
pub const EXT_ERR_NO_FILE_ARG: usize = 1;

/**
 *  Brief:
 *      This will get thrown if the ROM provided is to large.
 */
pub const EXT_ERR_ROM_BIG: usize = 2;

/**
 *  Brief: //Look at sunyat.c:213
 *      This will get thrown if after reading t
 */
pub const EXT_ERR_BYTE_SIZE: usize = 3;

/**
 *  Brief:
 *      The provided file could not be open.
 */
pub const EXT_ERR_FILE_NOT_OPEN: usize = 4;

pub const EXT_ERR_FILE_READ: usize = 5;

/**
 *  Brief:
 *      Will be thrown when theres an error in ncurses.
 */
pub const EXT_ERR_NCURSES: usize = 6; //This should be expand to cover all ncurses errors.

/**
 *	Brief:
 *		Defines size of a savestate file.
 *
 */
const SIZE_APP_SAVESTATE: usize = (SIZE_APP_RAM + SIZE_REG);



/**
 *  Brief:
 *      This is the save state command line switch.
 */
const SAVE_STATE_SWITCH: &'static str = "-s\0";

/**
 *  Brief:
 *      This is the debugger switch.
 */
const DEBUGGER_SWITCH: &'static str = "-d\0";


pub const ERR_BYTE_SIZE: &'static str = "\tApplication is not the correct byte size.\n";

pub const ERR_INVALID_PC: &'static str = "\tProgram counter is invalid\n";

pub const TERMINAL_WIDTH: i32 = 80;

pub const TERMINAL_HEIGHT: i32 = 24;

pub const TAB_SIZE: usize = 4;
